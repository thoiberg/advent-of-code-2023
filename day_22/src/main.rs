// TODO: Replace InclusiveRange with a better data structure
//      IR does not include a length method, so I need to calculate it myself
//      which makes it a lot less useful here.
// TODO: Replace Ids with reference to supporting brick.
//      I only need the id so I can retain a unique reference to the supporting
//      brick, but this should be do-able by supplying a reference to the data.
//      Will also allow me to avoid additional array lookups.

use std::{collections::HashSet, error::Error, ops::RangeInclusive};

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../data/puzzle_input.txt");
    let mut bricks = process_input(input)?;

    let part_one_answer = part_one_solution(&mut bricks);
    println!("Part One answer is {part_one_answer}");

    Ok(())
}

fn part_one_solution(bricks: &mut [Brick]) -> usize {
    bricks.sort_by(|a, b| a.z.start().partial_cmp(b.z.start()).unwrap());

    drop_bricks(bricks);

    let mut load_bearing_bricks: HashSet<&Brick> = HashSet::new();

    bricks.iter().for_each(|brick| {
        if brick.supporting_bricks.len() == 1 {
            let brick_id = brick.supporting_bricks[0];
            let supporting_brick = bricks.iter().find(|brick| brick.id == brick_id).unwrap();
            load_bearing_bricks.insert(supporting_brick);
        }
    });

    bricks.len() - load_bearing_bricks.len()
}

fn drop_bricks(bricks: &mut [Brick]) {
    for i in 0..bricks.len() {
        let brick = &bricks[i];
        let intersecting: Vec<Brick>;
        {
            intersecting = bricks
                .iter()
                .filter(|other| brick.intersects(other))
                .filter(|other| brick.z.end() > other.z.end())
                // TODO: remove need to clone
                .cloned()
                .collect();
        }
        // TODO: figure out a better way then re-setting the brick
        // (Needed because I can't have a mutable reference and then iter over bricks)
        let brick = &mut bricks[i];
        // TODO: Figure out the weird off by one issues with length
        let length = (brick.z.end() - brick.z.start()) + 1;

        if intersecting.is_empty() {
            brick.z = 1..=(length);
        } else {
            let highest_brick = intersecting
                .iter()
                .max_by(|a, b| a.z.end().cmp(b.z.end()))
                .unwrap();

            let supporting_brick_ids: Vec<u32> = intersecting
                .iter()
                .filter(|brick| brick.z.end() == highest_brick.z.end())
                .map(|brick| brick.id)
                .collect();

            brick.supporting_bricks = supporting_brick_ids;

            let new_z_start = highest_brick.z.end() + 1;
            brick.z = (new_z_start)..=(new_z_start + (length - 1));
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Brick {
    id: u32,
    supporting_bricks: Vec<u32>,
    x: RangeInclusive<u32>,
    y: RangeInclusive<u32>,
    z: RangeInclusive<u32>,
}

impl Brick {
    fn intersects(&self, other: &Brick) -> bool {
        let x_intersects = (self.x.contains(other.x.start()) || self.x.contains(other.x.end()))
            || (other.x.contains(self.x.start()) || other.x.contains(self.x.end()));
        let y_intersects = (self.y.contains(other.y.start()) || self.y.contains(other.y.end()))
            || (other.y.contains(self.y.start()) || other.y.contains(self.y.end()));

        x_intersects && y_intersects
    }
}

fn process_input(input: &str) -> Result<Vec<Brick>, Box<dyn Error>> {
    let bricks = input
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            let (brick_start, brick_end) = line.split_once('~').unwrap();

            let start_coordinates: Vec<_> = brick_start
                .split(',')
                // TODO: look for a nicer way to express this
                .flat_map(|pos| Ok::<u32, Box<dyn Error>>(pos.parse::<u32>()?))
                .collect();
            let end_coordinates: Vec<_> = brick_end
                .split(',')
                .flat_map(|pos| Ok::<u32, Box<dyn Error>>(pos.parse::<u32>()?))
                .collect();

            Brick {
                id: idx as u32,
                supporting_bricks: vec![],
                x: start_coordinates[0]..=end_coordinates[0],
                y: start_coordinates[1]..=end_coordinates[1],
                z: start_coordinates[2]..=end_coordinates[2],
            }
        })
        .collect();

    Ok(bricks)
}

#[cfg(test)]
mod test_super {
    use super::*;

    fn test_data() -> Vec<Brick> {
        process_input(include_str!("../data/test_input.txt")).unwrap()
    }

    #[test]
    fn test_process_input() {
        let bricks = test_data();

        assert_eq!(bricks.len(), 7);

        let first = &bricks[0];
        assert_eq!(first.x, 1..=1);
        assert_eq!(first.y, 0..=2);
        assert_eq!(first.z, 1..=1);
    }

    #[test]
    fn test_drop_bricks() {
        let mut bricks = test_data();

        drop_bricks(&mut bricks);

        assert_eq!(bricks.len(), 7);

        let a = &bricks[0];
        assert_eq!(a.x, 1..=1);
        assert_eq!(a.y, 0..=2);
        assert_eq!(a.z, 1..=1);
        assert_eq!(a.supporting_bricks, []);

        let b = &bricks[1];
        assert_eq!(b.x, 0..=2);
        assert_eq!(b.y, 0..=0);
        assert_eq!(b.z, 2..=2);
        assert_eq!(b.supporting_bricks, [0]);

        let c = &bricks[2];
        assert_eq!(c.x, 0..=2);
        assert_eq!(c.y, 2..=2);
        assert_eq!(c.z, 2..=2);
        assert_eq!(c.supporting_bricks, [0]);

        let d = &bricks[3];
        assert_eq!(d.x, 0..=0);
        assert_eq!(d.y, 0..=2);
        assert_eq!(d.z, 3..=3);
        assert_eq!(d.supporting_bricks, [1, 2]);

        let e = &bricks[4];
        assert_eq!(e.x, 2..=2);
        assert_eq!(e.y, 0..=2);
        assert_eq!(e.z, 3..=3);
        assert_eq!(e.supporting_bricks, [1, 2]);

        let f = &bricks[5];
        assert_eq!(f.x, 0..=2);
        assert_eq!(f.y, 1..=1);
        assert_eq!(f.z, 4..=4);
        assert_eq!(f.supporting_bricks, [3, 4]);

        let g = &bricks[6];
        assert_eq!(g.x, 1..=1);
        assert_eq!(g.y, 1..=1);
        assert_eq!(g.z, 5..=6);
        assert_eq!(g.supporting_bricks, [5]);
    }

    #[test]
    fn test_part_one_example() {
        let mut bricks = test_data();

        let part_one_example = part_one_solution(&mut bricks);

        assert_eq!(part_one_example, 5);
    }

    #[test]
    fn test_part_one_answer() {
        let mut bricks = process_input(include_str!("../data/puzzle_input.txt")).unwrap();

        assert_eq!(part_one_solution(&mut bricks), 480);
    }
}
