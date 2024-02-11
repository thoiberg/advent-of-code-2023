// TODO: Make faster, currently takes ~4 secs to complete for Part 1

use ndarray::{Array2, Axis};
use std::{error::Error, fmt::Display, str::FromStr};

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../data/puzzle_input.txt");
    let map = process_input(input)?;

    let part_one_answer = part_one_solution(&map);
    println!("Part one answer is {part_one_answer}");

    Ok(())
}

type ForestMap = Array2<Tile>;

fn part_one_solution(map: &ForestMap) -> usize {
    let start = find_start(map).expect("Could not find the start position");
    let end = find_end(map).expect("Could not find the end position");

    let mut paths = vec![Path { tiles: vec![start] }];
    let mut complete_paths: Vec<Path> = vec![];

    while let Some(current_path) = paths.pop() {
        let current_tile = current_path.tiles.last().unwrap();
        if current_tile == &end {
            complete_paths.push(current_path.clone());
            continue;
        }

        let next_possible_directions = get_next_directions(current_tile);
        let next_tiles = get_next_tiles(map, &current_path, &next_possible_directions);

        next_tiles.into_iter().for_each(|next_tile| {
            let mut next_path = current_path.clone();
            next_path.tiles.push(next_tile);
            paths.push(next_path);
        });
    }

    let longest_path = complete_paths
        .iter()
        .max_by(|x, y| x.tiles.len().cmp(&y.tiles.len()));

    longest_path
        .expect("Coud not find longest path")
        .tiles
        .len()
        - 1 // don't count the start position
}

fn find_start(map: &ForestMap) -> Option<&Tile> {
    map.row(0)
        .into_iter()
        .find(|tile| tile.r#type == TileType::Path)
}

fn find_end(map: &ForestMap) -> Option<&Tile> {
    let last_row = map.len_of(Axis(0)) - 1;

    map.row(last_row)
        .into_iter()
        .find(|tile| tile.r#type == TileType::Path)
}

fn get_next_directions(tile: &Tile) -> Vec<Direction> {
    match &tile.r#type {
        TileType::Path => vec![
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ],
        TileType::Slope(dir) => vec![*dir],
        TileType::Forest => panic!("Current tile is Forest, which should not be possible"),
    }
}
fn get_next_tiles<'a>(
    map: &'a ForestMap,
    current_path: &Path,
    directions: &[Direction],
) -> Vec<&'a Tile> {
    let (current_x, current_y) = current_path.tiles.last().unwrap().position;

    directions
        .iter()
        .map(|direction| match direction {
            Direction::Up => (Some(current_x), current_y.checked_sub(1)),
            Direction::Right => (current_x.checked_add(1), Some(current_y)),
            Direction::Down => (Some(current_x), current_y.checked_add(1)),
            Direction::Left => (current_x.checked_sub(1), Some(current_y)),
        })
        .flat_map(|(next_x, next_y)| next_x.zip(next_y).and_then(|(x, y)| map.get((y, x))))
        .filter(|tile| tile.r#type != TileType::Forest)
        .filter(|tile| !current_path.tiles.contains(tile))
        .collect()
}

fn process_input(input: &str) -> Result<ForestMap, Box<dyn Error>> {
    let map: Vec<Vec<Tile>> = input
        .lines()
        .enumerate()
        .map(|(y_idx, line)| {
            line.chars()
                .enumerate()
                .map(|(x_indx, position)| {
                    let tile_type = TileType::from_str(&position.to_string()).unwrap();

                    Tile {
                        r#type: tile_type,
                        position: (x_indx, y_idx),
                    }
                })
                .collect()
        })
        .collect();

    let row_length = map[0].len();
    let col_length = map.len();

    Array2::from_shape_vec(
        (row_length, col_length),
        map.into_iter().flatten().collect(),
    )
    .map_err(|err| err.into())
}

#[derive(PartialEq, Eq, Debug)]
struct Tile {
    position: (usize, usize),
    r#type: TileType,
}

#[derive(PartialEq, Eq, Debug)]
enum TileType {
    Path,
    Forest,
    Slope(Direction),
}

#[derive(Debug)]
struct ParseError;

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Unable to identify tile type from string")
    }
}
impl Error for ParseError {}

impl FromStr for TileType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "#" => Ok(Self::Forest),
            "." => Ok(Self::Path),
            ">" => Ok(Self::Slope(Direction::Right)),
            "<" => Ok(Self::Slope(Direction::Left)),
            "^" => Ok(Self::Slope(Direction::Up)),
            "v" => Ok(Self::Slope(Direction::Down)),
            _ => Err(ParseError),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Clone)]
struct Path<'a> {
    tiles: Vec<&'a Tile>,
}

#[cfg(test)]
mod test_super {
    use ndarray::Axis;

    use super::*;

    fn test_data() -> ForestMap {
        process_input(include_str!("../data/test_input.txt")).unwrap()
    }

    #[test]
    fn test_process_input() {
        let test_map = process_input(include_str!("../data/test_input.txt"));

        assert!(test_map.is_ok());

        let test_map = test_map.unwrap();
        assert_eq!(test_map.len_of(Axis(0)), 23);
        assert_eq!(test_map.len_of(Axis(1)), 23);
    }

    #[test]
    fn test_find_start() {
        let test_data = &test_data();
        let start_position = &test_data.get((0, 1)).unwrap();

        let found_start = find_start(test_data);

        assert!(found_start.is_some());
        assert_eq!(&found_start.unwrap(), start_position);
    }

    #[test]
    fn test_find_end() {
        let test_data = &test_data();
        let end_position = &test_data.get((22, 21)).unwrap();

        let found_end = find_end(test_data);

        assert!(found_end.is_some());
        assert_eq!(&found_end.unwrap(), end_position);
    }

    #[test]
    fn test_get_next_directions_for_paths() {
        let path_tile = Tile {
            position: (1, 1),
            r#type: TileType::Path,
        };

        assert_eq!(
            get_next_directions(&path_tile),
            vec![
                Direction::Up,
                Direction::Right,
                Direction::Down,
                Direction::Left,
            ]
        )
    }

    #[test]
    fn test_get_next_directions_for_slopes() {
        let up_slope_tile = Tile {
            position: (1, 1),
            r#type: TileType::Slope(Direction::Up),
        };
        assert_eq!(get_next_directions(&up_slope_tile), vec![Direction::Up]);

        let left_slope_tile = Tile {
            position: (1, 1),
            r#type: TileType::Slope(Direction::Left),
        };
        assert_eq!(get_next_directions(&left_slope_tile), vec![Direction::Left]);

        let down_slope_tile = Tile {
            position: (1, 1),
            r#type: TileType::Slope(Direction::Down),
        };
        assert_eq!(get_next_directions(&down_slope_tile), vec![Direction::Down]);

        let right_slope_tile = Tile {
            position: (1, 1),
            r#type: TileType::Slope(Direction::Right),
        };
        assert_eq!(
            get_next_directions(&right_slope_tile),
            vec![Direction::Right]
        );
    }

    #[test]
    fn test_get_next_tiles_from_center() {
        let test_data = test_data();

        // tiles above and below are forest, left and right are paths
        let tiles = vec![test_data.get((14, 7)).unwrap()];
        let current_path = Path { tiles };
        let directions = vec![
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ];
        let next_tiles = get_next_tiles(&test_data, &current_path, &directions);

        assert_eq!(next_tiles.len(), 2);
        assert_eq!(
            next_tiles[0],
            &Tile {
                position: (7, 13),
                r#type: TileType::Path
            }
        );
        assert_eq!(
            next_tiles[1],
            &Tile {
                position: (7, 15),
                r#type: TileType::Path
            }
        );
    }

    #[test]
    fn test_get_next_tiles_from_edge() {
        let test_data = test_data();

        // tiles above and below are forest, left and right are paths
        let tiles = vec![test_data.get((0, 1)).unwrap()];
        let current_path = Path { tiles };
        let directions = vec![
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ];
        let next_tiles = get_next_tiles(&test_data, &current_path, &directions);

        assert_eq!(next_tiles.len(), 1);
        assert_eq!(
            next_tiles[0],
            &Tile {
                position: (1, 1),
                r#type: TileType::Path
            }
        );
    }

    #[test]
    fn test_get_next_tiles_with_previous_tiles() {
        let test_data = test_data();

        let tiles = vec![
            test_data.get((1, 4)).unwrap(),
            test_data.get((1, 5)).unwrap(),
        ];
        let current_path = Path { tiles };
        let directions = vec![
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ];
        let next_tiles = get_next_tiles(&test_data, &current_path, &directions);

        assert_eq!(next_tiles.len(), 1);
        assert_eq!(next_tiles[0], test_data.get((1, 6)).unwrap());
    }

    #[test]
    fn test_part_one_example() {
        let test_data = test_data();

        assert_eq!(part_one_solution(&test_data), 94);
    }

    #[test]
    fn test_part_one_answer() {
        let map = process_input(include_str!("../data/puzzle_input.txt")).unwrap();

        assert_eq!(part_one_solution(&map), 2094);
    }
}
