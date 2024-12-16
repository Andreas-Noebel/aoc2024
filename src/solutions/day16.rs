use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

pub fn solve(input_path: &str) -> (String, String) {
    let input = std::fs::read_to_string(input_path).unwrap();
    let labyrinth = parse(&input);
    find_shortest_paths(&labyrinth);
    ("".to_string(), "".to_string())
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
        self.walls[(y * (self.height as i32) + x) as usize]
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

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Action {
    MoveForward,
    TurnClockwise,
    TurnCounterclockwise,
    Nothing,
}
#[derive(Clone, Eq, PartialEq, Debug)]
struct ReindeerState {
    //prev_state: Option<Box<ReindeerState>>,
    action: Action,
    position: (i32, i32),
    orientation: Direction,
    cost: i32,
    estimated_cost: i32,
}

impl Ord for ReindeerState {
    fn cmp(&self, other: &Self) -> Ordering {
        let left = self.cost + self.estimated_cost;
        let right = other.cost + other.estimated_cost;

        right
            .cmp(&left)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for ReindeerState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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

fn find_shortest_paths(labyrinth: &Labyrinth) -> i32 {
    let mut open_list = BinaryHeap::<Priority>::new();
    let mut visited: HashSet<Node> = HashSet::new();
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

    //let mut shortest_path = i32::MAX;

    while !open_list.is_empty() {
        let node = open_list.pop().unwrap().node;
        if node.position == labyrinth.end_position {
            let total_costs = costs.get(&node).unwrap();
            return total_costs.clone();
            /*
            if total_costs <= &shortest_path {
                shortest_path = total_costs.clone();
                println!("Solution Found {:?}", total_costs);

                let paths = extract_paths(node.clone(), &predecessors, Vec::new());
                println!("Found {:?} unique paths", paths.len());

                let mut unique_tiles: HashSet<(i32,i32)> = HashSet::new();
                for p in paths{
                    //    println!("Path {:?}", p);
                    for tile in p{
                        print!("{:?} ", tile.position);
                        unique_tiles.insert(tile.position);
                    }
                    println!();
                }
                println!("Unique tiles {:?}", unique_tiles.len());
            }else {
               return;
            }

             */
        }
        visited.insert(node.clone());

        // Expand node
        let successors: Vec<(Node, i32)> = get_successors(&node, &labyrinth);
        for (successor, cost) in successors {
            if visited.contains(&successor) {
                continue;
            }

            let tentative_cost = costs.get(&node).unwrap() + cost;
            // Missing optimization continue if node is already in open set
            /*
            if costs.get(&successor).is_some() {
                if costs.get(&successor).unwrap() < &tentative_cost {
                    continue;
                }else {
                    let pred = predecessors.entry(successor.clone()).or_insert(HashSet::new());
                    pred.clear()
                }
            }
            */

            let pred = predecessors
                .entry(successor.clone())
                .or_insert(HashSet::new());
            pred.insert(node.clone());

            costs.insert(successor.clone(), tentative_cost);

            let priority = tentative_cost + labyrinth.direct_distance_to_end(successor.position);
            open_list.push(Priority {
                priority: priority,
                node: successor,
            })
        }
    }
    0
}

fn extract_path(end_node: Node, p: &HashMap<Node, HashSet<Node>>) {
    let mut current_node = end_node;
    loop {
        match p.get(&current_node) {
            None => break,
            Some(set) => {
                println!("{:?}", set);
                current_node = set.iter().next().unwrap().clone();
            }
        }
    }
}

fn extract_paths(
    end_node: Node,
    predecessors: &HashMap<Node, HashSet<Node>>,
    mut acc: Vec<Node>,
) -> Vec<Vec<Node>> {
    acc.push(end_node.clone());
    match predecessors.get(&end_node) {
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
    let mut sucessors: Vec<(Node, i32)> = vec![
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
        sucessors.push((
            Node {
                position: forward_position,
                orientation: node.orientation,
            },
            1,
        ));
    }
    sucessors
}

fn find_lowest_costs(labyrinth: &Labyrinth) -> i32 {
    let heuristic =
        |pos: (i32, i32), orientation: Direction| labyrinth.direct_distance_to_end(pos) + 2000;

    let mut priority_queue = BinaryHeap::<(ReindeerState)>::new();

    let start_move = ReindeerState {
        //prev_state: None,
        action: Action::Nothing,
        position: labyrinth.start_position,
        orientation: Direction::East,
        cost: 0,
        estimated_cost: heuristic(labyrinth.start_position, Direction::East),
    };

    priority_queue.push(start_move);

    let mut visited: HashMap<((i32, i32), Direction), (i32, HashSet<((i32, i32), Direction)>)> =
        HashMap::new();

    while !priority_queue.is_empty() {
        let current_move = priority_queue.pop().unwrap();

        if labyrinth.is_goal(current_move.position) {
            return current_move.cost;
        }

        // Calculate all possible next moves
        for action in vec![
            Action::MoveForward,
            Action::TurnClockwise,
            Action::TurnCounterclockwise,
        ] {
            let mut next_position = *&current_move.position;
            let mut next_orientation = *&current_move.orientation;
            let mut costs_for_move = 0;
            let mut action_taken = Action::Nothing;
            match action {
                Action::MoveForward => {
                    next_position = match current_move.orientation {
                        Direction::North => {
                            (*&current_move.position.0, *&current_move.position.1 - 1)
                        }
                        Direction::South => {
                            (*&current_move.position.0, *&current_move.position.1 + 1)
                        }
                        Direction::East => {
                            (*&current_move.position.0 + 1, *&current_move.position.1)
                        }
                        Direction::West => {
                            (*&current_move.position.0 - 1, *&current_move.position.1)
                        }
                    };
                    costs_for_move = 1;
                    action_taken = Action::MoveForward;
                }
                Action::TurnClockwise => {
                    next_orientation = match current_move.orientation {
                        Direction::North => Direction::East,
                        Direction::East => Direction::South,
                        Direction::South => Direction::West,
                        Direction::West => Direction::North,
                    };
                    costs_for_move = 1000;
                    action_taken = Action::TurnClockwise;
                }
                Action::TurnCounterclockwise => {
                    next_orientation = match current_move.orientation {
                        Direction::North => Direction::West,
                        Direction::West => Direction::South,
                        Direction::South => Direction::East,
                        Direction::East => Direction::North,
                    };
                    costs_for_move = 1000;
                    action_taken = Action::TurnCounterclockwise;
                }
                Action::Nothing => continue,
            }

            // Check if move is valid
            if labyrinth.is_wall(next_position) {
                continue;
            }

            let next_move = ReindeerState {
                //prev_state: Some(Box::from(current_move.clone())),
                action: action_taken,
                position: next_position,
                orientation: next_orientation,
                cost: costs_for_move + current_move.cost,
                estimated_cost: heuristic(next_position, next_orientation),
            };

            if visited.contains_key(&(next_position, next_orientation)) {
                let (known_best_cost, predecessors) =
                    visited.get_mut(&(next_position, next_orientation)).unwrap();

                match known_best_cost.clone().cmp(&next_move.cost) {
                    Ordering::Less => {
                        continue;
                    }
                    Ordering::Equal => {}
                    _ => {
                        predecessors.insert((next_position, next_orientation));
                    }
                }
                //if *known_best_cost < next_move.cost { continue; }
            }

            visited.insert(
                (next_position, next_orientation),
                (
                    next_move.cost,
                    HashSet::from([(current_move.position, current_move.orientation)]),
                ),
            );
            priority_queue.push(next_move);
        }
    }
    i32::MAX
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
            'E' => {
                start_pos = ((index % width) as i32, (index / width) as i32);
                false
            }
            'S' => {
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
    fn test_parse_input() {
        let input = std::fs::read_to_string("./resources/day16/example.txt").unwrap();
        let labyrinth = parse(&input);
        println!("{:?}", labyrinth)
    }

    #[test]
    fn test_part_one() {
        let input = std::fs::read_to_string("./resources/day16/example.txt").unwrap();
        let labyrinth = parse(&input);
        let l = find_shortest_paths(&labyrinth);
        assert_eq!(l.to_string(), "7036");
    }

    #[test]
    fn test_part_two() {
        let input = std::fs::read_to_string("./resources/day16/example.txt").unwrap();
        let labyrinth = parse(&input);
        let l = find_shortest_paths(&labyrinth);
        println!("{:?}",l)
    }

    #[test]
    fn test_custom() {
        let input = std::fs::read_to_string("./resources/day16/example_custom.txt").unwrap();
        let labyrinth = parse(&input);
        find_shortest_paths(&labyrinth);
    }
}
