use std::collections::VecDeque;

advent_of_code::solution!(4);

#[derive(Clone)]
struct ParsedCard {
    pub index: usize,
    pub winning: Vec<u32>,
    pub numbers: Vec<u32>,
}

fn parse_numbers(line: &str) -> Vec<u32> {
    return line
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().parse().unwrap())
        .collect();
}

// Parses the input into a vector of ParsedCards
fn parse_input(input: &str) -> Vec<ParsedCard> {
    let mut cards: Vec<ParsedCard> = Vec::new();

    // go line by line
    for (id, line) in input.lines().enumerate() {
        // split
        let (_card, data) = line.split_once(": ").unwrap();
        let (winning, numbers) = data.split_once(" | ").unwrap();

        // process into numbers
        cards.push(ParsedCard {
            index: id + 1,
            winning: parse_numbers(winning),
            numbers: parse_numbers(numbers),
        });
    }

    cards
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total: u32 = 0;

    for card in parse_input(&input) {
        let mut local_total: u32 = 0;
        for number in card.numbers {
            if card.winning.contains(&number) {
                local_total = if local_total == 0 { 1 } else { local_total * 2 };
            }
        }

        total += local_total;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total: u32 = 0;
    let cards = parse_input(input);
    let mut dequeue = VecDeque::from(cards.clone());

    while let Some(card) = dequeue.pop_front() {
        total += 1;
        let mut local_total: usize = 0;

        for number in card.numbers {
            if !card.winning.contains(&number) {
                continue;
            }

            local_total += 1;
        }

        for i in card.index..(card.index + local_total) {
            if let Some(next_card) = cards.get(i as usize) {
                dequeue.push_back(next_card.clone());
            }
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
