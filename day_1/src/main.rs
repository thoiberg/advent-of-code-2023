fn main() {
    let calibrations = process_input();

    let part_one_answer = part_one_solution(&calibrations);
    println!("Part One Answer is: {part_one_answer}");
}

fn part_one_solution(calibrations: &[String]) -> u32 {
    calibrations
        .iter()
        .fold(0, |acc: u32, x| acc + first_and_last_digit(x).unwrap())
}

fn process_input() -> Vec<String> {
    include_str!("./puzzle_input.txt")
        .split('\n')
        .map(String::from)
        .collect()
}

fn first_and_last_digit(text: &str) -> Option<u32> {
    let x = first_digit(text)?;

    let reversed_string: String = text.chars().rev().collect();
    let last_digit = first_digit(&reversed_string)?;

    Some((x * 10) + last_digit)
}

fn first_digit(text: &str) -> Option<u32> {
    let digit = text.chars().find(|char| char.is_numeric())?;

    digit.to_digit(10)
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_first_digit() {
        assert_eq!(first_digit("1abc2"), Some(1));
        assert_eq!(first_digit("pqr3stu8vwx"), Some(3));
        assert_eq!(first_digit("a1b2c3d4e5f"), Some(1));
        assert_eq!(first_digit("treb7uchet"), Some(7));
    }

    #[test]
    fn test_first_and_last_digit() {
        assert_eq!(first_and_last_digit("1abc2"), Some(12));
        assert_eq!(first_and_last_digit("pqr3stu8vwx"), Some(38));
        assert_eq!(first_and_last_digit("a1b2c3d4e5f"), Some(15));
        assert_eq!(first_and_last_digit("treb7uchet"), Some(77));
    }

    #[test]
    fn test_part_one_solution() {
        let calibrations = vec![
            String::from("1abc2"),
            String::from("pqr3stu8vwx"),
            String::from("a1b2c3d4e5f"),
            String::from("treb7uchet"),
        ];

        assert_eq!(part_one_solution(&calibrations), 142);
    }
}
