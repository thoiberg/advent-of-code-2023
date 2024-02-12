// Maths taken from https://www.youtube.com/watch?v=guOyA7Ijqgk

use std::ops::RangeInclusive;

fn main() {
    let input = include_str!("../data/puzzle_input.txt");
    let hailstones = process_input(input);

    let test_area = test_area();
    let part_one_answer = part_one_solution(&hailstones, &test_area);
    println!("Part one answer is {part_one_answer}");
}

fn test_area() -> RangeInclusive<f64> {
    200_000_000_000_000.0..=400_000_000_000_000.0
}

#[derive(PartialEq, Debug)]
struct Hailstone {
    px: f64,
    vx: f64,
    py: f64,
    vy: f64,
    pz: f64,
    vz: f64,
    a: f64,
    b: f64,
    c: f64,
}

impl Hailstone {
    fn new(px: f64, py: f64, pz: f64, vx: f64, vy: f64, vz: f64) -> Self {
        Self {
            px,
            vx,
            py,
            vy,
            pz,
            vz,
            a: vy,
            b: -vx,
            c: vy * px - vx * py,
        }
    }
}

fn part_one_solution(hailstones: &[Hailstone], test_area: &RangeInclusive<f64>) -> usize {
    let mut total = 0;

    for i in 0..hailstones.len() {
        let h1 = &hailstones[i];

        for h2 in hailstones.iter().skip(i) {
            let (a1, b1, c1) = (h1.a, h1.b, h1.c);
            let (a2, b2, c2) = (h2.a, h2.b, h2.c);

            if a1 * b2 == b1 * a2 {
                continue;
            }

            let x = (c1 * b2 - c2 * b1) / (a1 * b2 - a2 * b1);
            let y = (c2 * a1 - c1 * a2) / (a1 * b2 - a2 * b1);

            if test_area.contains(&x) && test_area.contains(&y) {
                let intersects_in_the_future = [h1, h2].iter().all(|hailstone| {
                    (x - hailstone.px) * hailstone.vx >= 0.0
                        && (y - hailstone.py) * hailstone.vy >= 0.0
                });

                if intersects_in_the_future {
                    total += 1;
                }
            }
        }
    }

    total
}

fn process_input(input: &str) -> Vec<Hailstone> {
    input
        .lines()
        .map(|hail_data| {
            let (hail_pos, hail_velocity) = hail_data.split_once('@').unwrap();
            let hail_axes: Vec<_> = hail_pos
                .split(',')
                .map(|hail_axis| hail_axis.trim().parse::<f64>().unwrap())
                .collect();
            let hail_velocities: Vec<_> = hail_velocity
                .split(',')
                .map(|hail_axis_velocity| hail_axis_velocity.trim().parse::<f64>().unwrap())
                .collect();

            // TODO: create two iters and zip them together into the Hailstone
            Hailstone::new(
                hail_axes[0],
                hail_axes[1],
                hail_axes[2],
                hail_velocities[0],
                hail_velocities[1],
                hail_velocities[2],
            )
        })
        .collect()
}

#[cfg(test)]
mod test_super {
    use super::*;

    fn test_data() -> Vec<Hailstone> {
        process_input(include_str!("../data/test_input.txt"))
    }

    #[test]
    fn test_process_input() {
        let hailstones = test_data();

        assert_eq!(hailstones.len(), 5);

        let first = &hailstones[0];
        assert_eq!(first, &Hailstone::new(19.0, 13.0, 30.0, -2.0, 1.0, -2.0));
    }

    #[test]
    fn test_hailstones_constants() {
        let hailstone = &test_data()[0];

        assert_eq!(hailstone.a, 1.0);
        assert_eq!(hailstone.b, 2.0);
        assert_eq!(hailstone.c, 45.0);
    }

    #[test]
    fn test_part_one_example() {
        let hailstones = test_data();
        let test_area = 7.0..=27.0;

        let part_one_example = part_one_solution(&hailstones, &test_area);

        assert_eq!(part_one_example, 2);
    }

    #[test]
    fn test_part_one_solution() {
        let hailstones = process_input(include_str!("../data/puzzle_input.txt"));
        let test_area = test_area();

        let part_one_solution = part_one_solution(&hailstones, &test_area);

        assert_eq!(part_one_solution, 14799);
    }
}
