use std::collections::{HashMap, VecDeque};

type Dimension = (usize, usize);
type Position = (i32, i32);
type Puzzle = (Dimension, Vec<u8>);
pub fn solve(input_file_path: &str) -> (String, String) {
    let input = std::fs::read_to_string(input_file_path).unwrap();
    let puzzle = parse_puzzle(&input);
    let (solution_part_one, solution_part_two) = solve_both_parts(&puzzle);

    (solution_part_one, solution_part_two)
}

fn parse_puzzle(input: &str) -> Puzzle {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let data = input
        .replace('\r', "")
        .replace('\n', "")
        .as_bytes()
        .to_vec();
    ((width, height), data)
}

fn solve_both_parts(((width, height), data): &Puzzle) -> (String, String) {
    let w = *width as i32;
    let h = *height as i32;

    let mut visited: Vec<bool> = vec![false; (w * h) as usize];

    let mut price = 0;
    let mut price_discount = 0;

    for y in 0..*width {
        for x in 0..*height {
            if visited[y * width + x] {
                continue;
            }

            let mut neighbors: VecDeque<Position> = VecDeque::new();
            neighbors.push_back((x as i32, y as i32));

            let plant_type = data[y * width + x];
            let mut area = 0;
            let mut perimeter = 0;
            let mut plants_in_area: Vec<Position> = Vec::new();

            while !neighbors.is_empty() {
                let (node_x, node_y) = neighbors.pop_front().unwrap();

                if visited[(node_y * w + node_x) as usize] {
                    continue;
                }

                visited[(node_y * w + node_x) as usize] = true;
                plants_in_area.push((node_x, node_y));
                area += 1;
                perimeter += 4;

                let possible_neighbors = vec![
                    (node_x, node_y - 1),
                    (node_x + 1, node_y),
                    (node_x, node_y + 1),
                    (node_x - 1, node_y),
                ];

                for (n_x, n_y) in possible_neighbors {
                    if n_x >= w || n_x < 0 || n_y < 0 || n_y >= h {
                        continue;
                    }

                    let pos_as_index = (n_y * w + n_x) as usize;
                    let is_same_plant = data[pos_as_index] == plant_type;
                    let is_visited = visited[pos_as_index];

                    if is_same_plant {
                        perimeter -= 1;

                        if !is_visited {
                            neighbors.push_back((n_x, n_y));
                        }
                    }
                }
            }
            let unique_edges = count_unique_edges(plants_in_area);
            price += area * perimeter;
            price_discount += area * unique_edges;
        }
    }

    (price.to_string(), price_discount.to_string())
}

fn count_unique_edges(area: Vec<Position>) -> i32 {
    let directions = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut unique_edges = 0;
    for (dx, dy) in directions {
        let edges_in_direction = area
            .iter()
            .filter(|(x, y)| !area.contains(&(x + dx, y + dy)))
            .collect::<Vec<&Position>>();

        let group_horizontal = dx == 0;

        let mut grouped_by_merge_axis: HashMap<i32, Vec<i32>> = HashMap::new();
        for (e_x, e_y) in edges_in_direction {
            let key = if group_horizontal { *e_y } else { *e_x };
            let value = if group_horizontal { e_x } else { e_y };

            let entry = grouped_by_merge_axis.entry(key).or_insert(Vec::new());
            entry.push(*value);
        }

        let mut unique_edges_in_direction = 0;
        for sweep in grouped_by_merge_axis.values() {
            let sweep_copy = sweep.clone();
            unique_edges_in_direction += count_intervals(sweep_copy);
        }
        unique_edges += unique_edges_in_direction;
    }

    unique_edges
}

fn count_intervals(mut input: Vec<i32>) -> i32 {
    input.sort();
    if input.len() <= 1 {
        input.len() as i32
    } else {
        let mut intervals = 1;
        for i in 0..input.len() - 1 {
            if (input[i + 1] - input[i]).abs() != 1 {
                intervals += 1;
            }
        }
        intervals
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = std::fs::read_to_string("./resources/day12/example.txt").unwrap();
        let ((width, height), _) = parse_puzzle(&input);
        assert_eq!(width, 10);
        assert_eq!(height, 10);
    }

    #[test]
    fn test_solve_part_one() {
        let input = std::fs::read_to_string("./resources/day12/example.txt").unwrap();
        let puzzle = parse_puzzle(&input);
        let (solution_one, solution_two) = solve_both_parts(&puzzle);
        assert_eq!(solution_one, "1930");
        assert_eq!(solution_two, "1206");
    }

    #[test]
    fn test_interval_counter() {
        let output = count_intervals(vec![1, 2, 3]);
        assert_eq!(output, 1);

        let output = count_intervals(vec![1, 3]);
        assert_eq!(output, 2);
    }

    #[test]
    #[rustfmt::skip]
    fn test_edge_detector() {

        // Check single filed
        let input: Vec<Position> = vec![
            (0, 0)
        ];
        let solution = count_unique_edges(input);
        assert_eq!(solution, 4);

        // Check field strip
        let input: Vec<Position> = vec![
            (0, 0), (1, 0), (2, 0)
        ];
        let solution = count_unique_edges(input);
        assert_eq!(solution, 4);

        // Check rectangular field
        let input: Vec<Position> = vec![
            (0, 0),(1, 0),(2, 0),
            (0, 1),(1, 1),(2, 1),
            (0, 2),(1, 2),(2, 2),
        ];
        let solution = count_unique_edges(input);
        assert_eq!(solution, 4);

        // Check right L field
        let input: Vec<Position> = vec![
            (0, 0), (1, 0), (2, 0),
                            (2, 1),
                            (2, 2)
        ];
        let solution = count_unique_edges(input);
        assert_eq!(solution, 6);

        // Check left L field
        let input: Vec<Position> = vec![
            (0, 0), (1, 0), (2, 0),
            (0, 1),
            (0, 2)
        ];
        let solution = count_unique_edges(input);
        assert_eq!(solution, 6);

        // Check complex shape
        let input:Vec<Position> = vec![
            (0,0),(1,0),(2,0),(3,0),
            (0,1),
            (0,2),(1,2),(2,2),(3,2),
            (0,3),
            (0,4),(1,4),(2,4),(3,4),
        ];
        let solution = count_unique_edges(input);
        assert_eq!(solution, 12);

        // Check field with holes
        let input: Vec<Position> = vec![
            (0, 0),(1, 0),(2, 0),
            (0, 1),       (2, 1),
            (0, 2),(1, 2),(2, 2),
        ];
        let solution = count_unique_edges(input);
        assert_eq!(solution, 8);
    }
}
