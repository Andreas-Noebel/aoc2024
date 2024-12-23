use crate::solutions::day21::AbstractKey::{Accept, ArrowBottom, ArrowRight, ArrowLeft, ArrowTop, Number};
use std::cmp::min;
use std::collections::HashMap;

pub fn solve(input_file_path: &str) -> (String, String) {

    let input = std::fs::read_to_string(input_file_path).unwrap();
    let sequences = parse_input(&input);
    let part_one = calc_costs_for_seq(&sequences, 2);
    let part_two = calc_costs_for_seq(&sequences, 25);

    (part_one.to_string(), part_two.to_string())
}

type Key = AbstractKey;
type Position = (i8, i8);
type Costs = u64;
type Action = Key;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
enum AbstractKey {
    Number(u8),
    Accept,
    ArrowRight,
    ArrowTop,
    ArrowBottom,
    ArrowLeft,
}

struct AbstractKeyPad {
    keys: Vec<(Position, Key)>,
}

impl AbstractKeyPad {
    pub fn new(keys: Vec<(Position, Key)>) -> AbstractKeyPad {
        AbstractKeyPad { keys }
    }

    #[rustfmt::skip]
    fn new_numpad() -> AbstractKeyPad {
        let pad = AbstractKeyPad::new(vec![
            ((0, 3), Number(7)),((1, 3), Number(8)),((2, 3), Number(9)),
            ((0, 2), Number(4)),((1, 2), Number(5)),((2, 2), Number(6)),
            ((0, 1), Number(1)),((1, 1), Number(2)),((2, 1), Number(3)),
                                ((1, 0), Number(0)),((2, 0), Accept),
        ]);
        pad
    }

    #[rustfmt::skip]
    fn new_direction_pad() -> AbstractKeyPad {
        let pad = AbstractKeyPad::new(vec![
                                ((1, 1), ArrowTop),   ((2, 1), Accept),
            ((0, 0), ArrowLeft),((1, 0), ArrowBottom),((2, 0), ArrowRight),
        ]);
        pad
    }

    fn get_sequences_to_enter_key(&self, key_to_press: &Key, start_key: &Key) -> Vec<Vec<Action>> {
        let start_pos = self.get_position_of_key(start_key).unwrap();
        let end_pos = self.get_position_of_key(key_to_press).unwrap();

        let (dx, dy) = (end_pos.0 - start_pos.0, end_pos.1 - start_pos.1);

        let horizontal = if dx > 0 {
            vec![ArrowRight; dx.abs() as usize]
        } else {
            vec![ArrowLeft; dx.abs() as usize]
        };

        let vertical = if dy < 0 {
            vec![ArrowBottom; dy.abs() as usize]
        } else {
            vec![ArrowTop; dy.abs() as usize]
        };

        let can_move_horizontal = self.get_key_at_pos(&(end_pos.0, start_pos.1)).is_some();
        let can_move_vertical = self.get_key_at_pos(&(start_pos.0, end_pos.1)).is_some();

        let mut output: Vec<Vec<Action>> = Vec::new();

        if can_move_horizontal {
            let mut pos_seq_1 = horizontal.clone();
            pos_seq_1.append(&mut vertical.clone());
            pos_seq_1.push(Accept);
            output.push(pos_seq_1);
        }

        if can_move_vertical {
            let mut pos_seq_2 = vertical.clone();
            pos_seq_2.append(&mut horizontal.clone());
            pos_seq_2.push(Accept);
            output.push(pos_seq_2);
        }

        output
    }

    fn get_position_of_key(&self, key: &Key) -> Option<Position> {
        self.keys.iter().find(|&&(_, k)| k == *key).map(|&(p, _)| p)
    }

    fn get_key_at_pos(&self, pos: &Position) -> Option<Key> {
        self.keys.iter().find(|&&(p, _)| p == *pos).map(|&(_, k)| k)
    }
}

fn get_costs_recursive(
    keypads: &Vec<&AbstractKeyPad>,
    seq: Vec<Key>,
    depth: u8,
    cache: &mut HashMap<(Vec<Key>, u8), Costs>,
) -> Costs {
    if let Some(x) = cache.get(&(seq.clone(), depth)) {
        return *x;
    }

    let pad = keypads.get(depth as usize).unwrap();
    let mut total_sequence_costs: Costs = 0;
    if depth == (keypads.len() - 1) as u8 {
        let mut current_key = Accept;
        for key in seq.iter() {
            let c = pad.get_sequences_to_enter_key(&key, &current_key);
            total_sequence_costs += c.get(0).unwrap().len() as Costs;
            current_key = *key
        }
    } else {
        let mut current_key: Key = Accept;
        for key in seq.iter() {
            let mut min_costs = u64::MAX;
            for possible_sequence in pad.get_sequences_to_enter_key(key, &current_key) {
                let costs = get_costs_recursive(keypads, possible_sequence, depth + 1, cache);
                min_costs = min(costs, min_costs);
            }

            total_sequence_costs += min_costs;
            current_key = *key
        }
    }
    cache.insert((seq.clone(), depth), total_sequence_costs);
    total_sequence_costs
}

fn parse_input(input: &str) -> Vec<(Vec<Key>,u64)> {
    let keys = input
        .lines()
        .map(|line| {
            let key_presses = line.chars()
                .filter_map(|c| match c {
                    'A' => Some(Accept),
                    x => {
                        if x.is_digit(10) {
                            Some(Number(x.to_digit(10).unwrap() as u8))
                        } else {
                            None
                        }
                    }
                })
                .collect::<Vec<Key>>();
            let number = line.replace("A", "").parse::<u64>().unwrap();
            (key_presses, number)
        })
        .collect::<Vec<(Vec<Key>, u64)>>();
    keys
}

fn calc_costs_for_seq(sequences: &Vec<(Vec<Key>, u64)>, dir_pads: u8) -> Costs{
    let num_pad = AbstractKeyPad::new_numpad();
    let dir_pad = AbstractKeyPad::new_direction_pad();

    let mut pad_chain: Vec<&AbstractKeyPad> = vec![];
    pad_chain.append(&mut vec![&num_pad]);
    pad_chain.append(&mut vec![&dir_pad; dir_pads as usize]);

    let mut total_cost = 0;
    let mut cache: HashMap<(Vec<Key>, u8), Costs> = HashMap::new();
    for (sequence, number) in sequences {
        let c = get_costs_recursive(&pad_chain, sequence.clone(), 0, &mut cache);
        total_cost += c * number;
    }
    total_cost

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {

        let input = std::fs::read_to_string("./resources/day21/example.txt").unwrap();
        let sequences = parse_input(&input);
        let part_one = calc_costs_for_seq(&sequences, 2);
        assert_eq!(part_one.to_string(), "126384")
    }
}
