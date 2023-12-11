fn main() {
    let input = read_input();
    let readings = process_input(&input);

    let part_one_answer = part_one_solution(&readings);
    println!("Part One answer is {part_one_answer}");
}

fn part_one_solution(readings: &[Vec<i32>]) -> i32 {
    readings.iter().fold(0, |acc, reading| {
        let next_sequences = calculate_next_sequences(reading);
        acc + next_sequences.last().unwrap().last().unwrap()
    })
}

fn calculate_next_sequences(reading: &[i32]) -> Vec<Vec<i32>> {
    let mut diffs = generate_diffs(reading);
    diffs.reverse();

    for num in 0..(diffs.len() - 1) {
        let first = *diffs[num].last().unwrap();
        let last = &mut diffs[num + 1];

        let next_sequence = first + last.last().unwrap();
        last.push(next_sequence);
    }

    diffs
}

fn generate_diffs(reading: &[i32]) -> Vec<Vec<i32>> {
    let mut diffs: Vec<Vec<i32>> = vec![reading.to_vec()];

    while !diffs.last().unwrap().iter().all(|num| num == &0) {
        let diff = generate_differential_array(diffs.last().unwrap());

        diffs.push(diff);
    }

    diffs
}

fn generate_differential_array(reading: &[i32]) -> Vec<i32> {
    let mut diff: Vec<i32> = vec![];

    for pos in 0..(reading.len() - 1) {
        let first = reading[pos];
        let next = reading[pos + 1];

        // todo: how to handle if the diff is a negative number??
        diff.push(next - first);
    }

    diff
}

fn read_input() -> String {
    include_str!("../data/puzzle_input.txt").to_string()
}

fn process_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|reading| reading.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod test_super {
    use super::*;

    fn test_data() -> Vec<Vec<i32>> {
        process_input(include_str!("../data/test_input.txt"))
    }

    #[test]
    fn test_process_input() {
        let readings = test_data();

        assert_eq!(readings.len(), 3);
        assert_eq!(readings[0], vec![0, 3, 6, 9, 12, 15]);
    }

    #[test]
    fn test_generate_differential_array() {
        let diff_array = generate_differential_array(&[0, 3, 6, 9, 12, 15]);

        assert_eq!(diff_array, vec![3, 3, 3, 3, 3]);
    }

    #[test]
    fn test_generate_diffs() {
        let diffs = generate_diffs(&[0, 3, 6, 9, 12, 15]);

        assert_eq!(diffs.len(), 3);
        assert_eq!(
            diffs,
            vec![
                vec![0, 3, 6, 9, 12, 15,],
                vec![3, 3, 3, 3, 3],
                vec![0, 0, 0, 0,]
            ]
        );
    }

    #[test]
    fn test_calculate_next_sequences() {
        let next_sequences = calculate_next_sequences(&[0, 3, 6, 9, 12, 15]);

        assert_eq!(
            next_sequences,
            vec![
                vec![0, 0, 0, 0,],
                vec![3, 3, 3, 3, 3, 3],
                vec![0, 3, 6, 9, 12, 15, 18],
            ]
        )
    }

    #[test]
    fn test_part_one_example() {
        let readings = test_data();
        let part_one_answer = part_one_solution(&readings);

        assert_eq!(part_one_answer, 114);
    }

    #[test]
    fn test_part_one_actual() {
        let readings = process_input(include_str!("../data/puzzle_input.txt"));
        let part_one_answer = part_one_solution(&readings);

        assert_eq!(part_one_answer, 2008960228);
    }
}
