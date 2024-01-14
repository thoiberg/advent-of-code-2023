use std::{collections::HashSet, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../data/puzzle_input.txt");
    let garden = process_input(input)?;

    let part_one_answer = part_one_solution(&garden, 64);
    println!("Part One answer is {part_one_answer}");

    Ok(())
}

type Garden = Vec<Vec<TileType>>;

fn part_one_solution(garden: &Garden, number_of_steps: u32) -> usize {
    let max_x = garden[0].len();
    let max_y = garden.len();

    let start = find_start(garden);
    let mut possibilites: HashSet<(usize, usize)> = HashSet::new();
    possibilites.insert(start);

    for _ in 0..number_of_steps {
        let mut next_possibilites: HashSet<(usize, usize)> = HashSet::new();

        possibilites.iter().for_each(|coord| {
            find_cardinals(coord.0, coord.1, &max_x, &max_y)
                .into_iter()
                .filter(|(x, y)| !matches!(garden[*y][*x], TileType::Rock))
                .for_each(|coords| {
                    let _ = next_possibilites.insert(coords);
                });
        });

        possibilites = next_possibilites;
    }

    possibilites.len()
}

fn find_start(garden: &Garden) -> (usize, usize) {
    garden
        .iter()
        .enumerate()
        .find_map(|(y, line)| {
            let place_in_line = line.iter().enumerate().find_map(|(x, tile)| {
                if tile == &TileType::Start {
                    Some(x)
                } else {
                    None
                }
            });

            place_in_line.map(|x| (x, y))
        })
        .unwrap()
}

fn find_cardinals(x: usize, y: usize, max_x: &usize, max_y: &usize) -> Vec<(usize, usize)> {
    let above = (Some(x), y.checked_sub(1));
    let right = (x.checked_add(1), Some(y));
    let below = (Some(x), y.checked_add(1));
    let left = (x.checked_sub(1), Some(y));

    [above, right, below, left]
        .into_iter()
        .filter_map(|cardinal| {
            cardinal.0.and_then(|x| {
                cardinal.1.and_then(|y| {
                    if &x < max_x && &y < max_y {
                        Some((x, y))
                    } else {
                        None
                    }
                })
            })
        })
        .collect()
}

fn process_input(input: &str) -> Result<Garden, String> {
    input
        .lines()
        .map(|line| {
            let tiles = line.chars();
            tiles.map(TileType::try_from).collect()
        })
        .collect()
}

#[derive(Eq, PartialEq, Debug)]
enum TileType {
    Start,
    Garden,
    Rock,
}

impl TryFrom<char> for TileType {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'S' => Ok(Self::Start),
            '.' => Ok(Self::Garden),
            '#' => Ok(Self::Rock),
            _ => Err(format!("Could not determine tile type for: {}", value)),
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    fn test_data() -> Garden {
        process_input(include_str!("../data/test_input.txt")).unwrap()
    }

    #[test]
    fn test_process_input() {
        let garden = test_data();

        assert_eq!(garden.len(), 11);
        assert_eq!(garden[0].len(), 11);
        assert_eq!(garden[5][5], TileType::Start);
    }

    #[test]
    fn test_find_start() {
        let garden = test_data();

        let (x, y) = find_start(&garden);

        assert_eq!(x, 5);
        assert_eq!(y, 5);
    }

    #[test]
    fn test_part_one_example() {
        let garden = test_data();
        let answer = part_one_solution(&garden, 6);

        assert_eq!(answer, 16);
    }

    #[test]
    fn test_part_one_answer() {
        let garden = process_input(include_str!("../data/puzzle_input.txt")).unwrap();
        let answer = part_one_solution(&garden, 64);

        assert_eq!(answer, 3697);
    }
}
