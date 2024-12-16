use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

pub fn solve(input_path: &str) -> (String, String) {
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
                start_pos = ((index / width) as i32, (index % width) as i32);
                false
            }
            'S' => {
                end_pos = ((index / width) as i32, (index % width) as i32);
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
        let cost = find_lowest_costs(&labyrinth);
        assert_eq!(cost.to_string(), "7036");
    }

    fn test_part_two() {
        let input = std::fs::read_to_string("./resources/day16/input.txt").unwrap();
        let labyrinth = parse(&input);
        find_lowest_costs(&labyrinth);
    }
}
