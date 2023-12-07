// TODO: Parse the data file to retrieve the data, instead of manually adding
fn main() {
    let races = vec![
        Race {
            time: 38,
            distance: 234,
        },
        Race {
            time: 67,
            distance: 1027,
        },
        Race {
            time: 76,
            distance: 1157,
        },
        Race {
            time: 73,
            distance: 1236,
        },
    ];
    let part_one_answer = part_one_solution(&races);
    println!("Part One Answer is: {part_one_answer}");
}

fn part_one_solution(races: &[Race]) -> usize {
    races.iter().fold(1, |acc, race| {
        let possibilities = race_possibilities(race);
        let winners = winning_possibilities(race, &possibilities);
        acc * winners.len()
    })
}

fn race_possibilities(race: &Race) -> Vec<u32> {
    (1..race.time)
        .map(|hold_time| {
            let remaining_time = race.time - hold_time;
            remaining_time * hold_time
        })
        .collect()
}

fn winning_possibilities<'a>(race: &Race, possibilities: &'a Vec<u32>) -> Vec<&'a u32> {
    possibilities
        .iter()
        .filter(|possibility| possibility > &&race.distance)
        .collect()
}

struct Race {
    time: u32,
    distance: u32,
}

#[cfg(test)]
mod test_super {
    use super::*;

    fn test_data() -> Vec<Race> {
        vec![
            Race {
                time: 7,
                distance: 9,
            },
            Race {
                time: 15,
                distance: 40,
            },
            Race {
                time: 30,
                distance: 200,
            },
        ]
    }

    #[test]
    fn test_race_possibilities() {
        let races = test_data();

        let possibilities = race_possibilities(&races[0]);
        assert_eq!(possibilities, [6, 10, 12, 12, 10, 6]);
    }

    #[test]
    fn test_winning_possibilities() {
        let races = test_data();

        let possibilities = vec![6, 10, 12, 12, 10, 6];
        let winners = winning_possibilities(&races[0], &possibilities);

        assert_eq!(winners, [&10, &12, &12, &10]);
    }

    #[test]
    fn test_part_one_solution() {
        let races = test_data();

        let answer = part_one_solution(&races);

        assert_eq!(answer, 288);
    }
}
