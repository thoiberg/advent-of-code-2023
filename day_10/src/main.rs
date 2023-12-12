// TODO: Remove ndarray. I'm pretty sure I can do this with just a Vec<Vec<T>>
// just need to remember that x,y and are reversed when accessing it

use std::str::FromStr;

use ndarray::Array2;
use strum::EnumString;

type TileSet = Array2<Tile>;

fn main() {
    let input = include_str!("../data/puzzle_input.txt");
    let tiles = process_input(input);

    let part_one_answer = part_one_solution(&tiles);
    println!("The Part One solution is: {part_one_answer}");
}

fn part_one_solution(tiles: &TileSet) -> usize {
    let start_pos = find_start_tile(tiles);
    let start_surrounding_coords = find_surrounding_coordinates(start_pos);
    let connected_tiles = find_connected_tiles(tiles, start_pos, &start_surrounding_coords);

    if connected_tiles.len() != 2 {
        panic!(
            "Expected exactly 2 matching endpoints for the starting point, got: {}",
            connected_tiles.len()
        );
    }

    let mut path: Vec<&Tile> = vec![connected_tiles[0]];
    let mut last_tile: &Tile = start_pos;
    let mut current_tile = connected_tiles[0];

    while current_tile.r#type != TileType::Start {
        let connections = current_tile.connected_coordinates();

        // remove the node we came from from the list
        let next_step = connections
            .into_iter()
            .find(|conn| conn != &last_tile.coordinates)
            .unwrap();

        let next_tile = &tiles.get((next_step.1, next_step.0)).unwrap();

        path.push(next_tile);
        last_tile = current_tile;
        current_tile = next_tile;
    }

    // furtherst place from the start should be halfway through the path
    path.len() / 2
}

fn find_start_tile(tiles: &TileSet) -> &Tile {
    tiles
        .iter()
        .find(|tile| tile.r#type == TileType::Start)
        .unwrap()
}

fn find_surrounding_coordinates(tile: &Tile) -> Vec<(usize, usize)> {
    [
        // north
        (Some(tile.coordinates.0), tile.coordinates.1.checked_sub(1)),
        // east
        (tile.coordinates.0.checked_add(1), Some(tile.coordinates.1)),
        // south
        (Some(tile.coordinates.0), tile.coordinates.1.checked_add(1)),
        // west
        (tile.coordinates.0.checked_sub(1), Some(tile.coordinates.1)),
    ]
    .iter()
    .filter_map(|(x, y)| {
        if x.is_some() && y.is_some() {
            Some((x.unwrap(), y.unwrap()))
        } else {
            None
        }
    })
    .collect()
}

fn find_connected_tiles<'a>(
    tiles: &'a TileSet,
    tile: &Tile,
    coordinates: &Vec<(usize, usize)>,
) -> Vec<&'a Tile> {
    let mut surrounding_tiles: Vec<&Tile> = vec![];

    for (x, y) in coordinates {
        if let Some(found_tile) = tiles.get((*y, *x)) {
            surrounding_tiles.push(found_tile);
        }
    }

    surrounding_tiles
        .into_iter()
        .filter(|surrounding_tile| tile.connected_to(surrounding_tile))
        .collect()
}

fn process_input(input: &str) -> TileSet {
    let lines: Vec<Vec<Tile>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, tile_data)| Tile::new(tile_data, (x, y)))
                .collect()
        })
        .collect();

    let rows = lines.len();
    let columns = lines[0].len();

    let flattened_tiles: Vec<Tile> = lines.iter().flatten().cloned().collect();
    Array2::from_shape_vec((rows, columns), flattened_tiles).unwrap()
}

#[derive(EnumString, PartialEq, Clone, strum::Display, Debug)]
enum TileType {
    #[strum(serialize = ".")]
    Ground,
    #[strum(serialize = "|")]
    VerticalPipe,
    #[strum(serialize = "-")]
    HorizontalPipe,
    #[strum(serialize = "L")]
    NorthEastPipe,
    #[strum(serialize = "J")]
    NorthWestPipe,
    #[strum(serialize = "7")]
    SouthWestPipe,
    #[strum(serialize = "F")]
    SouthEastPipe,
    #[strum(serialize = "S")]
    Start,
}

#[derive(Clone, Debug)]
struct Tile {
    r#type: TileType,
    coordinates: (usize, usize),
}

impl Tile {
    fn new(tile_data: char, coordinates: (usize, usize)) -> Self {
        let tile_type = TileType::from_str(tile_data.to_string().as_str()).unwrap();

        Self {
            r#type: tile_type,
            coordinates,
        }
    }

    fn connected_coordinates(&self) -> Vec<(usize, usize)> {
        // I should technically do checked math to prevent an overflow
        // but AoC inputs should be trusted to be correct and not lead
        // me past the bounds (assuming my solution works properly)
        match self.r#type {
            TileType::VerticalPipe => vec![
                (self.coordinates.0, self.coordinates.1 + 1),
                (self.coordinates.0, self.coordinates.1 - 1),
            ],
            TileType::HorizontalPipe => vec![
                (self.coordinates.0 + 1, self.coordinates.1),
                (self.coordinates.0 - 1, self.coordinates.1),
            ],
            TileType::NorthEastPipe => vec![
                (self.coordinates.0, self.coordinates.1 - 1),
                (self.coordinates.0 + 1, self.coordinates.1),
            ],
            TileType::NorthWestPipe => vec![
                (self.coordinates.0, self.coordinates.1 - 1),
                (self.coordinates.0 - 1, self.coordinates.1),
            ],
            TileType::SouthWestPipe => vec![
                (self.coordinates.0, self.coordinates.1 + 1),
                (self.coordinates.0 - 1, self.coordinates.1),
            ],
            TileType::SouthEastPipe => vec![
                (self.coordinates.0, self.coordinates.1 + 1),
                (self.coordinates.0 + 1, self.coordinates.1),
            ],
            _ => panic!("Did not expect to run coordinates on a type without them"),
        }
    }

    fn connected_to(&self, other: &Tile) -> bool {
        if self.coordinates.0 < other.coordinates.0 {
            [
                TileType::HorizontalPipe,
                TileType::SouthWestPipe,
                TileType::NorthWestPipe,
            ]
            .contains(&other.r#type)
        } else if self.coordinates.0 > other.coordinates.0 {
            [
                TileType::HorizontalPipe,
                TileType::SouthEastPipe,
                TileType::NorthEastPipe,
            ]
            .contains(&other.r#type)
        } else if self.coordinates.1 < other.coordinates.1 {
            [
                TileType::VerticalPipe,
                TileType::NorthEastPipe,
                TileType::NorthWestPipe,
            ]
            .contains(&other.r#type)
        } else if self.coordinates.1 > other.coordinates.1 {
            [
                TileType::VerticalPipe,
                TileType::SouthEastPipe,
                TileType::SouthWestPipe,
            ]
            .contains(&other.r#type)
        } else {
            panic!("I got the coordinate checking wrong");
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    fn simple_test_data() -> TileSet {
        let simple_input = include_str!("../data/simple_test_input.txt");

        process_input(simple_input)
    }

    fn complex_test_data() -> TileSet {
        let complex_input = include_str!("../data/complex_test_input.txt");

        process_input(complex_input)
    }

    #[test]
    fn test_find_start_tile() {
        let simple_data = simple_test_data();
        let start_tile = find_start_tile(&simple_data);

        assert_eq!(start_tile.coordinates, (1, 1));

        let complex_data = complex_test_data();
        let start_tile = find_start_tile(&complex_data);

        assert_eq!(start_tile.coordinates, (0, 2));
    }

    #[test]
    fn test_find_surrounding_coordinates() {
        let simple_data = simple_test_data();
        let start_tile = find_start_tile(&simple_data);
        let surrounding_coords = find_surrounding_coordinates(start_tile);
        assert_eq!(surrounding_coords, vec![(1, 0), (2, 1), (1, 2), (0, 1)]);

        let complex_data = complex_test_data();
        let start_tile = find_start_tile(&complex_data);
        let surrounding_coords = find_surrounding_coordinates(start_tile);
        assert_eq!(surrounding_coords, vec![(0, 1), (1, 2), (0, 3)])
    }

    #[test]
    fn test_find_connected_tiles() {
        let simple_data = simple_test_data();
        let start_tile = find_start_tile(&simple_data);
        let surrounding_coords = find_surrounding_coordinates(start_tile);
        let connected_tiles = find_connected_tiles(&simple_data, start_tile, &surrounding_coords);

        assert_eq!(connected_tiles.len(), 2);
        assert_eq!(connected_tiles[0].coordinates, (2, 1));
        assert_eq!(connected_tiles[1].coordinates, (1, 2));

        let complex_data = complex_test_data();
        let start_tile = find_start_tile(&complex_data);
        let surrounding_coords = find_surrounding_coordinates(start_tile);
        let connected_tiles = find_connected_tiles(&complex_data, start_tile, &surrounding_coords);

        assert_eq!(connected_tiles.len(), 2);
        assert_eq!(connected_tiles[0].coordinates, (1, 2));
        assert_eq!(connected_tiles[1].coordinates, (0, 3));
    }

    #[test]
    fn test_connected() {
        let first_tile = Tile::new('F', (1, 1));
        let connected_tile = Tile::new('-', (2, 1));

        assert!(first_tile.connected_to(&connected_tile));

        let unconnected_tile = Tile::new('|', (2, 1));
        assert!(!first_tile.connected_to(&unconnected_tile));

        let another_start = Tile::new('S', (0, 2));
        let another_connected = Tile::new('J', (1, 2));

        assert!(another_start.connected_to(&another_connected));
    }

    #[test]
    fn test_part_one_test_solution() {
        let simple_data = simple_test_data();

        assert_eq!(part_one_solution(&simple_data), 4);

        let complex_data = complex_test_data();

        assert_eq!(part_one_solution(&complex_data), 8);
    }

    #[test]
    fn test_part_one_actual_solution() {
        let data = process_input(include_str!("../data/puzzle_input.txt"));
        let part_one_answer = part_one_solution(&data);

        assert_eq!(part_one_answer, 6870);
    }
}
