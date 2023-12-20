// TODO: Fix tilt_column method signature
use ndarray::{Array2, ArrayBase, Dim, ViewRepr};

const ROUNDED_ROCK: char = 'O';
const CUBE_ROCK: char = '#';

fn main() {
    let input = include_str!("../data/puzzle_input.txt");
    let platform = process_input(input);

    let part_one_answer = part_one_solution(&platform);
    println!("Part one answer is: {part_one_answer}");
}

fn part_one_solution(platform: &Array2<char>) -> u32 {
    // for each column move all O to the top of the array
    // any O that exist after # be moved into position behind them
    // prev points should be "swapped" or replaced with .
    platform.columns().into_iter().fold(0, |acc, column| {
        let tilted_column = tilt_column(&column);
        let load = calculate_column_load(&tilted_column);

        acc + load
    })
}

fn tilt_column(column: &ArrayBase<ViewRepr<&char>, Dim<[usize; 1]>>) -> Vec<char> {
    let mut tilted_column: Vec<char> = column.to_vec();
    let mut next_spot = 0;

    for i in 0..tilted_column.len() {
        let current_spot = tilted_column[i];
        if current_spot == ROUNDED_ROCK {
            tilted_column.swap(next_spot, i);
            next_spot += 1;
        } else if current_spot == CUBE_ROCK {
            next_spot = i + 1;
        }
    }

    tilted_column
}

fn calculate_column_load(column: &[char]) -> u32 {
    column.iter().rev().enumerate().fold(0, |acc, (i, col)| {
        if col == &ROUNDED_ROCK {
            acc + (i + 1) as u32
        } else {
            acc
        }
    })
}

fn process_input(input: &str) -> Array2<char> {
    let positions: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let row_length = positions[0].len();
    let col_length = positions.len();

    Array2::from_shape_vec(
        (row_length, col_length),
        positions.iter().flatten().cloned().collect(),
    )
    .unwrap()
}

#[cfg(test)]
mod test_super {
    use ndarray::Axis;

    use super::*;

    fn test_data() -> Array2<char> {
        process_input(include_str!("../data/test_input.txt"))
    }

    #[test]
    fn test_process_input() {
        let test_platform = test_data();

        assert_eq!(test_platform.len_of(Axis(1)), 10);
        assert_eq!(
            test_platform.column(0).to_vec(),
            "OO.O.O..##".chars().collect::<Vec<char>>()
        )
    }

    #[test]
    fn test_tilt_column() {
        let test_platform = test_data();

        let tilted_column = tilt_column(&test_platform.column(0));
        assert_eq!(tilted_column, "OOOO....##".chars().collect::<Vec<char>>());

        let tilted_column = tilt_column(&test_platform.column(2));
        assert_eq!(tilted_column, "O....#OO..".chars().collect::<Vec<char>>())
    }

    #[test]
    fn test_calculate_column_load() {
        assert_eq!(
            calculate_column_load(&"OOOO....##".chars().collect::<Vec<char>>()),
            34
        );

        assert_eq!(
            calculate_column_load(&"O....#OO..".chars().collect::<Vec<char>>()),
            17
        )
    }

    #[test]
    fn test_part_one_example() {
        let test_platform = test_data();

        let part_one_example = part_one_solution(&test_platform);

        assert_eq!(part_one_example, 136);
    }

    #[test]
    fn test_part_one_answer() {
        let platform = process_input(include_str!("../data/puzzle_input.txt"));

        let part_one_answer = part_one_solution(&platform);

        assert_eq!(part_one_answer, 108826);
    }
}
