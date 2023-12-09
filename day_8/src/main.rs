use std::str::FromStr;

use regex::Regex;
use strum::EnumString;

fn main() {
    println!("Hello, world!");
    let input = read_input();
    let (directions, nodes) = process_input(input);

    let part_one_answer = part_one_solution(&directions, &nodes);
    println!("Part One Answer is: {part_one_answer}");
}

fn part_one_solution(directions: &[Direction], nodes: &[Node]) -> u32 {
    let mut current_node = nodes.iter().find(|node| node.id == "AAA").unwrap();
    let mut step_counter = 0;

    for direction in directions.iter().cycle() {
        if current_node.id == "ZZZ" {
            break;
        }

        match direction {
            Direction::L => {
                current_node = nodes
                    .iter()
                    .find(|node| node.id == current_node.left)
                    .unwrap();
            }
            Direction::R => {
                current_node = nodes
                    .iter()
                    .find(|node| node.id == current_node.right)
                    .unwrap()
            }
        }

        step_counter += 1;
    }

    step_counter
}

fn read_input() -> String {
    String::from(include_str!("../data/puzzle_input.txt"))
}

fn process_input(input: String) -> (Vec<Direction>, Vec<Node>) {
    let lines: Vec<&str> = input.split("\n\n").collect();

    let directions = lines[0]
        .chars()
        .map(|direction| Direction::from_str(direction.to_string().as_str()).unwrap())
        .collect();

    let re =
        Regex::new(r"(?<node_id>[A-Z]{3}) = \((?<left>[A-Z]{3}), (?<right>[A-Z]{3})\)").unwrap();

    let nodes = lines[1]
        .lines()
        .map(|line| {
            let captures = re.captures(line).unwrap();

            Node {
                id: captures.name("node_id").unwrap().as_str().to_owned(),
                left: captures.name("left").unwrap().as_str().to_owned(),
                right: captures.name("right").unwrap().as_str().to_owned(),
            }
        })
        .collect();

    (directions, nodes)
}

#[derive(EnumString, PartialEq, Debug)]
enum Direction {
    #[strum(serialize = "L")]
    L,
    #[strum(serialize = "R")]
    R,
}

#[derive(PartialEq, Debug)]
struct Node {
    id: String,
    left: String,
    right: String,
}

#[cfg(test)]
mod test_super {
    use super::*;

    fn test_data() -> (Vec<Direction>, Vec<Node>) {
        process_input(String::from(include_str!("../data/test_input.txt")))
    }

    fn test_repeating_data() -> (Vec<Direction>, Vec<Node>) {
        process_input(String::from(include_str!(
            "../data/test_input_repeating.txt"
        )))
    }

    #[test]
    fn test_process_input() {
        let (directions, nodes) = test_data();

        assert_eq!(directions, vec![Direction::R, Direction::L]);
        assert_eq!(
            nodes[0],
            Node {
                id: "AAA".to_string(),
                left: "BBB".to_string(),
                right: "CCC".to_string()
            }
        );
        assert_eq!(
            nodes[1],
            Node {
                id: "BBB".to_string(),
                left: "DDD".to_string(),
                right: "EEE".to_string()
            }
        );
        assert_eq!(
            nodes[2],
            Node {
                id: "CCC".to_string(),
                left: "ZZZ".to_string(),
                right: "GGG".to_string()
            }
        );
    }

    #[test]
    fn test_part_one_test_answer() {
        let (directions, nodes) = test_data();
        let part_one_answer = part_one_solution(&directions, &nodes);

        assert_eq!(part_one_answer, 2);
    }

    #[test]
    fn test_part_one_test_repeating_answer() {
        let (directions, nodes) = test_repeating_data();
        let part_one_answer = part_one_solution(&directions, &nodes);

        assert_eq!(part_one_answer, 6);
    }

    #[test]
    fn test_part_one_real_answer() {
        let (directions, nodes) =
            process_input(include_str!("../data/puzzle_input.txt").to_string());
        let part_one_answer = part_one_solution(&directions, &nodes);

        assert_eq!(part_one_answer, 20777);
    }
}
