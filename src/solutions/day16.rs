use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

pub fn solve(input_path: &str) -> (String, String) {
    let input = std::fs::read_to_string(input_path).unwrap();
    let labyrinth = parse(&input);
    let (s1, s2) = find_shortest_paths(&labyrinth).unwrap();
    (s1.to_string(), s2.to_string())
}

#[derive(Debug)]
struct Labyrinth {
    width: usize,
    height: usize,
    start_position: (i32, i32),
    end_position: (i32, i32),
    walls: Vec<bool>,
}

impl Labyrinth {
    fn is_wall(&self, (x, y): (i32, i32)) -> bool {
        self.walls[(y * (self.width as i32) + x) as usize]
    }

    fn is_goal(&self, (x, y): (i32, i32)) -> bool {
        (x, y) == self.end_position
    }

    fn direct_distance_to_end(&self, (x, y): (i32, i32)) -> i32 {
        let (dx, dy) = (self.end_position.0 - x, self.end_position.1 - y);
        ((dx * dx + dy * dy) as f64).sqrt() as i32
    }
}
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn to_direction(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        }
    }
    fn rot_clockwise(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
    fn rot_anti_clockwise(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Node {
    position: (i32, i32),
    orientation: Direction,
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Priority {
    priority: i32,
    node: Node,
}

impl PartialOrd<Self> for Priority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Priority {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)
    }
}

fn find_shortest_paths(labyrinth: &Labyrinth) -> Option<(i32, i32)> {
    let mut open_list = BinaryHeap::<Priority>::new();
    let mut costs: HashMap<Node, i32> = HashMap::new();
    let mut predecessors: HashMap<Node, HashSet<Node>> = HashMap::new();

    let start_node = Node {
        position: labyrinth.start_position,
        orientation: Direction::East,
    };

    costs.insert(start_node.clone(), 0);

    open_list.push(Priority {
        priority: 0,
        node: start_node,
    });

    let mut shortest_path_costs = i32::MAX;
    let mut unique_tiles: HashSet<(i32, i32)> = HashSet::new();

    while !open_list.is_empty() {
        let node = open_list.pop().unwrap().node;

        if labyrinth.is_goal(node.position) {
            let total_path_costs = costs.get(&node).unwrap();

            // Keep collecting solutions while they are optimal
            if total_path_costs <= &shortest_path_costs {
                shortest_path_costs = total_path_costs.clone();
                let paths = extract_paths(node.clone(), &predecessors, Vec::new());
                for path_nodes in paths {
                    for tile in path_nodes {
                        unique_tiles.insert(tile.position);
                    }
                }
            } else {
                // print_with_paths(&labyrinth, &unique_tiles);
                return Some((shortest_path_costs, unique_tiles.len() as i32));
            }
        }

        // Expand node
        for (successor, cost) in get_successors(&node, &labyrinth) {
            let tentative_cost = costs.get(&node).unwrap() + cost;

            // This branch is too expensive
            if let Some(known_costs) = costs.get(&successor) {
                if known_costs < &tentative_cost {
                    continue;
                }
            }

            let pred = predecessors
                .entry(successor.clone())
                .or_insert(HashSet::new());
            pred.insert(node.clone());

            costs.insert(successor.clone(), tentative_cost);

            let priority = tentative_cost + labyrinth.direct_distance_to_end(successor.position);

            open_list.push(Priority {
                priority,
                node: successor,
            })
        }
    }
    None
}

#[allow(dead_code)]
fn print_with_paths(labyrinth: &Labyrinth, seats: &HashSet<(i32, i32)>) {
    for y in 0..labyrinth.height {
        for x in 0..labyrinth.width {
            let pos = (x as i32, y as i32);
            if labyrinth.is_wall(pos) {
                print!("#");
            } else if seats.contains(&pos) {
                print!("\x1b[38;2;76;175;80mO\x1b[0m");
            } else {
                print!(".")
            }
        }
        println!();
    }
}

fn extract_paths(
    start_node: Node,
    predecessors: &HashMap<Node, HashSet<Node>>,
    mut acc: Vec<Node>,
) -> Vec<Vec<Node>> {
    acc.push(start_node.clone());
    match predecessors.get(&start_node) {
        None => vec![acc],
        Some(set) => {
            let mut sub_paths: Vec<Vec<Node>> = Vec::new();
            for pred in set.iter() {
                let sub_paths_for_pred = extract_paths(pred.clone(), predecessors, Vec::new());
                for sub_sub_path in sub_paths_for_pred {
                    let mut a_s = acc.clone();
                    a_s.extend(sub_sub_path.clone());
                    sub_paths.push(a_s);
                }
            }
            sub_paths
        }
    }
}

fn get_successors(node: &Node, labyrinth: &Labyrinth) -> Vec<(Node, i32)> {
    let mut successors: Vec<(Node, i32)> = vec![
        (
            Node {
                position: node.position,
                orientation: node.orientation.rot_clockwise(),
            },
            1000,
        ),
        (
            Node {
                position: node.position,
                orientation: node.orientation.rot_anti_clockwise(),
            },
            1000,
        ),
    ];
    let (x, y) = node.position.clone();
    let (dx, dy) = node.orientation.to_direction();
    let forward_position = (x + dx, y + dy);

    if !labyrinth.is_wall(forward_position) {
        successors.push((
            Node {
                position: forward_position,
                orientation: node.orientation,
            },
            1,
        ));
    }
    successors
}

fn parse(input: &str) -> Labyrinth {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let mut start_pos = (0, 0);
    let mut end_pos = (0, 0);

    let data = input
        .chars()
        .filter(|c| !c.is_whitespace())
        .enumerate()
        .map(|(index, c)| match c {
            '#' => true,
            '.' => false,
            'S' => {
                start_pos = ((index % width) as i32, (index / width) as i32);
                false
            }
            'E' => {
                end_pos = ((index % width) as i32, (index / width) as i32);
                false
            }
            _ => {
                unreachable!()
            }
        })
        .collect::<Vec<bool>>();

    Labyrinth {
        width,
        height,
        start_position: start_pos,
        end_position: end_pos,
        walls: data,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = std::fs::read_to_string("./resources/day16/example.txt").unwrap();
        let labyrinth = parse(&input);
        let (l, _) = find_shortest_paths(&labyrinth).unwrap();
        assert_eq!(l.to_string(), "7036");
    }

    #[test]
    fn test_part_two() {
        let input = std::fs::read_to_string("./resources/day16/example.txt").unwrap();
        let labyrinth = parse(&input);
        let (_, l) = find_shortest_paths(&labyrinth).unwrap();
        assert_eq!(l.to_string(), "45");
    }
}
