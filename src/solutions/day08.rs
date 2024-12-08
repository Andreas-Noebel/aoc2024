use std::collections::{HashMap, HashSet};
type FieldSize = (i32, i32);
type Position = (i32, i32);
type Signal = char;
type Antennas = HashMap<Signal, Vec<Position>>;
pub fn solve(input_file_path: &str) -> (String, String) {
    let input = std::fs::read_to_string(input_file_path).unwrap();
    let (grid_size, an) = parse_puzzle(&input);

    let solution_part_one = solve_part_one(grid_size, &an).to_string();
    let solution_part_two = solve_part_two(grid_size, &an).to_string();
    (solution_part_one, solution_part_two)
}

fn solve_part_one((width, height): FieldSize, antennas: &Antennas) -> i32 {
    let mut anti_nodes_all: HashSet<Position> = HashSet::new();
    let out_of_bound = |(x, y): &Position| -> bool {
        return *x < 0 || *y < 0 || *x >= width || *y >= height;
    };

    for (_, positions) in antennas {
        for i in 0..positions.len() - 1 {
            for j in (i + 1)..positions.len() {
                let anti_nodes_signal =
                    get_anti_nodes(positions[i], positions[j], &out_of_bound);
                anti_nodes_all.extend(anti_nodes_signal);
            }
        }
    }

    anti_nodes_all.len() as i32
}

fn solve_part_two((width, height): FieldSize, antennas: &Antennas) -> i32 {

    // Code duplication but im too lazy ¯\_(ツ)_/¯
    let mut anti_nodes_all: HashSet<Position> = HashSet::new();
    let out_of_bound = |(x, y): &Position| -> bool {
        return *x < 0 || *y < 0 || *x >= width || *y >= height;
    };

    for (_, positions) in antennas {
        for i in 0..positions.len() - 1 {
            for j in (i + 1)..positions.len() {
                let anti_nodes_signal =
                    get_anti_nodes(positions[i], positions[j], &out_of_bound);
                let anti_nodes_resonance =
                    get_anti_nodes_resonance(positions[i], positions[j], &out_of_bound);
                anti_nodes_all.extend(anti_nodes_signal);
                anti_nodes_all.extend(anti_nodes_resonance);
            }
        }
    }

    anti_nodes_all.len() as i32
}

fn get_anti_nodes(
    antenna_1: Position,
    antenna_2: Position,
    out_of_bound: impl Fn(&Position) -> bool,
) -> HashSet<Position> {
    let (a1_x, a1_y) = antenna_1;
    let (a2_x, a2_y) = antenna_2;
    let mut positions: HashSet<Position> = HashSet::new();
    let (diff_x, diff_y) = (antenna_2.0 - antenna_1.0, antenna_2.1 - antenna_1.1);

    // Check and add inner anti_nodes
    if diff_x % 3 == 0 && diff_y % 3 == 0 {
        let (diff_x, diff_y) = (diff_x / 3, diff_y / 3);
        let anti_node_1 = (a1_x + diff_x, a1_y + diff_y);
        let anti_node_2 = (a2_x - diff_x, a2_y - diff_y);
        positions.insert(anti_node_1);
        positions.insert(anti_node_2);
    }
    // Add outer anti_nodes
    let anti_node_3 = (a2_x + diff_x, a2_y + diff_y);
    let anti_node_4 = (a1_x - diff_x, a1_y - diff_y);
    positions.insert(anti_node_3);
    positions.insert(anti_node_4);

    positions.retain(|pos| !out_of_bound(pos));
    positions
}

fn get_anti_nodes_resonance(
    antenna_1: Position,
    antenna_2: Position,
    out_of_bound: impl Fn(&Position) -> bool,
) -> HashSet<Position> {
    let mut positions: HashSet<Position> = HashSet::new();
    let (diff_x, diff_y) = (antenna_2.0 - antenna_1.0, antenna_2.1 - antenna_1.1);

    positions.insert(antenna_1);
    positions.insert(antenna_2);

    let mut resonate_pos = antenna_1;
    loop {
        resonate_pos = (resonate_pos.0 + diff_x, resonate_pos.1 + diff_y);
        if out_of_bound(&resonate_pos) {
            break;
        }
        positions.insert(resonate_pos);
    }

    let mut resonate_pos = antenna_2;
    loop {
        resonate_pos = (resonate_pos.0 - diff_x, resonate_pos.1 - diff_y);
        if out_of_bound(&resonate_pos) {
            break;
        }
        positions.insert(resonate_pos);
    }
    positions
}

fn parse_puzzle(input: &str) -> (FieldSize, Antennas) {
    let width = input.lines().nth(0).unwrap().len() as i32;
    let height = input.lines().count() as i32;
    let mut antennas: Antennas = HashMap::new();

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, ch)| match ch {
            '.' => {}
            _ => {
                let positions: &mut Vec<Position> = antennas.entry(ch).or_insert(vec![]);
                positions.push((x as i32, y as i32));
            }
        });
    });
    ((width, height), antennas)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = std::fs::read_to_string("./resources/day08/example.txt").unwrap();
        let (grid_size, an) = parse_puzzle(&input);
        let solution = solve_part_one(grid_size, &an).to_string();
        assert_eq!(solution, "14");
    }

    #[test]
    fn test_part_two() {
        let input = std::fs::read_to_string("./resources/day08/example.txt").unwrap();
        let (grid_size, an) = parse_puzzle(&input);
        let solution = solve_part_two(grid_size, &an).to_string();
        assert_eq!(solution, "34");
    }
}
