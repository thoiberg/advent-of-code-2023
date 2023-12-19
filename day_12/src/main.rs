// TODO: Optimise Part One. Currently takes ~10s to complete with
//       the brute force approach.

use std::str::FromStr;

fn main() {
    let input = include_str!("../data/puzzle_input.txt");
    let rows = process_input(input);

    let part_one_answer = part_one_solution(&rows);
    println!("Part One Answer is {part_one_answer}");
}

fn part_one_solution(rows: &[Row]) -> u64 {
    let mut valid_count = 0;

    for row in rows {
        let permutations = generate_all_permutations(row);
        for permutation in &permutations {
            if is_valid(permutation, &row.damaged_spring_grouping) {
                valid_count += 1;
            }
        }
    }

    valid_count
}

fn generate_all_permutations(row: &Row) -> Vec<Vec<SpringCondition>> {
    generate_permutation(row.springs.clone(), 0)
}

fn generate_permutation(
    spring_conditions: Vec<SpringCondition>,
    idx: usize,
) -> Vec<Vec<SpringCondition>> {
    if idx >= spring_conditions.len() {
        return vec![spring_conditions];
    }

    for pos in idx..spring_conditions.len() {
        if spring_conditions[pos] == SpringCondition::Unknown {
            let mut operational = spring_conditions.clone();
            let mut damaged = spring_conditions.clone();
            operational[pos] = SpringCondition::Operational;
            damaged[pos] = SpringCondition::Damaged;

            let mut operational = generate_permutation(operational, idx + 1);
            let mut damaged = generate_permutation(damaged, idx + 1);

            operational.append(&mut damaged);

            return operational;
        }
    }

    vec![spring_conditions]
}

fn is_valid(permutation: &Vec<SpringCondition>, count_grouping: &Vec<u32>) -> bool {
    let mut total: Vec<u32> = vec![];

    let mut current = 0;

    for position in permutation {
        if position == &SpringCondition::Operational {
            if current > 0 {
                total.push(current);
                current = 0;
            }
            continue;
        } else {
            current += 1;
        }
    }

    if current > 0 {
        total.push(current)
    }

    &total == count_grouping
}

#[derive(strum::EnumString, PartialEq, Debug, Clone)]
enum SpringCondition {
    #[strum(serialize = ".")]
    Operational,
    #[strum(serialize = "#")]
    Damaged,
    #[strum(serialize = "?")]
    Unknown,
}

struct Row {
    springs: Vec<SpringCondition>,
    damaged_spring_grouping: Vec<u32>,
}

fn process_input(input: &str) -> Vec<Row> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(' ').collect();
            let springs: Vec<SpringCondition> = parts[0]
                .chars()
                .map(|char| SpringCondition::from_str(char.to_string().as_ref()).unwrap())
                .collect();

            let grouping = parts[1]
                .split(',')
                .map(|group_number| group_number.parse::<u32>().unwrap())
                .collect();

            Row {
                springs,
                damaged_spring_grouping: grouping,
            }
        })
        .collect()
}

#[cfg(test)]
mod test_super {
    use super::*;

    fn test_data() -> Vec<Row> {
        let test_data = include_str!("../data/test_input.txt");

        process_input(test_data)
    }

    #[test]
    fn test_process_input() {
        let rows = test_data();

        assert_eq!(rows.len(), 6);
        assert_eq!(rows[0].damaged_spring_grouping, vec![1, 1, 3]);
        assert_eq!(
            rows[0].springs,
            vec![
                SpringCondition::Unknown,
                SpringCondition::Unknown,
                SpringCondition::Unknown,
                SpringCondition::Operational,
                SpringCondition::Damaged,
                SpringCondition::Damaged,
                SpringCondition::Damaged
            ]
        );
    }

    #[test]
    fn test_generate_all_permutations() {
        let simple_row = Row {
            springs: vec![
                SpringCondition::Unknown,
                SpringCondition::Unknown,
                SpringCondition::Unknown,
                SpringCondition::Operational,
                SpringCondition::Damaged,
                SpringCondition::Damaged,
                SpringCondition::Damaged,
            ],
            damaged_spring_grouping: vec![1, 1, 3],
        };
        let complex_row = Row {
            springs: vec![
                SpringCondition::Unknown,
                SpringCondition::Damaged,
                SpringCondition::Damaged,
                SpringCondition::Unknown,
                SpringCondition::Unknown,
                SpringCondition::Unknown,
                SpringCondition::Unknown,
                SpringCondition::Unknown,
                SpringCondition::Unknown,
                SpringCondition::Unknown,
                SpringCondition::Unknown,
            ],
            damaged_spring_grouping: vec![3, 2, 1],
        };

        let simple_permutations = generate_all_permutations(&simple_row);
        assert_eq!(simple_permutations.len(), 8);

        let complex_permutations = generate_all_permutations(&complex_row);
        assert_eq!(complex_permutations.len(), 512);
    }

    #[test]
    fn test_is_valid() {
        let valid_permutation = vec![
            SpringCondition::Damaged,
            SpringCondition::Operational,
            SpringCondition::Damaged,
            SpringCondition::Operational,
            SpringCondition::Damaged,
            SpringCondition::Damaged,
            SpringCondition::Damaged,
        ];
        let count_grouping = vec![1, 1, 3];
        let validity = is_valid(&valid_permutation, &count_grouping);

        assert!(validity);

        let invalid_permutation = vec![
            SpringCondition::Operational,
            SpringCondition::Operational,
            SpringCondition::Damaged,
            SpringCondition::Operational,
            SpringCondition::Damaged,
            SpringCondition::Damaged,
            SpringCondition::Damaged,
        ];
        let validity = is_valid(&invalid_permutation, &count_grouping);

        assert!(!validity);
    }

    #[test]
    fn test_part_one_example() {
        let test_data = test_data();

        assert_eq!(part_one_solution(&test_data), 21);
    }

    // Too slow to run in CI (~10s)
    // #[test]
    // fn test_part_one_solution() {
    //     let puzzle_data = process_input(include_str!("../data/puzzle_input.txt"));

    //     assert_eq!(part_one_solution(&puzzle_data), 8419);
    // }
}
