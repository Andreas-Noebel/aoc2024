use std::collections::{HashMap, HashSet};

pub fn solve(input_file_path: &str) -> (String, String) {
    let input = std::fs::read_to_string(input_file_path).unwrap();
    let puzzle = parse_input(&input);
    (solve_part_one(&puzzle), solve_part_two(&puzzle))
}

fn solve_part_one(graph: &Graph) -> String {
    let res = find_k_cliques(graph, 3);
    res.len().to_string()
}

fn solve_part_two(graph: &Graph) -> String {

    for clique_size in 0..graph.0.len() {
        let clique_count = find_k_cliques(graph, clique_size);
        if clique_count.len() == 1 {
            return clique_count[0].join(",");
        }
    }
    "No solution found".to_string()

}

type Node = String;
type Edge = (Node, Node);
type Graph = (HashSet<Node>, HashSet<Edge>, HashMap<Node, HashSet<Node>>);
fn parse_input(input: &str) -> Graph {
    let mut nodes: HashSet<Node> = HashSet::new();
    let mut edges: HashSet<Edge> = HashSet::new();
    let mut neighbours: HashMap<Node, HashSet<Node>> = HashMap::new();

    input.lines().for_each(|line| {
        let (l, r) = line.split_once('-').unwrap();
        let left = l.parse::<Node>().unwrap();
        let right = r.parse::<Node>().unwrap();

        neighbours
            .entry(left.clone())
            .or_insert(HashSet::new())
            .insert(right.clone());

        neighbours
            .entry(right.clone())
            .or_insert(HashSet::new())
            .insert(left.clone());

        nodes.insert(left.clone());
        nodes.insert(right.clone());
        edges.insert((left, right));
    });

    (nodes, edges, neighbours)
}

fn find_k_cliques(graph: &Graph, k: usize) -> Vec<Vec<Node>> {
    let possible_nodes = remove_recursively(&graph.2, (k - 1) as u32);

    let mut result: HashSet<Vec<Node>> = HashSet::new();

    possible_nodes
        .iter()
        .filter(|(n, _)| n.starts_with("t"))
        .for_each(|(s, n)| {
            'combo: for combo_group in Permutes::new(Vec::from_iter(n.clone()), (k - 1) as i32) {

                // Check if other nodes are connected to each other
                for i in 0..combo_group.len() {
                    for j in i + 1..combo_group.len() {
                        let edge: Edge = (combo_group[i].clone(), combo_group[j].clone());
                        let edge_rev: Edge = (edge.1.clone(), edge.0.clone());

                        if !(graph.1.contains(&edge) || graph.1.contains(&edge_rev)) {
                            continue 'combo;
                        }
                    }
                }
                // Combo group is valid
                let mut connected_group: Vec<Node> = Vec::new();
                connected_group.push(s.clone());
                connected_group.append(&mut combo_group.clone());
                connected_group.sort();
                result.insert(connected_group);
            }
        });

    result.into_iter().collect::<Vec<Vec<Node>>>()
}

fn remove_recursively(
    nodes: &HashMap<Node, HashSet<Node>>,
    min_edges: u32,
) -> HashMap<Node, HashSet<Node>> {
    let reduced: HashMap<Node, HashSet<Node>> =
        HashMap::from_iter(nodes.iter().filter_map(|(n, neighbors)| {
            if neighbors.len() >= min_edges as usize {
                Some((n.clone(), neighbors.clone()))
            } else {
                None
            }
        }));

    reduced
}

struct Permutes<T>
where
    T: Clone,
{
    data: Vec<T>,
    bases: Vec<i32>,
    index: i32,
    n: i32,
    k: i32,
}

impl<T> Permutes<T>
where
    T: Clone,
{
    fn new(data: Vec<T>, combinations_length: i32) -> Self {
        Permutes {
            data: data.clone(),
            bases: vec![0; combinations_length as usize],
            index: 0,
            n: combinations_length,
            k: data.len() as i32,
        }
    }
}

impl<T: Clone> Iterator for Permutes<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < 0 {
            return None;
        }

        let output = &self.bases.clone();

        self.index = &self.n - 1;

        while self.index >= 0 {
            self.bases[self.index as usize] += 1;

            if self.index == 0 {
                if self.bases[self.index as usize] < self.k {
                    break;
                }
            } else {
                if self.bases[self.index as usize] < self.bases[self.index as usize - 1] {
                    break;
                }
            }

            self.bases[self.index as usize] = 0;
            self.index -= 1;
        }

        let mut unique_values = output.clone();
        unique_values.sort();
        unique_values.dedup();

        if unique_values.len() != output.len() {
            self.next()
        } else {
            let o = output
                .iter()
                .map(|x| self.data[*x as usize].clone())
                .collect::<Vec<T>>();
            Some(o)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::day23::{find_k_cliques, parse_input};

    #[test]
    fn test_solve_part_one() {
        let input = std::fs::read_to_string("./resources/day23/input.txt").unwrap();
        let puzzle = parse_input(&input);

        let res = find_k_cliques(&puzzle, 3);
        println!("{:?}", res.len());

        let res = find_k_cliques(&puzzle, 13);
        println!("{:?}", res);


        // 2495 too high
        // 2452
    }
}
