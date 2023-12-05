advent_of_code::solution!(3);

use std::collections::HashMap;
use std::collections::HashSet;
use std::str::Lines;

#[derive(Eq, Hash, PartialEq)]
struct Location {
    x: usize,
    y: usize,
}

// Function that returns all neighboring points
fn neighbors(location: Location) -> Vec<Location> {
    let mut neighbors: Vec<Location> = Vec::new();

    for x in location.x - 1..=location.x + 1 {
        for y in location.y - 1..=location.y + 1 {
            if x == location.x && y == location.y {
                continue;
            }

            neighbors.push(Location { x, y });
        }
    }

    neighbors
}

pub fn part_one(contents: &str) -> Option<u32> {
    // variables to keep track of state
    let mut sum = 0;
    let mut locations: Vec<Location> = Vec::new();
    let mut map: HashMap<Location, u32> = HashMap::new();

    // loop line by line
    for (y, line) in contents.lines().enumerate() {
        let mut start_x = 0;
        while start_x < line.len() {
            // get char
            let mut c = line.chars().nth(start_x).unwrap_or('.');

            // try to parse
            let mut end_x = start_x;
            let mut num = String::new();
            while c.is_numeric() {
                num += &c.to_string();

                end_x += 1;
                c = line.chars().nth(end_x).unwrap_or('.');
            }

            // get that number block
            if start_x != end_x {
                let parsed: u32 = num.parse().unwrap();

                // add to map
                for x in start_x..end_x {
                    map.insert(Location { x, y }, parsed);
                }

                // jump
                start_x = end_x;
                continue;
            }

            // otherwise, see if is special character
            if c != '.' {
                locations.push(Location { x: start_x, y });
            }

            // jump
            start_x = end_x + 1;
        }
    }

    // now that we've parsed file, find all matches
    let mut used_numbers: HashSet<u32> = HashSet::new();
    for location in locations.into_iter() {
        used_numbers.clear();

        for neighbor in neighbors(location).into_iter() {
            if let Some(value) = map.get(&neighbor) {
                if used_numbers.contains(&value) {
                    continue;
                }

                sum += value;
                used_numbers.insert(*value);
            }
        }
    }

    // print
    Some(sum)
}

/////// DAY 2 solution /////////
// took a different approach //

fn check_around(x: usize, start_y: usize, lines: &Lines) -> Option<Location> {
    let range = (if start_y > 0 { start_y - 1 } else { start_y })..start_y + 2;

    for y in range {
        let c = lines
            .clone()
            .nth(y)
            .unwrap_or("")
            .chars()
            .nth(x)
            .unwrap_or('.');

        if c == '*' {
            return Some(Location { x, y });
        }
    }

    None
}

pub fn part_two(contents: &str) -> Option<u32> {
    let lines = contents.lines();
    let lines_cpy = lines.clone();

    // variables to keep track of state
    let mut sum = 0;
    let mut gears: HashMap<Location, u32> = HashMap::new();

    // loop line by line
    for (y, line) in contents.lines().enumerate() {
        let mut start_x = 0;
        while start_x < line.len() {
            // get char
            let mut c = line.chars().nth(start_x).unwrap_or('.');

            // increment if not starter
            if c == '.' {
                start_x += 1;
                continue;
            }

            // try to parse
            let mut end_x = start_x;
            let mut num = String::new();

            // start checking around to see if this is an important character
            let mut gear_location: Option<Location> = None;

            if start_x > 0 {
                gear_location = check_around(start_x - 1, y, &lines_cpy);
            }

            loop {
                c = line.chars().nth(end_x).unwrap_or('.');

                // check if is numeric
                if !c.is_numeric() {
                    break;
                }

                // append
                num.push(c);

                // check around for gear if not found already
                if gear_location.is_none() {
                    gear_location = check_around(end_x, y, &lines_cpy);
                }

                // increment
                end_x += 1;
            }

            // check end
            if gear_location.is_none() {
                gear_location = check_around(end_x, y, &lines_cpy);
            }

            // only care about the number if there is a gear
            if let Some(gear) = gear_location {
                // get number
                let parsed: u32 = num.parse().unwrap_or(u32::MAX);

                if parsed == u32::MAX {
                    start_x = end_x + 1;
                    continue;
                }

                // check if we have the other gear ID for this location
                if let Some(other) = gears.remove(&gear) {
                    // multiply the two and add
                    sum += parsed * other;
                } else {
                    // add to map
                    gears.insert(gear, parsed);
                }
            }

            // jump
            start_x = end_x + 1;
        }
    }

    // print sum
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
