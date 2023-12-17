fn main() {
    let input = include_str!("../data/puzzle_input.txt");
    let galaxy_map = process_input(input);
    let expanded_galaxy = expand(galaxy_map);

    let part_one_answer = part_one_solution(&expanded_galaxy);
    println!("Part One Answer is {part_one_answer}");
}
fn part_one_solution(galaxy_map: &[Vec<char>]) -> usize {
    let galaxy_pairs = generate_pairs(galaxy_map);

    galaxy_pairs
        .iter()
        .fold(0, |acc, pair| acc + calculate_distance(&pair.0, &pair.1))
}

fn process_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn expand(galaxy_map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut row_expanded: Vec<Vec<char>> = vec![];

    for row in &galaxy_map {
        row_expanded.push(row.to_vec());

        if row.iter().all(|pos| pos == &'.') {
            row_expanded.push(row.to_vec());
        }
    }

    let mut final_expanded: Vec<Vec<char>> = vec![];

    for row in &row_expanded {
        let mut new_row: Vec<char> = vec![];

        for (col_idx, col) in row.iter().enumerate() {
            let col_data: Vec<char> = row_expanded.iter().map(|row| row[col_idx]).collect();

            new_row.push(*col);

            if col_data.iter().all(|char| char == &'.') {
                new_row.push('.');
            }
        }

        final_expanded.push(new_row);
    }

    final_expanded
}

type Coordinate = (usize, usize);

fn generate_pairs(galaxy_map: &[Vec<char>]) -> Vec<(Coordinate, Coordinate)> {
    let mut galaxy_pairs: Vec<(Coordinate, Coordinate)> = vec![];

    let mut galaxies: Vec<Coordinate> = vec![];

    for (y_index, row) in galaxy_map.iter().enumerate() {
        for (x_index, char) in row.iter().enumerate() {
            if char == &'#' {
                galaxies.push((x_index, y_index));
            }
        }
    }

    for (idx, galaxy) in galaxies.iter().enumerate() {
        for other in galaxies.iter().skip(idx + 1) {
            galaxy_pairs.push((*galaxy, *other));
        }
    }

    galaxy_pairs
}

fn calculate_distance(first: &Coordinate, other: &Coordinate) -> usize {
    let x_diff = first.0.abs_diff(other.0);
    let y_diff = first.1.abs_diff(other.1);

    x_diff + y_diff
}

#[cfg(test)]
mod test_super {
    use super::*;

    fn test_data() -> Vec<Vec<char>> {
        let test_input = include_str!("../data/test_input.txt");

        process_input(test_input)
    }

    #[test]
    fn test_process_input() {
        let galaxy_map = test_data();

        assert_eq!(galaxy_map.len(), 10);

        let expected_first_line = vec!['.', '.', '.', '#', '.', '.', '.', '.', '.', '.'];
        assert_eq!(galaxy_map[0], expected_first_line);
    }

    #[test]
    fn test_expand() {
        let galaxy_map = test_data();
        let expanded_map = expand(galaxy_map);

        // rows
        assert_eq!(expanded_map.len(), 12);
        assert_eq!(
            expanded_map[0],
            vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.', '.', '.', '.']
        );

        assert_eq!(
            expanded_map[1],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '.', '.',]
        );

        // cols
        assert_eq!(expanded_map[0].len(), 13);
    }

    #[test]
    fn test_generate_pairs() {
        let galaxy_map = test_data();
        let expanded_map = expand(galaxy_map);
        let pairs = generate_pairs(&expanded_map);

        assert_eq!(pairs.len(), 36);

        assert_eq!(pairs[0], ((4, 0), (9, 1)));
        assert_eq!(pairs[35], ((0, 11), (5, 11)));
    }

    #[test]
    fn test_calculate_distance() {
        let first_galaxy = (4, 0);
        let second_galaxy = (9, 1);
        let third_galaxy = (0, 2);
        let sixth_galaxy = (12, 7);
        let seventh_galaxy = (9, 10);
        let eighth_galaxy = (0, 11);
        let ninth_galaxy = (5, 11);

        assert_eq!(calculate_distance(&first_galaxy, &seventh_galaxy), 15);
        assert_eq!(calculate_distance(&third_galaxy, &sixth_galaxy), 17);
        assert_eq!(calculate_distance(&eighth_galaxy, &ninth_galaxy), 5);

        assert_eq!(calculate_distance(&second_galaxy, &third_galaxy), 10);
    }

    #[test]
    fn test_part_one_example() {
        let galaxy_map = test_data();
        let expanded_map = expand(galaxy_map);

        let part_one_example = part_one_solution(&expanded_map);
        assert_eq!(part_one_example, 374);
    }

    #[test]
    fn test_part_one_solution() {
        let galaxy_map = process_input(include_str!("../data/puzzle_input.txt"));
        let expanded_map = expand(galaxy_map);

        let part_one_example = part_one_solution(&expanded_map);
        assert_eq!(part_one_example, 9370588);
    }
}
