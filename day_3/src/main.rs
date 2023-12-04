use ndarray::Array2;

fn main() {
    let input_data = read_input();
    let grid = process_input(&input_data);

    let part_one_answer = part_one_solution(&grid);
    println!("Part One Solution is {part_one_answer}")
}

fn part_one_solution(grid: &Array2<char>) -> u32 {
    let part_numbers = find_parts(grid);

    part_numbers.iter().sum()
}

fn find_parts(grid: &Array2<char>) -> Vec<u32> {
    let mut part_numbers: Vec<u32> = vec![];
    let mut current_num: Vec<&char> = vec![];
    let mut current_num_good = false;
    for ((row, col), position) in grid.indexed_iter() {
        if position.is_ascii_digit() {
            current_num.push(position);

            let surrounding = surrounding_cells(grid, row, col);
            if contain_symbol(&surrounding) {
                current_num_good = true;
            }

            if number_finished(grid, row, col) {
                if current_num_good {
                    let boop: String = current_num.iter().cloned().collect();
                    part_numbers.push(boop.parse::<u32>().unwrap());
                }
                current_num = vec![];
                current_num_good = false;
            }
        }
    }

    part_numbers
}

fn surrounding_cells(grid: &Array2<char>, row: usize, col: usize) -> Vec<Option<&char>> {
    let coordinates = vec![
        // row above
        (row.checked_sub(1), col.checked_sub(1)),
        (row.checked_sub(1), Some(col)),
        (row.checked_sub(1), col.checked_add(1)),
        // current row
        (Some(row), col.checked_sub(1)),
        (Some(row), col.checked_add(1)),
        // row below
        (row.checked_add(1), col.checked_sub(1)),
        (row.checked_add(1), Some(col)),
        (row.checked_add(1), col.checked_add(1)),
    ];

    coordinates
        .into_iter()
        .filter(|(row, col)| row.is_some() && col.is_some())
        .map(|(row, col)| (row.unwrap(), col.unwrap()))
        .map(|coords| grid.get(coords))
        .collect()
}

fn contain_symbol(chars: &[Option<&char>]) -> bool {
    chars.iter().any(|char| match char {
        Some(char) => !char.is_ascii_digit() && char != &&'.',
        None => false,
    })
}

fn number_finished(grid: &Array2<char>, row: usize, col: usize) -> bool {
    match grid.get((row, col + 1)) {
        Some(x) => !x.is_ascii_digit(),
        None => true,
    }
}

fn process_input(input: &str) -> Array2<char> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let rows = grid.len();
    let columns = grid[0].len(); // Should be fine since the input is in a consistent shape

    let flattened_grid: Vec<char> = grid.iter().flatten().cloned().collect();
    Array2::from_shape_vec((rows, columns), flattened_grid).unwrap()
}

fn read_input() -> String {
    String::from(include_str!("../data/puzzle_input.txt"))
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_find_parts() {
        let grid = process_input(include_str!("../data/test_input.txt"));
        assert_eq!(
            find_parts(&grid),
            vec![467, 35, 633, 617, 592, 755, 664, 598]
        );
    }

    #[test]
    fn test_part_one_solution_test() {
        let grid: ndarray::prelude::ArrayBase<
            ndarray::OwnedRepr<char>,
            ndarray::prelude::Dim<[usize; 2]>,
        > = process_input(include_str!("../data/test_input.txt"));
        assert_eq!(part_one_solution(&grid), 4361);
    }

    #[test]
    fn test_part_one_solution_actual() {
        let grid = process_input(include_str!("../data/puzzle_input.txt"));
        assert_eq!(part_one_solution(&grid), 557705);
    }
}
