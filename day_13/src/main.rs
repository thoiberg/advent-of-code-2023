// TODO: pull generic logic out of find_reflection_point
use ndarray::{Array2, Axis};

fn main() {
    let input = include_str!("../data/puzzle_input.txt");

    let terrains = process_input(input);

    let part_one_answer = part_one_solution(&terrains);
    println!("Part One answer is {part_one_answer}");
}

#[derive(PartialEq, Debug)]
enum ReflectionType {
    Vertical,
    Horizontal,
}

fn part_one_solution(terrains: &[Array2<char>]) -> u64 {
    terrains.iter().fold(0, |acc, terrain| {
        let (reflection_point, reflection_type) = find_reflection_point(terrain);

        acc + (match reflection_type {
            ReflectionType::Vertical => reflection_point,
            ReflectionType::Horizontal => reflection_point * 100,
        } as u64)
    })
}

fn find_reflection_point(terrain: &Array2<char>) -> (u32, ReflectionType) {
    // vertical
    let column_length = terrain.len_of(Axis(1));
    for y_idx in 0..(column_length - 1) {
        let mut valid = true;
        let mut less_step = 0;
        let mut more_step = 1;

        while valid {
            if y_idx.checked_sub(less_step).is_none() || (y_idx + more_step) >= column_length {
                break;
            }

            let left = terrain.column(y_idx - less_step);
            let right = terrain.column(y_idx + more_step);

            valid = left == right;
            less_step += 1;
            more_step += 1;
        }

        if valid {
            return ((y_idx + 1) as u32, ReflectionType::Vertical);
        }
    }

    // horizontal
    let row_amount = terrain.len_of(Axis(0));
    for x_idx in 0..(row_amount - 1) {
        let mut valid = true;
        let mut less_step = 0;
        let mut more_step = 1;

        while valid {
            if x_idx.checked_sub(less_step).is_none() || (x_idx + more_step) >= row_amount {
                break;
            }

            let left = terrain.row(x_idx - less_step);
            let right = terrain.row(x_idx + more_step);

            valid = left == right;
            less_step += 1;
            more_step += 1;
        }

        if valid {
            return ((x_idx + 1) as u32, ReflectionType::Horizontal);
        }
    }

    panic!("could not find reflection point")
}

fn process_input(input: &str) -> Vec<Array2<char>> {
    let terrains: Vec<&str> = input.split("\n\n").collect();

    terrains
        .into_iter()
        .map(|terrain| {
            let terrain_data: Vec<Vec<char>> =
                terrain.lines().map(|line| line.chars().collect()).collect();
            let rows = terrain_data.len();
            let cols = terrain_data[0].len();

            let flattened_terrain = terrain_data.iter().flatten().cloned().collect();
            Array2::from_shape_vec((rows, cols), flattened_terrain).unwrap()
        })
        .collect()
}

#[cfg(test)]
mod test_super {
    use super::*;

    fn test_data() -> Vec<Array2<char>> {
        let test_input = include_str!("../data/test_input.txt");

        process_input(test_input)
    }

    #[test]
    fn test_process_input() {
        let test_terrains = test_data();

        assert_eq!(test_terrains.len(), 2);

        let terrain = &test_terrains[0];
        assert_eq!(terrain.row(0).len(), 9);
        assert_eq!(terrain.column(0).len(), 7);

        assert_eq!(
            &test_terrains[0].row(0).to_vec(),
            &vec!['#', '.', '#', '#', '.', '.', '#', '#', '.']
        );
    }

    #[test]
    fn test_find_reflection_point() {
        let test_terrains = test_data();

        let (point, reflection_type) = find_reflection_point(&test_terrains[0]);
        assert_eq!(point, 5);
        assert_eq!(reflection_type, ReflectionType::Vertical);

        let (point, reflection_type) = find_reflection_point(&test_terrains[1]);
        assert_eq!(point, 4);
        assert_eq!(reflection_type, ReflectionType::Horizontal);
    }

    #[test]
    fn test_part_one_example() {
        let test_terrains = test_data();
        let part_one_example = part_one_solution(&test_terrains);

        assert_eq!(part_one_example, 405);
    }

    #[test]
    fn test_part_one_answer() {
        let terrains = process_input(include_str!("../data/puzzle_input.txt"));

        let part_one_answer = part_one_solution(&terrains);

        assert_eq!(part_one_answer, 33735);
    }
}
