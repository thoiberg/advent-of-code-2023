// TODO: Replace tuples with Structs. Tuples with multiple same types are
// confusing to work with.
// TODO: Redo the entire input processing code, it's gross
use std::collections::{HashMap, VecDeque};

fn main() {
    let input = include_str!("../data/puzzle_input.txt");
    let mut modules = process_input(input);

    let part_one_answer = part_one_solution(&mut modules);
    println!("The Answer for part one is {part_one_answer}");
}

type Modules = HashMap<String, Box<dyn Module>>;

fn part_one_solution(modules: &mut Modules) -> u64 {
    let mut low_count = 0;
    let mut high_count = 0;
    (0..1000).for_each(|_| {
        let (round_low_count, round_high_count) = push_button(modules);

        low_count += round_low_count;
        high_count += round_high_count;
    });

    low_count * high_count
}

fn push_button(modules: &mut Modules) -> (u64, u64) {
    let mut pulses: VecDeque<(String, String, Pulse)> = VecDeque::new();
    let mut low_pulse_count = 0;
    let mut high_pulse_count = 0;

    let broadcaster = modules
        .get_mut("broadcaster")
        .expect("broadcaster not found");

    let initial_pulses = broadcaster
        .receive_pulse(Pulse::Low, "a")
        .expect("Broadcaster had no pulses");

    low_pulse_count += 1;

    initial_pulses.0.iter().for_each(|dest| {
        pulses.push_back((String::from("broadcaster"), dest.clone(), initial_pulses.1));
    });

    while !pulses.is_empty() {
        let next_pulse = pulses.pop_front().unwrap();
        // println!(
        //     "running {} -{}-> {}",
        //     next_pulse.0, next_pulse.2, next_pulse.1
        // );

        match next_pulse.2 {
            Pulse::High => high_pulse_count += 1,
            Pulse::Low => low_pulse_count += 1,
        }

        let next_destination = modules.get_mut(&next_pulse.1);

        if let Some(next_destination) = next_destination {
            let more_pulses = next_destination.receive_pulse(next_pulse.2, &next_pulse.0);

            if let Some(more_pulses) = more_pulses {
                more_pulses.0.into_iter().for_each(|next_next_destination| {
                    pulses.push_back((next_pulse.1.clone(), next_next_destination, more_pulses.1));
                })
            }
        }
    }

    (low_pulse_count, high_pulse_count)
}
trait Module {
    fn receive_pulse(&mut self, pulse: Pulse, input: &str) -> Option<(Vec<String>, Pulse)>;
}
struct FlipFlop {
    is_on: bool,
    destinations: Vec<String>,
}

impl FlipFlop {
    fn new(destinations: Vec<String>) -> Self {
        Self {
            is_on: false,
            destinations,
        }
    }
}

impl Module for FlipFlop {
    fn receive_pulse(&mut self, pulse: Pulse, _: &str) -> Option<(Vec<String>, Pulse)> {
        let next_pulse: Option<(Vec<String>, Pulse)> = match pulse {
            Pulse::High => None,
            Pulse::Low => {
                let boop = if self.is_on {
                    Some((self.destinations.clone(), Pulse::Low))
                } else {
                    Some((self.destinations.clone(), Pulse::High))
                };

                self.is_on = !self.is_on;

                boop
            }
        };

        next_pulse
    }
}

struct Conjunction {
    most_recent_pulses: HashMap<String, Pulse>,
    destinations: Vec<String>,
}

impl Conjunction {
    fn new(sources: Vec<String>, destinations: Vec<String>) -> Self {
        let mut most_recent_pulses: HashMap<String, Pulse> = HashMap::new();
        sources.into_iter().for_each(|source| {
            most_recent_pulses.insert(source, Pulse::Low);
        });

        Self {
            most_recent_pulses,
            destinations,
        }
    }
}

impl Module for Conjunction {
    fn receive_pulse(&mut self, pulse: Pulse, input: &str) -> Option<(Vec<String>, Pulse)> {
        self.most_recent_pulses.insert(String::from(input), pulse);

        let all_highs = self
            .most_recent_pulses
            .iter()
            .all(|(_, pulse)| pulse == &Pulse::High);

        if all_highs {
            Some((self.destinations.clone(), Pulse::Low))
        } else {
            Some((self.destinations.clone(), Pulse::High))
        }
    }
}

struct Broadcaster {
    destinations: Vec<String>,
}

impl Module for Broadcaster {
    fn receive_pulse(&mut self, pulse: Pulse, _: &str) -> Option<(Vec<String>, Pulse)> {
        Some((self.destinations.clone(), pulse))
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Copy)]
enum Pulse {
    High,
    Low,
}

impl std::fmt::Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::High => f.write_str("high"),
            Self::Low => f.write_str("low"),
        }
    }
}

fn process_input(input: &str) -> Modules {
    let mut modules: Modules = HashMap::new();

    let module_data: Vec<_> = input
        .lines()
        .map(|line| {
            let (name, destinations) = line.split_once(" -> ").unwrap();

            let destinations: Vec<String> = destinations.split(", ").map(String::from).collect();
            let real_name: &str;
            let module_type: &str;

            if name == "broadcaster" {
                real_name = "broadcaster";
                module_type = "broadcaster";
            } else if name.starts_with('&') {
                (_, real_name) = name.split_once('&').unwrap();
                module_type = "&";
            } else if name.starts_with('%') {
                (_, real_name) = name.split_once('%').unwrap();
                module_type = "%";
            } else {
                panic!("oops");
            }

            (real_name, module_type, destinations)
        })
        .collect();

    module_data
        .iter()
        .for_each(|(name, module_type, destinations)| {
            // TODO: try to use a match statement
            let module: Box<dyn Module> = if module_type == &"broadcaster" {
                Box::new(Broadcaster {
                    destinations: destinations.clone(),
                })
            } else if module_type == &"%" {
                Box::new(FlipFlop::new(destinations.clone()))
            } else if module_type == &"&" {
                let sources: Vec<_> = module_data
                    .iter()
                    .filter_map(|(source_name, _, destinations)| {
                        if destinations.contains(&name.to_string()) {
                            Some(String::from(*source_name))
                        } else {
                            None
                        }
                    })
                    .collect();
                Box::new(Conjunction::new(sources, destinations.clone()))
            } else {
                panic!("Could not process module")
            };

            modules.insert(name.to_string(), module);
        });

    modules
}

#[cfg(test)]
mod test_super {
    use super::*;

    fn simple_test_data() -> Modules {
        process_input(include_str!("../data/simple_test_input.txt"))
    }

    fn complex_test_data() -> Modules {
        process_input(include_str!("../data/complex_test_input.txt"))
    }

    #[test]
    fn test_process_input() {
        let test_data = simple_test_data();

        assert_eq!(test_data.len(), 5);
        assert!(test_data.get("a").is_some());
    }

    #[test]
    fn test_broadcaster_receive_pulse() {
        let mut broadcaster = Broadcaster {
            destinations: vec![String::from("a"), String::from("b"), String::from("c")],
        };

        let pulse_output = broadcaster.receive_pulse(Pulse::High, "a").unwrap();

        assert_eq!(
            pulse_output.0,
            vec![String::from("a"), String::from("b"), String::from("c")]
        );
        assert_eq!(pulse_output.1, Pulse::High)
    }

    #[test]
    fn test_flip_flop_receive_high_pulse() {
        let mut flip_flop = FlipFlop::new(vec![String::from("a"), String::from("b")]);

        let pulse_output = flip_flop.receive_pulse(Pulse::High, "a");

        assert!(pulse_output.is_none());
    }

    #[test]
    fn test_flip_flop_receive_low_pulse() {
        let mut flip_flop = FlipFlop::new(vec![String::from("a"), String::from("b")]);

        let pulse_output = flip_flop.receive_pulse(Pulse::Low, "a").unwrap();

        assert_eq!(pulse_output.0, vec![String::from("a"), String::from("b")]);
        assert_eq!(pulse_output.1, Pulse::High);
        assert!(flip_flop.is_on);

        let pulse_output = flip_flop.receive_pulse(Pulse::Low, "a").unwrap();
        assert_eq!(pulse_output.0, vec![String::from("a"), String::from("b")]);
        assert_eq!(pulse_output.1, Pulse::Low);
        assert!(!flip_flop.is_on);
    }

    #[test]
    fn test_conjunction_receive_pulse_with_all_most_recent_lows() {
        let destinations = vec![String::from("a"), String::from("b")];
        let mut conjunction = Conjunction::new(vec![String::from("d")], destinations);

        let pulse_output = conjunction.receive_pulse(Pulse::Low, "d").unwrap();
        assert_eq!(
            conjunction.most_recent_pulses.get("d").unwrap(),
            &Pulse::Low
        );
        assert_eq!(pulse_output.1, Pulse::High);
    }

    #[test]
    fn test_conjunction_receive_pulse_with_all_most_recent_highs() {
        let destinations = vec![String::from("a")];
        let mut conjunction = Conjunction::new(vec![String::from("d")], destinations);

        let pulse_output = conjunction.receive_pulse(Pulse::High, "d").unwrap();
        assert_eq!(
            conjunction.most_recent_pulses.get("d").unwrap(),
            &Pulse::High
        );
        assert_eq!(pulse_output.1, Pulse::Low);
    }

    #[test]
    fn test_conjunction_receive_pulse_with_mixed() {
        let destinations = vec![String::from("a")];
        let mut conjunction = Conjunction::new(vec![String::from("d")], destinations);

        conjunction.receive_pulse(Pulse::High, "d").unwrap();
        let pulse_output = conjunction.receive_pulse(Pulse::Low, "e").unwrap();

        assert_eq!(pulse_output.1, Pulse::High);
    }

    #[test]
    fn test_push_button_with_simple_data() {
        let mut test_modules = simple_test_data();
        let (low_count, high_count) = push_button(&mut test_modules);

        assert_eq!(low_count, 8);
        assert_eq!(high_count, 4);
    }

    #[test]
    fn test_push_button_with_complex_data() {
        let mut test_modules = complex_test_data();
        let (low_count, high_count) = push_button(&mut test_modules);

        assert_eq!(low_count, 4);
        assert_eq!(high_count, 4);
    }

    #[test]
    fn test_part_one_simple_example() {
        let mut test_modules = simple_test_data();
        let answer = part_one_solution(&mut test_modules);

        assert_eq!(answer, 32_000_000);
    }

    #[test]
    fn test_part_one_complex_example() {
        let mut test_modules = complex_test_data();
        let answer = part_one_solution(&mut test_modules);

        assert_eq!(answer, 11_687_500);
    }

    #[test]
    fn test_part_one_answer() {
        let mut modules = process_input(include_str!("../data/puzzle_input.txt"));

        assert_eq!(part_one_solution(&mut modules), 919_383_692);
    }
}
