// TODO: Cache the results for each rule to speed up execution.
//      Since the data is static once a rule has been resolved to the
//      eventual outcome then it does not need to be re-calculated.
use std::{collections::HashMap, str::FromStr};

use regex::Regex;
use strum::EnumString;

type InputData = (HashMap<String, Queue>, Vec<Part>);

fn main() {
    let input = include_str!("../data/puzzle_input.txt");
    let data = process_input(input);

    let part_one_answer = part_one_solution(&data);
    println!("Part one answer is: {part_one_answer}");
}

fn part_one_solution(data: &InputData) -> u64 {
    let (queues, parts) = &data;
    let mut rejected_parts: Vec<&Part> = vec![];
    let mut accepted_parts: Vec<&Part> = vec![];
    let first_queue = queues.get("in").expect("Could not find \"in\" queue");

    parts.iter().for_each(|part| {
        let outcome = apply_queue(part, first_queue, queues);

        match outcome {
            OutCome::Accepted => accepted_parts.push(part),
            OutCome::Rejected => rejected_parts.push(part),
            _ => panic!("Should not have returned a non final outcome"),
        }
    });

    accepted_parts
        .iter()
        .fold(0, |acc, ap| acc + ap.total_value())
}

fn apply_queue(part: &Part, queue: &Queue, queues: &HashMap<String, Queue>) -> OutCome {
    let rule_check = queue.rules.iter().find_map(|rule| rule.check(part));

    let queue_outcome = rule_check.unwrap_or(queue.default.clone());

    match queue_outcome {
        OutCome::Redirect(next_queue_name) => {
            let next_queue = queues
                .get(&next_queue_name)
                .expect("could not find next queue");

            apply_queue(part, next_queue, queues)
        }
        final_outcome => final_outcome,
    }
}

fn process_input(input: &str) -> InputData {
    let (workflows, parts_chunk) = input.split_once("\n\n").unwrap();

    let workflow_re = Regex::new(r"(\w+)\{(.+)\}").unwrap();
    let queues: Vec<_> = workflows
        .lines()
        .map(|workflow| {
            let caps = workflow_re.captures(workflow).unwrap();
            let queue_name = String::from(&caps[1]);
            let ruleset = &caps[2];

            let mut rules: Vec<_> = ruleset.split(',').collect();

            let default = rules.remove(rules.len() - 1);
            let default_outcome = match default {
                "A" => OutCome::Accepted,
                "R" => OutCome::Rejected,
                queue_name => OutCome::Redirect(queue_name.to_string()),
            };

            let rule_re =
                Regex::new(r"(?<cat>[xmas])(?<comp>[<>])(?<amount>\d+):(?<dest>\w+)").unwrap();

            let rules: Vec<_> = rules
                .into_iter()
                .map(|rule| {
                    let caps = rule_re.captures(rule).unwrap();

                    let category = caps.name("cat").unwrap().as_str();
                    let comparator = caps.name("comp").unwrap().as_str();
                    let amount = caps
                        .name("amount")
                        .unwrap()
                        .as_str()
                        .parse::<u32>()
                        .unwrap();
                    let dest = caps.name("dest").unwrap().as_str();

                    let dest = match dest {
                        "R" => OutCome::Rejected,
                        "A" => OutCome::Accepted,
                        queue_name => OutCome::Redirect(String::from(queue_name)),
                    };

                    Rule {
                        category: Category::from_str(category).unwrap(),
                        comparison_type: ComparisonType::from_str(comparator).unwrap(),
                        amount,
                        outcome: dest,
                    }
                })
                .collect();

            Queue {
                name: queue_name,
                rules,
                default: default_outcome,
            }
        })
        .collect();

    let mut queue_hash: HashMap<String, Queue> = HashMap::new();

    queues.into_iter().for_each(|queue| {
        queue_hash.insert(queue.name.clone(), queue);
    });

    let parts_re = Regex::new(r"\{x=(?<x>\d+),m=(?<m>\d+),a=(?<a>\d+),s=(?<s>\d+)\}").unwrap();
    let parts: Vec<_> = parts_chunk
        .lines()
        .map(|line| {
            let caps = parts_re.captures(line).unwrap();
            let x = caps.name("x").unwrap().as_str().parse::<u32>().unwrap();
            let m = caps.name("m").unwrap().as_str().parse::<u32>().unwrap();
            let a = caps.name("a").unwrap().as_str().parse::<u32>().unwrap();
            let s = caps.name("s").unwrap().as_str().parse::<u32>().unwrap();

            Part { x, m, a, s }
        })
        .collect();

    (queue_hash, parts)
}

struct Queue {
    name: String,
    rules: Vec<Rule>,
    default: OutCome,
}

#[derive(EnumString, PartialEq, Eq, Debug)]
enum ComparisonType {
    #[strum(serialize = "<")]
    Le,
    #[strum(serialize = ">")]
    Ge,
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum OutCome {
    Accepted,
    Rejected,
    Redirect(String),
}

#[derive(EnumString, PartialEq, Eq, Debug)]
enum Category {
    #[strum(serialize = "x")]
    X,
    #[strum(serialize = "m")]
    M,
    #[strum(serialize = "a")]
    A,
    #[strum(serialize = "s")]
    S,
}

struct Rule {
    category: Category,
    comparison_type: ComparisonType,
    amount: u32,
    outcome: OutCome,
}

impl Rule {
    fn check(&self, part: &Part) -> Option<OutCome> {
        let category_value = part.category_for(&self.category);

        match self.comparison_type {
            ComparisonType::Ge => {
                if category_value.gt(&self.amount) {
                    Some(self.outcome.clone())
                } else {
                    None
                }
            }
            ComparisonType::Le => {
                if category_value.lt(&self.amount) {
                    Some(self.outcome.clone())
                } else {
                    None
                }
            }
        }
    }
}

struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn total_value(&self) -> u64 {
        (self.x + self.m + self.a + self.s).into()
    }

    fn category_for(&self, cat: &Category) -> u32 {
        match cat {
            Category::X => self.x,
            Category::M => self.m,
            Category::A => self.a,
            Category::S => self.s,
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    fn test_data() -> InputData {
        process_input(include_str!("../data/test_input.txt"))
    }

    #[test]
    fn test_process_input() {
        let (queues, parts) = test_data();

        // Queues
        assert_eq!(queues.len(), 11);

        let first_queue = queues.get("px").unwrap();
        assert_eq!(first_queue.name, "px");
        assert_eq!(first_queue.rules.len(), 2);
        assert_eq!(first_queue.default, OutCome::Redirect("rfg".to_string()));

        let first_rule = &first_queue.rules[0];
        assert_eq!(first_rule.category, Category::A);
        assert_eq!(first_rule.comparison_type, ComparisonType::Le);
        assert_eq!(first_rule.amount, 2006);
        assert_eq!(first_rule.outcome, OutCome::Redirect(String::from("qkq")));
        // Parts

        assert_eq!(parts.len(), 5);

        let first_part = &parts[0];
        assert_eq!(first_part.x, 787);
        assert_eq!(first_part.m, 2655);
        assert_eq!(first_part.a, 1222);
        assert_eq!(first_part.s, 2876);

        let second_part = &parts[1];
        assert_eq!(second_part.x, 1679);
        assert_eq!(second_part.m, 44);
        assert_eq!(second_part.a, 2067);
        assert_eq!(second_part.s, 496);
    }

    #[test]
    fn test_part_one_example() {
        let part_one_answer = part_one_solution(&test_data());

        assert_eq!(part_one_answer, 19114);
    }

    #[test]
    fn test_part_one_answer() {
        let data = process_input(include_str!("../data/puzzle_input.txt"));

        assert_eq!(part_one_solution(&data), 374873);
    }
}
