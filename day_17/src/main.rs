// TODO: Switch the Node Ord trait back and then pull from the back of the list,
// or reverse the order when adding to the heap. I don't like inverting the generic
// Ord impl.
use std::collections::BinaryHeap;

fn main() {
    // let input = include_str!("../data/puzzle_input.txt");
    let input = include_str!("../data/test_input.txt");
    let lava_ducts = process_input(input);

    let part_one_answer = part_one_solution(&lava_ducts);
    println!("The Part one answer is {part_one_answer}");

    // distance to the node
    // distance to the node and any remaining distance

    // S current distance 0, distance to end 10
    // A distance of 7, combined distance of 16
    // B distance of 2, combined distance of 9
    // use B first as it's smaller than A
    // keeping tracking of distance but not using it to determine next node to check
    // C has distance of 3, combined distance of 11
    // H has the distance of the previous steps (S -> B) plus the current cost B -> H, combined cost is that plus the distance from H to end
    // once B is done, put it into the finished stack

    // translation
    // each node's heat loss should be the cumulative cost of all the heat loss it
    //      takes to get to the current position
    // plus the cost from _just that_ node to the end
    //
}

type DuctMap = Vec<Vec<u32>>;
type Coord = (usize, usize);

#[derive(Clone, PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Right,
    Left,
    Down,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Node {
    position: Coord,
    direction: Direction,
    cost: u32,
    heat_loss: u32,
    last_three_directions: Vec<Direction>,
    path: Vec<(Coord, Direction)>,
}

impl Node {
    fn get_cardinal_neighbours(&self, map: &DuctMap) -> Vec<(Coord, Direction)> {
        let (x, y) = self.position;

        let above = (Some(x), y.checked_sub(1), Direction::Up);
        let right = (x.checked_add(1), Some(y), Direction::Right);
        let below = (Some(x), y.checked_add(1), Direction::Down);
        let left = (x.checked_sub(1), Some(y), Direction::Left);

        let mut neighbours = vec![above, right, below, left];

        if self.last_three_directions.len() > 2 {
            let boop = &self.direction;

            // TODO: Either switch last three to be last two, or update vec with current node
            // before checking
            let limit_hit = self.last_three_directions[1] == self.last_three_directions[2]
                && self.last_three_directions[1] == self.direction;

            if limit_hit {
                neighbours = neighbours
                    .into_iter()
                    .filter(|(_, _, direction)| direction != boop)
                    .collect();
            }
        }

        let opposite_direction = match self.direction {
            Direction::Down => Direction::Up,
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
        };

        neighbours.retain(|(_, _, direction)| direction != &opposite_direction);

        // if self.position == (3, 0) {
        //     println!("we did it!!!");
        //     println!("last three: {:?}", &self.last_three_directions);
        //     println!("current node: {:?}", &self.direction);
        //     println!("neighbours: {:?}", neighbours);
        //     //
        //     println!("last neighbour: {:?}", &self.last_three_directions[2]);
        //     println!(
        //         "second last neighbour: {:?}",
        //         &self.last_three_directions[1]
        //     );
        // }

        let mut formatted_neighbours: Vec<_> = neighbours
            .into_iter()
            .filter_map(|neighbour| {
                if neighbour.0.is_some() && neighbour.1.is_some() {
                    Some(((neighbour.0.unwrap(), neighbour.1.unwrap()), neighbour.2))
                } else {
                    None
                }
            })
            .collect();

        let last = self.path.last();

        if let Some(((x, y), _)) = last {
            let parent = formatted_neighbours
                .iter()
                .position(|((nx, ny), _)| nx == x && ny == y);

            if let Some(parent) = parent {
                formatted_neighbours.remove(parent);
            }
        }

        formatted_neighbours
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // reversing the order so the binary heap returns the smallest cost first
        match self.cost.cmp(&other.cost) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Equal => std::cmp::Ordering::Equal,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Less,
        }
    }
}

fn distance_to_end(from: &Coord, to: &Coord) -> u32 {
    (from.0.abs_diff(to.0) + from.1.abs_diff(to.1)) as u32
}

fn destination_coordinates(duct_map: &DuctMap) -> Coord {
    let y = duct_map.len() - 1;
    let x = duct_map[0].len() - 1;

    (x, y)
}

fn calculate_node_cost(heat_loss: u32, distance: u32, current_node: &Node) -> u32 {
    // distance to end for neighbour
    // cost to here + (heat_loss + distance_to_end)
    (current_node.heat_loss + heat_loss) + distance
}

fn part_one_solution(map: &DuctMap) -> u32 {
    let start = Node {
        position: (0, 0),
        direction: Direction::Left,
        cost: 0,
        heat_loss: 0,
        last_three_directions: vec![],
        path: vec![],
    };
    let destination = destination_coordinates(map);

    let mut open_list = BinaryHeap::new();
    open_list.push(start);

    let mut closed_list: Vec<Node> = vec![];

    let mut current_node = open_list.pop().unwrap();

    while current_node.position != destination {
        // println!("current node is at: {:?}", current_node.position);
        // println!("open node size is: {}", open_list.len());
        // println!("closed node size is: {}", closed_list.len());

        let neighbours = current_node.get_cardinal_neighbours(map);

        let mut new_nodes: BinaryHeap<Node> = neighbours
            .into_iter()
            .filter(|((x, y), _)| {
                let exists = map.get(*y).and_then(|row| row.get(*x));
                exists.is_some()
            })
            // .filter(|(new_coords, _)| !open_list.iter().any(|node| &node.position == new_coords))
            .filter(|(coords, _)| !closed_list.iter().any(|node| &node.position == coords))
            .map(|(neighbour, direction)| {
                let distance = distance_to_end(&neighbour, &destination);
                let node_heat_loss = map[neighbour.1][neighbour.0];
                // let cost = calculate_node_cost(node_heat_loss, distance, &current_node);
                let mut last_three = current_node.last_three_directions.clone();
                if last_three.len() >= 3 {
                    last_three.remove(0);
                }
                last_three.push(current_node.direction.clone());
                let mut new_path = current_node.path.clone();
                new_path.push((current_node.position, current_node.direction.clone()));

                Node {
                    position: neighbour,
                    direction,
                    heat_loss: current_node.heat_loss + node_heat_loss,
                    cost: (current_node.heat_loss + node_heat_loss) + distance,
                    last_three_directions: last_three,
                    path: new_path,
                }
            })
            .collect();

        // if the position exists in the open list
        // and the current one has a lower cost
        // remove it from the open node list
        // else skip it
        // println!("new_nodes: {:?}", new_nodes);

        new_nodes.retain(|new_node| {
            let existing_node = open_list
                .iter()
                .find(|node| node.position == new_node.position);

            if let Some(existing_node) = existing_node {
                if new_node.cost < existing_node.cost {
                    // remove it from the list
                    open_list.retain(|node: &Node| node.position != new_node.position);

                    true
                } else {
                    false
                }
            } else {
                true
            }
        });

        open_list.append(&mut new_nodes);
        closed_list.push(current_node);

        current_node = open_list.pop().unwrap();
    }

    println!("current_node.path: {:?}", current_node.path);
    // println!("open list: {}", open_list.len());
    // println!("last open pos: {:?}", open_list.pop().unwrap().position);
    // println!("last open cost: {:?}", open_list.pop().unwrap().cost);

    current_node.heat_loss
}

fn process_input(input: &str) -> DuctMap {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod test_super {
    use super::*;

    fn test_data() -> DuctMap {
        process_input(include_str!("../data/test_input.txt"))
    }

    #[test]
    fn test_process_input() {
        let map = test_data();

        assert_eq!(map.len(), 13);
        assert_eq!(map[0].len(), 13);
        assert_eq!(map[0][0], 2);
    }

    #[test]
    fn test_destination_coordinates() {
        let destination = destination_coordinates(&test_data());

        assert_eq!(destination, (12, 12));
    }

    #[test]
    fn test_get_cardinal_neighbours() {
        let start = Node {
            position: (0, 0),
            direction: Direction::Left,
            cost: 0,
            heat_loss: 0,
            last_three_directions: vec![],
            path: vec![],
        };

        let neighbours = start.get_cardinal_neighbours(&test_data());

        assert_eq!(neighbours.len(), 2);

        let ((x, y), direction) = &neighbours[0];
        assert_eq!(direction, &Direction::Right);
        assert_eq!(x, &1);
        assert_eq!(y, &0);

        let ((x, y), direction) = &neighbours[1];
        assert_eq!(direction, &Direction::Down);
        assert_eq!(x, &0);
        assert_eq!(y, &1);
    }

    // TODO test distance_to_end and calculate_node_cost

    #[test]
    fn test_part_one_example() {
        let part_one_answer = part_one_solution(&test_data());

        assert_eq!(part_one_answer, 102);
    }
}
