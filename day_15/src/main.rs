fn main() {
    let input = include_str!("../data/puzzle_input.txt");
    let init_sequence = process_input(input);

    let part_one_answer = part_one_solution(&init_sequence);
    println!("Part one answer is {part_one_answer}");
}

fn part_one_solution(sequence: &[&str]) -> u32 {
    sequence
        .iter()
        .fold(0, |acc, step| acc + calculate_hash(step))
}

fn calculate_hash(step: &str) -> u32 {
    step.chars().fold(0, |mut acc, char| {
        let ascii_code = char as u32;
        acc += ascii_code;
        acc *= 17;
        acc %= 256;

        acc
    })
}

fn process_input(input: &str) -> Vec<&str> {
    input.split(',').collect()
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_calculate_hash() {
        assert_eq!(calculate_hash("HASH"), 52);
        assert_eq!(calculate_hash("rn=1"), 30);
        assert_eq!(calculate_hash("pc=4"), 180);
        assert_eq!(calculate_hash("ot=7"), 231);
    }

    #[test]
    fn test_part_one_example() {
        let test_data = process_input("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");

        let part_one_example = part_one_solution(&test_data);

        assert_eq!(part_one_example, 1_320);
    }

    #[test]
    fn test_part_one_solution() {
        let data = process_input(include_str!("../data/puzzle_input.txt"));

        let part_one_solution = part_one_solution(&data);

        assert_eq!(part_one_solution, 515_974);
    }
}
