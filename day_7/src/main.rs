use std::{collections::HashMap, str::FromStr};

use strum::EnumString;

fn main() {
    let input = read_input();
    let rounds = process_input(input);

    let part_one_answer = part_one_solution(rounds);
    println!("Part One answer is: {part_one_answer}");
}

fn part_one_solution(mut rounds: Vec<Round>) -> u64 {
    rounds.sort();

    rounds.iter().enumerate().fold(0, |acc, (idx, round)| {
        acc + (((idx + 1) as u64) * (round.bid as u64))
    })
}

fn read_input() -> String {
    String::from(include_str!("../data/puzzle_input.txt"))
}

fn process_input(input: String) -> Vec<Round> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(' ').collect();
            let bid = parts[1].parse::<u32>().unwrap();
            let cards = parts[0]
                .chars()
                .map(|card| Card::from_str(card.to_string().as_str()).unwrap())
                .collect();

            let hand = Hand::new(cards);
            Round { bid, hand }
        })
        .collect()
}

#[derive(PartialEq, Eq)]
struct Round {
    bid: u32,
    hand: Hand,
}

impl PartialOrd for Round {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.hand.partial_cmp(&other.hand)
    }
}

impl Ord for Round {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand.partial_cmp(&other.hand).unwrap()
    }
}

#[derive(PartialEq, Eq, Debug, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, Eq, Debug, PartialOrd)]
struct Hand {
    r#type: HandType,
    cards: Vec<Card>,
}

impl Hand {
    fn new(cards: Vec<Card>) -> Self {
        let hand_type = Self::determine_hand_type(&cards);
        Self {
            cards,
            r#type: hand_type,
        }
    }

    fn determine_hand_type(cards: &[Card]) -> HandType {
        // TODO: see if there's an iterator for this
        let mut card_type_count: HashMap<&Card, u32> = HashMap::new();
        for card in cards {
            let update = if let Some(count) = card_type_count.get(card) {
                count + 1
            } else {
                1
            };

            card_type_count.insert(card, update);
        }

        let highest_card = card_type_count.iter().max_by_key(|(_k, v)| *v).unwrap();

        match card_type_count.len() {
            1 => HandType::FiveOfAKind,
            2 => {
                if highest_card.1 == &4 {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                if highest_card.1 == &3 {
                    HandType::ThreeOfAKind
                } else {
                    HandType::TwoPair
                }
            }
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => panic!("Only expected 5 possible combos, how did I get here?"),
        }
    }
}

#[derive(PartialEq, PartialOrd, EnumString, Debug, Hash, Eq)]
enum Card {
    #[strum(serialize = "2")]
    Two,
    #[strum(serialize = "3")]
    Three,
    #[strum(serialize = "4")]
    Four,
    #[strum(serialize = "5")]
    Five,
    #[strum(serialize = "6")]
    Six,
    #[strum(serialize = "7")]
    Seven,
    #[strum(serialize = "8")]
    Eight,
    #[strum(serialize = "9")]
    Nine,
    #[strum(serialize = "T")]
    Ten,
    J,
    Q,
    K,
    A,
}

#[cfg(test)]
mod test_super {
    use std::cmp;

    use super::*;

    fn test_data() -> String {
        String::from(include_str!("../data/test_input.txt"))
    }

    #[test]
    fn test_process_input() {
        let rounds = process_input(test_data());

        assert_eq!(rounds.len(), 5);

        let first_card = &rounds[0];
        assert_eq!(first_card.bid, 765);
        assert_eq!(
            first_card.hand.cards,
            vec![Card::Three, Card::Two, Card::Ten, Card::Three, Card::K]
        );
    }

    #[test]
    fn test_card_comparison() {
        assert!(Card::A > Card::Two);
        assert!(Card::K > Card::Q);
        assert!(Card::Ten > Card::Nine);
    }

    #[test]
    fn test_determine_card_type() {
        let five_of_a_kind = vec![Card::A, Card::A, Card::A, Card::A, Card::A];
        let five_card_type = Hand::determine_hand_type(&five_of_a_kind);
        assert_eq!(five_card_type, HandType::FiveOfAKind);

        let four_of_a_kind = vec![Card::A, Card::A, Card::Eight, Card::A, Card::A];
        let four_card_type = Hand::determine_hand_type(&four_of_a_kind);
        assert_eq!(four_card_type, HandType::FourOfAKind);

        let full_house = vec![Card::Two, Card::Three, Card::Three, Card::Three, Card::Two];
        let full_house_type = Hand::determine_hand_type(&full_house);
        assert_eq!(full_house_type, HandType::FullHouse);

        let three_of_a_kind = vec![Card::Ten, Card::Ten, Card::Ten, Card::Nine, Card::Eight];
        let three_of_a_kind_type = Hand::determine_hand_type(&three_of_a_kind);
        assert_eq!(three_of_a_kind_type, HandType::ThreeOfAKind);

        let two_pair = vec![Card::Two, Card::Three, Card::Four, Card::Three, Card::Two];
        let two_pair_type = Hand::determine_hand_type(&two_pair);
        assert_eq!(two_pair_type, HandType::TwoPair);

        let one_pair = vec![Card::A, Card::Two, Card::Three, Card::A, Card::Four];
        let one_pair_type = Hand::determine_hand_type(&one_pair);
        assert_eq!(one_pair_type, HandType::OnePair);

        let high_card = vec![Card::Two, Card::Three, Card::Four, Card::Five, Card::Six];
        let high_card_type = Hand::determine_hand_type(&high_card);
        assert_eq!(high_card_type, HandType::HighCard);
    }

    #[test]
    fn test_hand_comparison() {
        let five_aces = Hand {
            cards: vec![Card::A, Card::A, Card::A, Card::A, Card::A],
            r#type: HandType::FiveOfAKind,
        };
        let four_aces = Hand {
            cards: vec![Card::A, Card::A, Card::A, Card::A, Card::K],
            r#type: HandType::FourOfAKind,
        };

        assert!(five_aces > four_aces);

        let five_kings = Hand {
            cards: vec![Card::K, Card::K, Card::K, Card::K, Card::K],
            r#type: HandType::FiveOfAKind,
        };

        assert_eq!(
            five_aces.partial_cmp(&five_kings),
            Some(std::cmp::Ordering::Greater)
        );

        let three_jacks = Hand {
            cards: vec![Card::Two, Card::J, Card::J, Card::J, Card::Three],
            r#type: HandType::ThreeOfAKind,
        };
        let three_twos = Hand {
            cards: vec![Card::Q, Card::Two, Card::Two, Card::Two, Card::Three],
            r#type: HandType::ThreeOfAKind,
        };

        assert_eq!(
            three_jacks.partial_cmp(&three_twos),
            Some(cmp::Ordering::Less)
        );
    }

    #[test]
    fn test_sorting_rounds() {
        let mut rounds = process_input(test_data());

        rounds.sort();

        assert_eq!(rounds[0].bid, 765);
        assert_eq!(rounds[1].bid, 220);
        assert_eq!(rounds[2].bid, 28);
        assert_eq!(rounds[3].bid, 684);
        assert_eq!(rounds[4].bid, 483);
    }

    #[test]
    fn test_part_one_test_answer() {
        let rounds = process_input(test_data());
        let part_one_answer = part_one_solution(rounds);

        assert_eq!(part_one_answer, 6440);
    }

    #[test]
    fn test_part_one_real_answer() {
        let rounds = process_input(read_input());
        let part_one_answer = part_one_solution(rounds);

        assert_eq!(part_one_answer, 246_424_613);
    }
}
