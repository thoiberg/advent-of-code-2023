use regex::Regex;

fn main() {
    println!("Hello, world!");
    let input = read_input();
    let cards = process_input(&input);

    let part_one_answer = part_one_solution(&cards);
    println!("Part One answer is {part_one_answer}");
}

fn part_one_solution(cards: &[Card]) -> u32 {
    cards
        .iter()
        .fold(0, |acc, card| acc + calculate_card_points(card))
}

fn calculate_card_points(card: &Card) -> u32 {
    let matched_numbers = card.matching_numbers();
    // TODO: There has to be a better way to do this with just binary
    if matched_numbers.is_empty() {
        0
    } else if matched_numbers.len() == 1 {
        1
    } else {
        let point_value = 1;
        point_value << (matched_numbers.len() - 1)
    }
}

struct Card {
    number: u32,
    winning_numbers: Vec<u32>,
    scratched_numbers: Vec<u32>,
}

impl Card {
    fn matching_numbers(&self) -> Vec<&u32> {
        self.winning_numbers
            .iter()
            .filter(|num| self.scratched_numbers.contains(num))
            .collect()
    }
}

fn read_input() -> String {
    String::from(include_str!("../data/puzzle_input.txt"))
}

fn process_input(input: &str) -> Vec<Card> {
    let re = Regex::new(r"^Card\s+(\d{1,3}): (.+) \| (.+)$").unwrap();

    input
        .lines()
        .map(|line| {
            let captures = re.captures(line).unwrap();
            let card_number = &captures[1].parse::<u32>().unwrap();
            let winning_numbers_str = &captures[2];
            let winning_numbers: Vec<u32> = winning_numbers_str
                .split(' ')
                .filter_map(|c| c.parse::<u32>().ok())
                .collect();
            let scratched_numbers_str = &captures[3];
            let scratched_numbers: Vec<u32> = scratched_numbers_str
                .split(' ')
                .filter_map(|c| c.parse::<u32>().ok())
                .collect();

            Card {
                number: *card_number,
                winning_numbers,
                scratched_numbers,
            }
        })
        .collect()
}

#[cfg(test)]
mod test_super {
    use super::*;

    fn test_data() -> Vec<Card> {
        let test_data = String::from(include_str!("../data/test_input.txt"));
        process_input(&test_data)
    }

    #[test]
    fn test_process_input() {
        let cards = test_data();

        assert_eq!(cards[0].number, 1);
        assert_eq!(cards[0].winning_numbers, vec![41, 48, 83, 86, 17]);
        assert_eq!(
            cards[0].scratched_numbers,
            vec![83, 86, 6, 31, 17, 9, 48, 53]
        );
    }

    #[test]
    fn test_calculate_card_points() {
        let cards = test_data();

        assert_eq!(calculate_card_points(&cards[0]), 8);
        assert_eq!(calculate_card_points(&cards[1]), 2);
        assert_eq!(calculate_card_points(&cards[2]), 2);
        assert_eq!(calculate_card_points(&cards[3]), 1);
        assert_eq!(calculate_card_points(&cards[4]), 0);
        assert_eq!(calculate_card_points(&cards[5]), 0);
    }

    #[test]
    fn test_part_one_solution() {
        let cards = test_data();
        assert_eq!(part_one_solution(&cards), 13);
    }

    #[test]
    fn test_part_one_answer() {
        let cards = process_input(&String::from(include_str!("../data/puzzle_input.txt")));
        assert_eq!(part_one_solution(&cards), 26346);
    }
}
