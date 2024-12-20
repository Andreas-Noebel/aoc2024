use std::cmp::min;
use std::collections::{HashSet, VecDeque};
use std::fmt::Display;
use std::iter;
use std::iter::{repeat, zip};

pub fn solve(input_file_path: &str) -> (String, String) {
    ("".to_string(), "".to_string())
}

struct Maze {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

type Position = (usize, usize);
type Costs = usize;

impl Maze {
    fn get_start_pos(&self) -> (usize, usize) {
        let start_index = self.data.iter().position(|&x| x == b'S').unwrap_or(0);
        (start_index % self.width, start_index / self.width)
    }

    fn get_end_pos(&self) -> (usize, usize) {
        let end_index = self.data.iter().position(|&x| x == b'E').unwrap_or(0);
        (end_index % self.width, end_index / self.width)
    }

    fn shortest_path(&self) -> Result<usize, String> {
        let mut open_set: VecDeque<(usize, usize)> = VecDeque::new();
        let mut closed_set = HashSet::new();
        let mut predecessors: Vec<Option<(usize, usize)>> = vec![None; self.data.len()];
        let mut costs_to_end: Vec<usize> = vec![usize::MAX; self.data.len()];

        let start_pos = self.get_start_pos();
        let end_pos = self.get_end_pos();

        //open_set.push_back(start_pos);
        open_set.push_back(end_pos);
        costs_to_end[end_pos.1 * self.width + end_pos.0] = 0;

        while !open_set.is_empty() {
            let current_pos = open_set.pop_front().unwrap();
            let current_cost = costs_to_end[current_pos.1 * self.width + current_pos.0];
            if closed_set.contains(&current_pos) {
                continue;
            }
            closed_set.insert(current_pos);

            /*
            if current_pos == end_pos {
                //
                continue;
                // reconstruct path
                let mut path_length = 0;
                let mut node = current_pos;
                while let Some(prev_node) = predecessors[node.1 * self.width + node.0] {
                    path_length += 1;
                    node = prev_node;
                }
                return Ok(path_length)
            }*/

            let neighbors = vec![
                (current_pos.0 - 1, current_pos.1),
                (current_pos.0 + 1, current_pos.1),
                (current_pos.0, current_pos.1 - 1),
                (current_pos.0, current_pos.1 + 1),
            ];

            neighbors
                .iter()
                .filter(|&&pos| self.is_position_free(pos))
                .filter(|&&pos| !closed_set.contains(&pos))
                .for_each(|&neighbor| {
                    predecessors[neighbor.1 * self.width + neighbor.0] = Some(current_pos);
                    costs_to_end[neighbor.1 * self.width + neighbor.0] = current_cost + 1;
                    open_set.push_back(neighbor);
                });
        }
        println!("Bing");

        // Calculate Costs

        // Visualise costs
        /*
        for y in 0..self.height {
            for x in 0..self.width {
                let c = costs_to_end[y * self.width + x];
                match c {
                    usize::MAX => print!("∞ "),
                    _ => print!("{} ", c),
                }
            }
            println!();
        }*/

        let costs_to_enf_from_start = costs_to_end[start_pos.0 * self.width + start_pos.1];
        println!("Costs {:?}", costs_to_enf_from_start);

        let mut found_shortcuts: HashSet<(Position, Position)> = HashSet::new();
        for y in 1..self.height - 1 {
            for x in 1..self.width - 1 {
                let shortcuts = vec![
                    ((x - 1, y), (x + 1, y)),
                    ((x, y - 1), (x, y + 1)),
                    ((x - 1, y - 1), (x + 1, y + 1)),
                ];
                for shortcut @ (a, b) in shortcuts {
                    let costs_a = costs_to_end[a.1 * self.width + a.0];
                    let costs_b = costs_to_end[b.1 * self.width + b.0];
                    if costs_a > costs_to_enf_from_start
                        || costs_b > costs_to_enf_from_start
                        || costs_b.abs_diff(costs_a) <= 2
                    {
                        continue;
                    }
                    if costs_a < costs_b {
                        found_shortcuts.insert((b, a));
                    } else {
                        found_shortcuts.insert((a, b));
                    }
                }
            }
        }
        println!("Shortcuts {:?}", found_shortcuts);

        Err("No path found.".to_string())
    }

    fn get_costs_to_position(&self, target_position: &Position) -> Result<Vec<Costs>, String> {
        let mut open_set: VecDeque<(usize, usize)> = VecDeque::new();
        let mut closed_set = HashSet::new();
        let mut costs_to_target: Vec<usize> = vec![usize::MAX; self.data.len()];

        open_set.push_back(*target_position);
        costs_to_target[target_position.1 * self.width + target_position.0] = 0;

        while !open_set.is_empty() {
            let current_pos = open_set.pop_front().unwrap();
            let current_cost = costs_to_target[current_pos.1 * self.width + current_pos.0];
            if closed_set.contains(&current_pos) {
                continue;
            }
            closed_set.insert(current_pos);

            let neighbors = vec![
                (current_pos.0 - 1, current_pos.1),
                (current_pos.0 + 1, current_pos.1),
                (current_pos.0, current_pos.1 - 1),
                (current_pos.0, current_pos.1 + 1),
            ];

            neighbors
                .iter()
                .filter(|&&pos| self.is_position_free(pos))
                .filter(|&&pos| !closed_set.contains(&pos))
                .for_each(|&neighbor| {
                    costs_to_target[neighbor.1 * self.width + neighbor.0] = current_cost + 1;
                    open_set.push_back(neighbor);
                });
        }
        Ok(costs_to_target)
    }

    fn get_shortcuts(&self) -> Vec<(Position, Costs, Position)> {
        let costs_to_end = self.get_costs_to_position(&self.get_end_pos()).unwrap();
        let costs_to_start = self.get_costs_to_position(&self.get_start_pos()).unwrap();

        let start_pos = self.get_start_pos();
        let solution_without_shortcuts = costs_to_end[start_pos.1 * self.width + start_pos.0];
        println!("{}", solution_without_shortcuts);

        let mut cheats: Vec<(Position, Costs, Position)> = Vec::new();
        for y in 1..self.height - 1 {
            for x in 1..self.width - 1 {
                if self.is_position_free((x, y)) {
                    continue;
                }


                let shortcuts = vec![
                    ((x - 1, y), (x + 1, y)),
                    ((x, y - 1), (x, y + 1)),
                    //((x - 1, y - 1), (x + 1, y + 1)),
                    //((x - 1, y + 1), (x + 1, y - 1)),
                    // diagonal moves not included
                ];

                //let shortcut_end_pos = self.get_cheat_end_positions((x,y),2);
                //let shortcuts = zip(repeat((x,y)), shortcut_end_pos);


                for (a, b) in shortcuts {
                    if !self.is_position_free(a) || !self.is_position_free(b) {
                        continue;
                    }

                    let cost_a_to_start = costs_to_start[a.1 * self.width + a.0];
                    let cost_b_to_start = costs_to_start[b.1 * self.width + b.0];
                    let cost_a_to_end = costs_to_end[a.1 * self.width + a.0];
                    let cost_b_to_end = costs_to_end[b.1 * self.width + b.0];

                    let min_costs_to_start = min(cost_a_to_start, cost_b_to_start);
                    let min_costs_to_end = min(cost_a_to_end, cost_b_to_end);

                    let discounted_path_costs = min_costs_to_end + min_costs_to_start + 2;
                    if discounted_path_costs >= solution_without_shortcuts {
                        continue;
                    }
                    let discount = solution_without_shortcuts - discounted_path_costs;

                    if cost_a_to_end < cost_b_to_end {
                        cheats.push((a, discount, b))
                    } else {
                        cheats.push((b, discount, a))
                    }
                }
            }
        }
        cheats
    }

    fn get_cheat_end_positions(&self, start_pos @ (sx, sy): Position, steps: u32) -> Vec<Position> {
        let mut cheat_end_positions: Vec<Position> = Vec::new();
        let sx = sx as i32;
        let sy = sy as i32;
        let steps = steps as i32;

        for y in 1..steps + 1{
            for x in 1..steps + 1 {
                if x + y > steps {
                    continue;
                }
                let step_end_pos = vec![
                    (sx + x, sy + y),
                    (sx + x, sy - y),
                    (sx - x, sy + y),
                    (sx - x, sy - y),
                ];
                let step_end_pos = step_end_pos
                    .iter()
                    .filter(|(px, py)| {
                        px >= &0
                            && py >= &0
                            && px < &(self.width as i32)
                            && py < &(self.height as i32)
                    })
                    .filter(|(px, py)| {
                        let p = (*px as usize, *py as usize);
                        self.is_position_free(p)
                    })
                    .map(|(px, py)| (*px as usize, *py as usize))
                    .collect::<Vec<Position>>();
                cheat_end_positions.extend(step_end_pos);
            }
        }
        cheat_end_positions
    }

    fn is_position_free(&self, (x, y): (usize, usize)) -> bool {
        self.data[y * self.width + x] != b'#'
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "Width: {} , Height: {}", self.width, self.height,)
    }
}

fn parse(input: &str) -> Maze {
    let width = input.lines().next().unwrap().chars().count();
    let height = input.lines().count();

    let flat_input = input.replace("\n", "").replace("\r", "");

    Maze {
        width,
        height,
        data: flat_input.as_bytes().to_vec(),
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::day20::parse;

    #[test]
    fn test_parse() {
        let input = std::fs::read_to_string("./resources/day20/example.txt").unwrap();
        let puzzle = parse(&input);

        assert_eq!(puzzle.get_start_pos(), (1, 3));
        assert_eq!(puzzle.get_end_pos(), (5, 7));
        assert_eq!(puzzle.width, 15);
        assert_eq!(puzzle.height, 15);
    }

    #[test]
    fn test_part_one() {
        let input = std::fs::read_to_string("./resources/day20/input.txt").unwrap();
        let puzzle = parse(&input);
        let cheats = puzzle.get_shortcuts();
        println!("Cheats: {:#?}", cheats.len());
        println!("{}", cheats.iter().filter(|(_, c, _)| *c >= 100).count());
    }
}
