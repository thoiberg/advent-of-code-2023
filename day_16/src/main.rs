use std::{collections::HashSet, str::FromStr};

use strum::EnumString;

fn main() {
    let input = include_str!("../data/puzzle_input.txt");
    let contraption = process_input(input);

    let part_one_answer = part_one_solution(&contraption);
    println!("Part One answer is: {part_one_answer}");
}

fn part_one_solution(contraption: &[Vec<Tile>]) -> usize {
    let mut beams: Vec<Beam> = vec![Beam {
        x: 0,
        y: 0,
        direction: Direction::Right,
    }];
    let mut energized_tiles: HashSet<&Tile> = HashSet::new();
    energized_tiles.insert(&contraption[0][0]);

    let mut used_splitters: HashSet<&Tile> = HashSet::new();

    while !beams.is_empty() {
        let mut new_beams: Vec<Beam> = vec![];

        for beam in beams {
            let current_tile = &contraption[beam.y][beam.x];

            if used_splitters.contains(current_tile) {
                continue;
            }

            let new_directions = match current_tile.r#type {
                TileType::Empty => vec![beam.direction],
                TileType::LeftMirror => match beam.direction {
                    Direction::Up => vec![Direction::Right],
                    Direction::Down => vec![Direction::Left],
                    Direction::Left => vec![Direction::Down],
                    Direction::Right => vec![Direction::Up],
                },
                TileType::RightMirror => match beam.direction {
                    Direction::Up => vec![Direction::Left],
                    Direction::Down => vec![Direction::Right],
                    Direction::Left => vec![Direction::Up],
                    Direction::Right => vec![Direction::Down],
                },
                TileType::VerticalSplitter => match beam.direction {
                    Direction::Up | Direction::Down => vec![beam.direction],
                    Direction::Left | Direction::Right => {
                        used_splitters.insert(current_tile);
                        vec![Direction::Up, Direction::Down]
                    }
                },
                TileType::HorizontalSplitter => match beam.direction {
                    Direction::Left | Direction::Right => vec![beam.direction],
                    Direction::Up | Direction::Down => {
                        used_splitters.insert(current_tile);
                        vec![Direction::Left, Direction::Right]
                    }
                },
            };

            for new_direction in new_directions {
                let next_position = new_position(beam.x, beam.y, &new_direction);

                let next_tile = next_position.and_then(|(x, y)| contraption.get(y)?.get(x));

                if let Some(next_tile) = next_tile {
                    energized_tiles.insert(next_tile);

                    new_beams.push(Beam {
                        x: next_tile.x,
                        y: next_tile.y,
                        direction: new_direction,
                    })
                }
            }
        }

        beams = new_beams;
    }

    // create a beam and add to beams vec
    // while beams are not empty
    // get each beam and move to the next tile
    // add each tile to the energized_tiles Set
    // if beam hits the edge (out of vec bounds) then remove from the beams
    // if written properly it _should_ eventually finish once all beams are empty
    energized_tiles.len()
}

fn new_position(
    current_x: usize,
    current_y: usize,
    direction: &Direction,
) -> Option<(usize, usize)> {
    let new_coords = match direction {
        Direction::Up => (Some(current_x), current_y.checked_sub(1)),
        Direction::Down => (Some(current_x), current_y.checked_add(1)),
        Direction::Left => (current_x.checked_sub(1), Some(current_y)),
        Direction::Right => (current_x.checked_add(1), Some(current_y)),
    };

    if let (Some(new_x), Some(new_y)) = new_coords {
        Some((new_x, new_y))
    } else {
        None
    }
}

fn process_input(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .enumerate()
        .map(|(y_index, line)| {
            line.chars()
                .enumerate()
                .map(|(x_index, spot)| Tile {
                    r#type: TileType::from_str(spot.to_string().as_str()).unwrap(),
                    x: x_index,
                    y: y_index,
                })
                .collect()
        })
        .collect()
}

#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Beam {
    x: usize,
    y: usize,
    direction: Direction,
}

#[derive(Debug, EnumString, PartialEq, Hash, Eq)]
enum TileType {
    #[strum(serialize = ".")]
    Empty,
    #[strum(serialize = "-")]
    HorizontalSplitter,
    #[strum(serialize = "|")]
    VerticalSplitter,
    #[strum(serialize = "\\")]
    RightMirror,
    #[strum(serialize = "/")]
    LeftMirror,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Tile {
    r#type: TileType,
    x: usize,
    y: usize,
}

#[cfg(test)]
mod test_super {
    use super::*;

    fn test_data() -> Vec<Vec<Tile>> {
        process_input(include_str!("../data/test_input.txt"))
    }

    #[test]
    fn test_part_one_example() {
        let test_data = test_data();

        assert_eq!(part_one_solution(&test_data), 46);
    }
}
