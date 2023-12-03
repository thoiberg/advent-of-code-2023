fn main() {
    let calibrations = process_input();

    let part_one_answer = part_one_solution(&calibrations);
    println!("Part One Answer is: {part_one_answer}");

    let part_two_answer = part_two_solution(&calibrations);
    println!("Part Two Answer is: {part_two_answer}")
}

fn part_one_solution(calibrations: &[String]) -> u32 {
    calibrations
        .iter()
        .fold(0, |acc: u32, x| acc + calibration_total(x))
}

fn part_two_solution(calibrations: &[String]) -> u32 {
    calibrations
        .iter()
        .fold(0, |acc, x| acc + part_two_calibration_total(x))
}

fn process_input() -> Vec<String> {
    include_str!("./puzzle_input.txt")
        .split('\n')
        .map(String::from)
        .collect()
}

fn calibration_total(text: &str) -> u32 {
    match first_and_last_digit(text) {
        Some((first, last)) => (first * 10) + last,
        None => panic!("Could not find first or last in {text}"),
    }
}

fn first_and_last_digit(text: &str) -> Option<(u32, u32)> {
    let first_number = first_digit(text)?;

    let reversed_string: String = text.chars().rev().collect();
    let last_number = first_digit(&reversed_string)?;

    Some((first_number, last_number))
}

fn first_digit(text: &str) -> Option<u32> {
    let digit = text.chars().find(|char| char.is_numeric())?;

    digit.to_digit(10)
}

fn part_two_calibration_total(text: &str) -> u32 {
    match part_two_first_and_last_digit(text) {
        Some((first, last)) => (first * 10) + last,
        None => panic!("Could not find first or last in {text}"),
    }
}

fn part_two_first_and_last_digit(text: &str) -> Option<(u32, u32)> {
    let first_number = part_two_first_digit(text)?;

    let reversed_string: String = text.chars().rev().collect();
    let last_number = part_two_first_digit(&reversed_string)?;

    Some((first_number, last_number))
}

fn part_two_first_digit(text: &str) -> Option<u32> {
    let mut first_dig: Option<u32> = None;
    let char_array: Vec<char> = text.chars().collect();

    for (idx, char) in text.chars().enumerate() {
        if char.is_numeric() {
            // todo check if this works before finishing the loop
            first_dig = char.to_digit(10);
            break;
        }

        let boop: &String = &char_array[idx..].iter().collect();
        let spelt_digit = word_to_digit(boop);

        if spelt_digit.is_some() {
            first_dig = spelt_digit;
            break;
        }
    }

    first_dig
}

fn word_to_digit(word: &str) -> Option<u32> {
    if word.starts_with("one") || word.starts_with("eno") {
        Some(1)
    } else if word.starts_with("two") || word.starts_with("owt") {
        Some(2)
    } else if word.starts_with("three") || word.starts_with("eerht") {
        Some(3)
    } else if word.starts_with("four") || word.starts_with("ruof") {
        Some(4)
    } else if word.starts_with("five") || word.starts_with("evif") {
        Some(5)
    } else if word.starts_with("six") || word.starts_with("xis") {
        Some(6)
    } else if word.starts_with("seven") || word.starts_with("neves") {
        Some(7)
    } else if word.starts_with("eight") || word.starts_with("thgie") {
        Some(8)
    } else if word.starts_with("nine") || word.starts_with("enin") {
        Some(9)
    } else {
        None
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_part_one_answer() {
        let calibrations = process_input();
        assert_eq!(part_one_solution(&calibrations), 54561);
    }

    #[test]
    fn test_part_two_answer() {
        let calibrations = process_input();
        assert_eq!(part_two_solution(&calibrations), 54076);
    }

    #[test]
    fn test_first_digit() {
        assert_eq!(first_digit("1abc2"), Some(1));
        assert_eq!(first_digit("pqr3stu8vwx"), Some(3));
        assert_eq!(first_digit("a1b2c3d4e5f"), Some(1));
        assert_eq!(first_digit("treb7uchet"), Some(7));
    }

    #[test]
    fn test_first_and_last_digit() {
        assert_eq!(first_and_last_digit("1abc2"), Some((1, 2)));
        assert_eq!(first_and_last_digit("pqr3stu8vwx"), Some((3, 8)));
        assert_eq!(first_and_last_digit("a1b2c3d4e5f"), Some((1, 5)));
        assert_eq!(first_and_last_digit("treb7uchet"), Some((7, 7)));
    }

    #[test]
    fn test_calibration_total() {
        assert_eq!(calibration_total("1abc2"), 12);
        assert_eq!(calibration_total("pqr3stu8vwx"), 38);
        assert_eq!(calibration_total("a1b2c3d4e5f"), 15);
        assert_eq!(calibration_total("treb7uchet"), 77);
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

    #[test]
    fn test_part_two_first_digit() {
        assert_eq!(part_two_first_digit("two1nine"), Some(2));
        assert_eq!(part_two_first_digit("eightwothree"), Some(8));
        assert_eq!(part_two_first_digit("abcone2threexyz"), Some(1));
        assert_eq!(part_two_first_digit("xtwone3four"), Some(2));
        assert_eq!(part_two_first_digit("4nineeightseven2"), Some(4));
        assert_eq!(part_two_first_digit("zoneight234"), Some(1));
        assert_eq!(part_two_first_digit("7pqrstsixteen"), Some(7));
    }

    #[test]
    fn test_part_two_first_and_last_digit() {
        assert_eq!(part_two_first_and_last_digit("two1nine"), Some((2, 9)));
        assert_eq!(part_two_first_and_last_digit("eightwothree"), Some((8, 3)));
        assert_eq!(
            part_two_first_and_last_digit("abcone2threexyz"),
            Some((1, 3))
        );
        assert_eq!(part_two_first_and_last_digit("xtwone3four"), Some((2, 4)));
        assert_eq!(
            part_two_first_and_last_digit("4nineeightseven2"),
            Some((4, 2))
        );
        assert_eq!(part_two_first_and_last_digit("zoneight234"), Some((1, 4)));
        assert_eq!(part_two_first_and_last_digit("7pqrstsixteen"), Some((7, 6)));
    }

    #[test]
    fn test_part_two_calibration_total() {
        assert_eq!(part_two_calibration_total("two1nine"), 29);
        assert_eq!(part_two_calibration_total("eightwothree"), 83);
        assert_eq!(part_two_calibration_total("abcone2threexyz"), 13);
        assert_eq!(part_two_calibration_total("xtwone3four"), 24);
        assert_eq!(part_two_calibration_total("4nineeightseven2"), 42);
        assert_eq!(part_two_calibration_total("zoneight234"), 14);
        assert_eq!(part_two_calibration_total("7pqrstsixteen"), 76);
    }

    #[test]
    fn test_part_two_solution() {
        let calibrations = vec![
            String::from("two1nine"),
            String::from("eightwothree"),
            String::from("abcone2threexyz"),
            String::from("xtwone3four"),
            String::from("4nineeightseven2"),
            String::from("zoneight234"),
            String::from("7pqrstsixteen"),
        ];

        assert_eq!(part_two_solution(&calibrations), 281);
    }
}
