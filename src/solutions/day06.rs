use std::collections::{HashMap, HashSet};

type Obstacles = HashSet<(i32, i32)>;
type Position = (i32, i32);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}
pub fn solve(puzzle_input_path: &str) -> (String, String) {
    let input = std::fs::read_to_string(puzzle_input_path).unwrap();
    let (width, height, position, obstacles) = parse_puzzle(&input);
    let solution_one = solve_part_one(width, height, &position, &obstacles)
        .unwrap()
        .to_string();
    let solution_two = solve_part_two(width, height, &position, &obstacles)
        .unwrap()
        .to_string();
    (solution_one, solution_two)
}

fn solve_part_one(
    width: i32,
    height: i32,
    guard_start_position: &Position,
    obstacles: &Obstacles,
) -> Result<i32, String> {
    match simulate_guard(width, height, &guard_start_position, &obstacles) {
        Ok(solution) => Ok(solution.keys().len() as i32),
        Err(error) => Err(error.to_string()),
    }
}

fn simulate_guard(
    width: i32,
    height: i32,
    guard_start_position: &Position,
    obstacles: &Obstacles,
) -> Result<HashMap<Position, HashSet<Direction>>, String> {
    let mut current_guard_position = guard_start_position.clone();
    let mut current_direction = Direction::North;
    let mut visited: HashMap<Position, HashSet<Direction>> = HashMap::new();

    loop {
        let guard_x = current_guard_position.0;
        let guard_y = current_guard_position.1;

        // Check if guard has left map
        if guard_x < 0 || guard_x >= width || guard_y < 0 || guard_y >= height {
            break;
        }

        visited
            .entry(current_guard_position)
            .or_default()
            .insert(current_direction);

        let next_guard_position = match current_direction {
            Direction::North => (guard_x, guard_y - 1),
            Direction::East => (guard_x + 1, guard_y),
            Direction::South => (guard_x, guard_y + 1),
            Direction::West => (guard_x - 1, guard_y),
        };

        if obstacles.contains(&next_guard_position) {
            let next_direction = match current_direction {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            };
            current_direction = next_direction;
        } else {
            current_guard_position = next_guard_position;
        }

        // Check for Loop
        if visited.contains_key(&current_guard_position) {
            if visited[&current_guard_position].contains(&current_direction) {
                return Err(format!(
                    "Loop detected at {:?} facing {:?}",
                    current_guard_position, current_direction
                ));
            }
        }
    }
    Ok(visited)
}
fn solve_part_two(
    width: i32,
    height: i32,
    guard_position: &Position,
    obstacles: &Obstacles,
) -> Result<i32, String> {
    let mut obstacles_extended = obstacles.clone();
    let mut possible_looping_obstacles = 0;

    let default_path = simulate_guard(width, height, &guard_position, &obstacles)?;

    for test_obstacle in default_path.keys() {
        if &guard_position == &test_obstacle {
            continue;
        }
        obstacles_extended.insert(test_obstacle.clone());

        match simulate_guard(width, height, &guard_position, &obstacles_extended) {
            Ok(_) => {}
            Err(_) => {
                possible_looping_obstacles += 1;
            }
        }
        obstacles_extended.remove(&test_obstacle);
    }

    Ok(possible_looping_obstacles)
}

fn parse_puzzle(input: &str) -> (i32, i32, Position, Obstacles) {
    let width = (input.find('\n').unwrap() - 1) as i32;
    let height = input.len() as i32 / (width + 1);

    let mut obstacles: Obstacles = HashSet::new();
    let mut guard_position: (i32, i32) = (-1, -1);
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, ch)| {
            let x = x as i32;
            let y = y as i32;
            match ch {
                '#' => {
                    obstacles.insert((x, y));
                }
                '^' | '<' | '>' | 'v' => {
                    guard_position = (x, y);
                }
                _ => {}
            }
        })
    });
    (width, height, guard_position, obstacles)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = std::fs::read_to_string("./resources/day06/example.txt").unwrap();
        let (width, height, position, obstacles) = parse_puzzle(&input);
        let s = solve_part_one(width, height, &position, &obstacles);
        assert_eq!(s.unwrap().to_string(), "41")
    }

    #[test]
    fn test_part_two() {
        let input = std::fs::read_to_string("./resources/day06/example.txt").unwrap();
        let (width, height, position, obstacles) = parse_puzzle(&input);
        let s = solve_part_two(width, height, &position, &obstacles);
        assert_eq!(s.unwrap().to_string(), "6")
    }
}
