// Implementation heavily cribbed from https://advent-of-code.xavd.id/writeups/2023/day/18/
// Still don't fully understand Pick's theorem, I thought I could just pass in a list of vertices but no dice
use std::str::FromStr;

use regex::Regex;
use strum::EnumString;

fn main() {
    let input = include_str!("../data/puzzle_input.txt");
    let instructions = process_input(input);

    let part_one_solution = part_one_answer(&instructions);
    println!("Part one answer: {part_one_solution}");
}

#[derive(EnumString, PartialEq, Eq, Debug)]
enum Direction {
    #[strum(serialize = "U")]
    Up,
    #[strum(serialize = "R")]
    Right,
    #[strum(serialize = "D")]
    Down,
    #[strum(serialize = "L")]
    Left,
}

struct Instruction {
    direction: Direction,
    metres: u32,
    #[allow(dead_code)]
    colour_code: String,
}
#[derive(Clone, Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn move_to(&self, direction: &Direction, metres: &u32) -> Coordinate {
        match direction {
            Direction::Up => Coordinate {
                x: self.x,
                y: self.y - (*metres as i32),
            },
            Direction::Right => Coordinate {
                x: self.x + (*metres as i32),
                y: self.y,
            },
            Direction::Down => Coordinate {
                x: self.x,
                y: self.y + (*metres as i32),
            },
            Direction::Left => Coordinate {
                x: self.x - (*metres as i32),
                y: self.y,
            },
        }
    }
}

fn part_one_answer(instructions: &[Instruction]) -> f32 {
    let coordinates = generate_coordinates(instructions);
    let border_length = calculate_border(instructions);
    let shoelace_area = shoelace(&coordinates);

    picks_theorem(&border_length, &shoelace_area)
}

fn generate_coordinates(instructions: &[Instruction]) -> Vec<Coordinate> {
    let mut current_point = Coordinate { x: 0, y: 0 };
    let mut coordinates = vec![current_point.clone()];

    instructions.iter().for_each(|instruction| {
        let next_point = current_point.move_to(&instruction.direction, &instruction.metres);

        coordinates.push(next_point.clone());
        current_point = next_point;
    });

    coordinates
}

fn shoelace(coordinates: &[Coordinate]) -> f32 {
    let mut total_x = 0;
    let mut total_y = 0;

    for i in 0..coordinates.len() {
        let first = &coordinates[i];
        let second = coordinates.get(i + 1).unwrap_or(&coordinates[0]);

        total_x += first.x * second.y;
        total_y += first.y * second.x;
    }

    total_x.abs_diff(total_y) as f32 / 2.0
}

fn calculate_border(instructions: &[Instruction]) -> u32 {
    instructions
        .iter()
        .fold(0, |acc, instruction| acc + instruction.metres)
}

fn picks_theorem(border_length: &u32, shoelace_area: &f32) -> f32 {
    // int(abs(area) - 0.5 * len(outline) + 1) + len(outline)

    let outline_length = *border_length as f32;

    let i = shoelace_area + 1.0 - (outline_length * 0.5);

    i + outline_length
}

fn process_input(input: &str) -> Vec<Instruction> {
    let re = Regex::new(r"([RDUL]) (\d+)\s\((#[a-z0-9]{6})\)").unwrap();

    input
        .lines()
        .map(|line| {
            let caps = re.captures(line).expect("Regex failed");

            let direction = Direction::from_str(&caps[1]).unwrap();
            let meters = caps[2].parse::<u32>().unwrap();
            let colour_code = caps[3].to_string();

            Instruction {
                direction,
                metres: meters,
                colour_code,
            }
        })
        .collect()
}

#[cfg(test)]
mod test_super {
    use super::*;

    fn test_data() -> Vec<Instruction> {
        process_input(include_str!("../data/test_input.txt"))
    }

    #[test]
    fn test_process_input() {
        let test_data = test_data();

        assert_eq!(test_data.len(), 14);

        let first = &test_data[0];
        assert_eq!(first.direction, Direction::Right);
        assert_eq!(first.metres, 6);
        assert_eq!(first.colour_code, "#70c710");
    }

    #[test]
    fn test_generate_coordinates() {
        let coordinates = generate_coordinates(&test_data());

        assert_eq!(coordinates.len(), 15);

        // shape should be complete (ie, last should be the same position as the first one)
        assert_eq!(&coordinates[0].x, &coordinates.last().unwrap().x);
        assert_eq!(&coordinates[0].y, &coordinates.last().unwrap().y);

        assert_eq!(&coordinates[1].x, &6);
        assert_eq!(&coordinates[1].y, &0);

        assert_eq!(&coordinates[2].x, &6);
        assert_eq!(&coordinates[2].y, &5);
    }

    #[test]
    fn test_calculate_border() {
        assert_eq!(calculate_border(&test_data()), 38);
    }

    #[test]
    fn test_shoelace() {
        let coordinates = generate_coordinates(&test_data());
        assert_eq!(shoelace(&coordinates), 42.0);
    }

    #[test]
    fn test_picks_theorem() {
        let coordinates = generate_coordinates(&test_data());
        let border_length = calculate_border(&test_data());
        let shoelace_area = shoelace(&coordinates);

        assert_eq!(picks_theorem(&border_length, &shoelace_area), 62.0);
    }

    #[test]
    fn test_part_one_example() {
        assert_eq!(part_one_answer(&test_data()), 62.0)
    }

    #[test]
    fn test_part_one_solution() {
        let instructions = &process_input(include_str!("../data/puzzle_input.txt"));
        assert_eq!(part_one_answer(instructions), 46359.0);
    }
}
