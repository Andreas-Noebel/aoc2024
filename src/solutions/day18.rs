use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};

pub fn solve(input_file_path: &str) -> (String, String) {
    let input = std::fs::read_to_string(input_file_path).unwrap();
    let maze = parse_puzzle(&input, (71, 71));

    let solution_path = solve_part_one(&maze);
    let solution_one = solution_path.len() - 1;

    let solution_two = solve_part_two(&maze);

    (solution_one.to_string(), solution_two)
}

fn solve_part_one(maze: &Maze) -> Vec<(usize, usize)> {
    let mut maze = maze.clone();
    maze.simulate(1024);
    let solution_path = maze
        .shortest_path(maze.get_start_position(), maze.get_end_position())
        .unwrap();
    solution_path
}

fn solve_part_two(maze: &Maze) -> String {
    let mut lower = 0;
    let mut higher = maze.falling_bytes.len();

    while higher - lower > 1 {
        let mid = (higher - lower) / 2 + lower;
        let mut m = maze.clone();
        m.simulate(mid);
        match m.shortest_path(m.get_start_position(), m.get_end_position()) {
            Ok(_) => {
                lower = mid;
            }
            Err(_) => {
                higher = mid;
            }
        }
    }
    let blocking_byte = *maze.falling_bytes.get(lower).unwrap();
    let res = format!("{},{}", blocking_byte.0, blocking_byte.1);
    res
}

#[derive(Clone)]
struct Maze {
    width: usize,
    height: usize,
    falling_bytes: Vec<(u8, u8)>,
    corrupted_grid: Vec<bool>,
}

impl Maze {
    fn new(width: usize, height: usize, falling_bytes: Vec<(u8, u8)>) -> Maze {
        Maze {
            width,
            height,
            falling_bytes,
            corrupted_grid: vec![false; width * height],
        }
    }
    fn is_corrupted(&self, x: usize, y: usize) -> bool {
        self.corrupted_grid[y * self.width + x]
    }

    fn get_start_position(&self) -> (usize, usize) {
        (0, 0)
    }

    fn get_end_position(&self) -> (usize, usize) {
        (self.width - 1, self.height - 1)
    }

    fn simulate(&mut self, ticks: usize) {
        for i in 0..ticks {
            let &(cx, cy) = self.falling_bytes.get(i).unwrap();
            self.corrupted_grid[cy as usize * self.width + cx as usize] = true;
        }
    }

    fn shortest_path(
        &self,
        start: (usize, usize),
        end: (usize, usize),
    ) -> Result<Vec<(usize, usize)>, String> {
        let mut open_set: VecDeque<(usize, usize)> = VecDeque::new();
        let mut closed_set: HashSet<(usize, usize)> = HashSet::new();
        let mut pred: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

        open_set.push_back(start);

        while !open_set.is_empty() {
            let pos @ (x, y) = open_set.pop_front().unwrap();

            if closed_set.contains(&pos) {
                continue;
            }

            closed_set.insert(pos);
            if pos == end {
                // Found end node reconstruct path
                let mut node = pos.clone();
                let mut path: Vec<(usize, usize)> = Vec::new();
                path.push(pos);
                while let Some(pred) = pred.get(&node) {
                    node = pred.clone();
                    path.push(node);
                }
                return Ok(path);
            }

            let n = self.get_neighbors(x, y);
            for neighbor in n {
                if closed_set.contains(&neighbor) {
                    continue;
                }
                open_set.push_back(neighbor);
                pred.insert(neighbor, pos);
            }
        }

        Err("No path found".to_string())
    }

    fn get_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors: Vec<(usize, usize)> = Vec::new();
        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if x < self.width - 1 {
            neighbors.push((x + 1, y));
        }
        if y > 0 {
            neighbors.push((x, y - 1));
        }
        if y < self.height - 1 {
            neighbors.push((x, y + 1));
        }
        neighbors
            .iter()
            .filter(|(nx, ny)| !self.is_corrupted(*nx, *ny))
            .cloned()
            .collect::<Vec<(usize, usize)>>()
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Maze width: {}, height: {}\n", self.width, self.height)?;

        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", if self.is_corrupted(x, y) { "#" } else { "." })?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn parse_puzzle(input: &str, size: (usize, usize)) -> Maze {
    let fb = input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(',').unwrap();
            (l.parse().unwrap(), r.parse().unwrap())
        })
        .collect::<Vec<(u8, u8)>>();
    Maze::new(size.0, size.1, fb)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = std::fs::read_to_string("./resources/day18/example.txt").unwrap();
        let mut maze = parse_puzzle(&input, (7, 7));
        maze.simulate(12);
        let solution_path = maze
            .shortest_path(maze.get_start_position(), maze.get_end_position())
            .unwrap();
        assert_eq!(solution_path.len() - 1, 22);
    }

    #[test]
    fn test_part_two() {
        let input = std::fs::read_to_string("./resources/day18/example.txt").unwrap();
        let maze = parse_puzzle(&input, (7, 7));
        let solution = solve_part_two(&maze);
        assert_eq!(solution, "6,1");
    }
}
