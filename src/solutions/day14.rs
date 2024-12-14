use std::cmp::max;
use std::collections::{HashSet, VecDeque};

type Puzzle = Vec<Robot>;
type Position = (i32, i32);
type Velocity = (i32, i32);
type Robot = (Position, Velocity);

pub fn solve(input_file_path: &str) -> (String, String) {
    let input = std::fs::read_to_string(input_file_path).unwrap();
    let puzzle = parse_puzzle(&input);
    let solution_part_one = solve_part_one(101, 103, &puzzle);
    let solution_part_two = solve_part_two(101, 103, &puzzle);

    (solution_part_one, solution_part_two)
}

fn solve_part_one(width: i32, height: i32, puzzle: &Puzzle) -> String {
    let middle = (width / 2, height / 2);
    let simulations_steps = 100;
    let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);

    for robot in puzzle {
        let pos_new = simulate_robot(robot, width, height, simulations_steps);
        if pos_new.0 < middle.0 && pos_new.1 < middle.1 {
            q1 += 1
        } else if pos_new.0 < middle.0 && pos_new.1 > middle.1 {
            q2 += 1
        } else if pos_new.0 > middle.0 && pos_new.1 > middle.1 {
            q3 += 1
        } else if pos_new.0 > middle.0 && pos_new.1 < middle.1 {
            q4 += 1
        }
    }

    let solution = q1 * q2 * q3 * q4;
    solution.to_string()
}

fn solve_part_two(width: i32, height: i32, puzzle: &Puzzle) -> String {
    let cluster_size_threshold = 115; // Found after pressing enter multiple thousand times

    for steps in 0..20000 {
        let robot_positions: Vec<Position> = puzzle
            .iter()
            .map(|robot| simulate_robot(robot, width, height, steps))
            .collect();

        let robot_positions: HashSet<Position> =
            HashSet::from_iter(robot_positions.iter().cloned());

        let largest_cluster = get_largest_cluster(&robot_positions);

        if largest_cluster >= cluster_size_threshold {
            // visualize(&robot_positions, width, height);
            return steps.to_string();
        }
    }

    "No solution in step: [0,20000] with cluster_threshold: 115".to_string()
}

fn simulate_robot(robot: &Robot, width: i32, height: i32, steps: i32) -> Position {
    let ((x, y), (vx, vy)) = robot;
    let mut new_x = (x + steps * vx) % width;
    let mut new_y = (y + steps * vy) % height;
    if new_x < 0 {
        new_x += width;
    }
    if new_y < 0 {
        new_y += height;
    }
    (new_x, new_y)
}

fn get_largest_cluster(positions: &HashSet<Position>) -> i32 {
    let pos_copy = positions.clone();
    let mut largest_cluster = 0;

    let mut visited: HashSet<Position> = HashSet::new();

    for position in pos_copy {
        let mut open_set: VecDeque<Position> = VecDeque::from(vec![position]);
        let mut cluster_size = 0;
        while !open_set.is_empty() {
            let current_position = open_set.pop_front().unwrap();
            if visited.contains(&current_position) {
                continue;
            } else {
                visited.insert(current_position);
                cluster_size += 1;
                // Add neighbors to open set
                let possible_neighbors = vec![
                    (current_position.0 + 1, current_position.1 + 1),
                    (current_position.0 + 1, current_position.1 - 1),
                    (current_position.0 - 1, current_position.1 + 1),
                    (current_position.0 - 1, current_position.1 - 1),
                ];
                for possible_neighbor in possible_neighbors {
                    if positions.contains(&possible_neighbor)
                        && !visited.contains(&possible_neighbor)
                    {
                        open_set.push_back(possible_neighbor);
                    }
                }
            }
        }
        largest_cluster = max(largest_cluster, cluster_size);
    }

    largest_cluster
}

#[allow(dead_code)]
fn visualize(robot_positions: &HashSet<Position>, width: i32, height: i32) {
    for y in 0..height {
        for x in 0..width {
            let has_pos_robot = robot_positions.contains(&((x, y)));
            if has_pos_robot {
                print!("\x1b[38;2;76;175;80m#\x1b[0m");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn parse_puzzle(input: &str) -> Puzzle {
    let mut robots: Vec<Robot> = vec![];
    for line in input.lines() {
        let (pos, vel) = line.split_once(" ").unwrap();
        let pos = pos.strip_prefix("p=").unwrap().split_once(",").unwrap();
        let vel = vel.strip_prefix("v=").unwrap().split_once(",").unwrap();
        let robot: Robot = (
            (pos.0.parse::<i32>().unwrap(), pos.1.parse::<i32>().unwrap()),
            (vel.0.parse::<i32>().unwrap(), vel.1.parse::<i32>().unwrap()),
        );
        robots.push(robot);
    }
    robots
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    #[test]
    fn test_solve_part_one() {
        let input = fs::read_to_string("./resources/day14/example.txt").unwrap();
        let puzzle = parse_puzzle(&input);
        let solution = solve_part_one(11, 7, &puzzle);
        assert_eq!(solution, "12");
    }
    #[test]
    fn test_solve_part_two() {}
}
