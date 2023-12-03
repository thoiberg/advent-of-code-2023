fn main() {
    let input = read_input();
    let games = process_input(input);

    let part_one_answer = part_one_solution(&games);
    println!("Part One Answer is: {part_one_answer}");

    let part_two_answer = part_two_solution(&games);
    println!("Part Two Answer is: {part_two_answer}");
}

fn part_one_solution(games: &[Game]) -> u32 {
    let cube_checks = vec![
        Cube {
            amount: 14,
            colour: CubeColour::Blue,
        },
        Cube {
            amount: 13,
            colour: CubeColour::Green,
        },
        Cube {
            amount: 12,
            colour: CubeColour::Red,
        },
    ];

    check_games(games, &cube_checks)
        .iter()
        .fold(0, |acc, game| acc + game.id)
}

fn part_two_solution(games: &[Game]) -> u32 {
    let colours = vec![CubeColour::Red, CubeColour::Blue, CubeColour::Green];

    games
        .iter()
        .map(|game| {
            colours
                .iter()
                .fold(1, |acc, colour| acc * game.max_for(colour))
        })
        .sum()
}

fn check_games<'a>(games: &'a [Game], cube_checks: &[Cube]) -> Vec<&'a Game> {
    games
        .iter()
        .filter(|game| {
            cube_checks.iter().all(|cube| {
                game.amounts_for(&cube.colour)
                    .iter()
                    .max()
                    .map(|max| max <= &cube.amount)
                    .unwrap_or(true)
            })
        })
        .collect()
}

fn read_input() -> String {
    String::from(include_str!("./puzzle_input.txt"))
}

fn process_input(input: String) -> Vec<Game> {
    let games: Vec<&str> = input.split('\n').collect();

    games
        .iter()
        .map(|game| {
            let parts = game.split(':').collect::<Vec<&str>>();
            if parts.len() < 2 {
                panic!("could not split {game}");
            }

            let game_id: u32 = parts[0].split(' ').collect::<Vec<&str>>()[1]
                .parse::<u32>()
                .unwrap();
            let rounds = parts[1].trim().split(';').collect::<Vec<&str>>();

            let round_objs: Vec<Round> = rounds
                .iter()
                .map(|round| {
                    let cubes = round.split(',').collect::<Vec<&str>>();
                    let cube_objs: Vec<Cube> = cubes
                        .iter()
                        .map(|cube| {
                            let cube_stuff = cube.trim().split(' ').collect::<Vec<&str>>();

                            let cube_amount = cube_stuff[0].parse::<u32>().unwrap();
                            let cube_colour = cube_stuff[1];

                            Cube::try_new(cube_amount, cube_colour).unwrap()
                        })
                        .collect();

                    Round { cubes: cube_objs }
                })
                .collect();

            Game {
                id: game_id,
                rounds: round_objs,
            }
        })
        .collect()
}

struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl Game {
    fn amounts_for(&self, colour: &CubeColour) -> Vec<u32> {
        self.rounds
            .iter()
            .map(|round| round.amount_for(colour))
            .collect()
    }

    fn max_for(&self, colour: &CubeColour) -> u32 {
        *(self.amounts_for(colour).iter().max().unwrap())
    }
}

struct Round {
    cubes: Vec<Cube>,
}

impl Round {
    fn amount_for(&self, colour: &CubeColour) -> u32 {
        self.cubes
            .iter()
            .find(|cube| &cube.colour == colour)
            .map(|cube| cube.amount)
            .unwrap_or(0)
    }
}

struct Cube {
    amount: u32,
    colour: CubeColour,
}

impl Cube {
    fn try_new(amount: u32, colour: &str) -> Result<Self, &str> {
        let cube_colour = CubeColour::try_from(colour)?;

        Ok(Self {
            amount,
            colour: cube_colour,
        })
    }
}

#[derive(PartialEq, Debug)]
enum CubeColour {
    Red,
    Blue,
    Green,
}

impl CubeColour {
    fn try_from(colour: &str) -> Result<Self, &str> {
        match colour {
            "green" => Ok(Self::Green),
            "red" => Ok(Self::Red),
            "blue" => Ok(Self::Blue),
            _ => Err("Could not convert {colour} into a cube property"),
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    fn test_games() -> Vec<Game> {
        let test_input = String::from(include_str!("./test_puzzle_input.txt"));
        process_input(test_input)
    }

    #[test]
    fn test_part_one_solution() {
        assert_eq!(part_one_solution(&test_games()), 8);
    }

    #[test]
    fn test_part_one_answer() {
        let test_input = String::from(include_str!("./puzzle_input.txt"));
        let games = process_input(test_input);

        assert_eq!(part_one_solution(&games), 2727);
    }

    #[test]
    fn test_part_two_answer() {
        assert_eq!(part_two_solution(&test_games()), 2286);
    }

    #[test]
    fn test_part_two_solution() {
        let test_input = String::from(include_str!("./puzzle_input.txt"));
        let games = process_input(test_input);

        assert_eq!(part_two_solution(&games), 56580);
    }

    #[test]
    fn test_processing_input() {
        let test_input = String::from(include_str!("./test_puzzle_input.txt"));
        let games = process_input(test_input);

        assert_eq!(games.len(), 5);

        let first_game = &games[0];
        assert_eq!(first_game.rounds.len(), 3);

        assert_eq!(first_game.rounds[0].cubes[0].amount, 3);
        assert_eq!(first_game.rounds[0].cubes[0].colour, CubeColour::Blue);

        assert_eq!(first_game.rounds[0].cubes[1].amount, 4);
        assert_eq!(first_game.rounds[0].cubes[1].colour, CubeColour::Red);
    }
}
