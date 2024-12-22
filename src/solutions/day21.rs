use crate::solutions::day21::AbstractKey::{Accept, ArrowBottom, ArrowRight, ArrowTop, Number};
use std::cmp::min;
use std::collections::HashMap;
use AbstractKey::ArrowLeft;

pub fn solve() -> (String, String) {
    ("".to_string(), "".to_string())
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

    fn new_numpad() -> AbstractKeyPad {
        let pad = AbstractKeyPad::new(vec![
            ((0, 3), Number(7)),
            ((1, 3), Number(8)),
            ((2, 3), Number(9)),
            ((0, 2), Number(4)),
            ((1, 2), Number(5)),
            ((2, 2), Number(6)),
            ((0, 1), Number(1)),
            ((1, 1), Number(2)),
            ((2, 1), Number(3)),
            ((1, 0), Number(0)),
            ((2, 0), Accept),
        ]);
        pad
    }

    fn new_direction_pad() -> AbstractKeyPad {
        let pad = AbstractKeyPad::new(vec![
            ((1, 1), ArrowTop),
            ((2, 1), Accept),
            ((0, 0), ArrowLeft),
            ((1, 0), ArrowBottom),
            ((2, 0), ArrowRight),
        ]);
        pad
    }

    fn get_sequences_to_enter_key(&self, key_to_press: &Key, start_key: &Key) -> Vec<Vec<Action>> {
        let start_pos = self.get_position_of_key(start_key).unwrap();
        let end_pos = self.get_position_of_key(key_to_press).unwrap();

        let (dx, dy) = (end_pos.0 - start_pos.0, end_pos.1 - start_pos.1);

        let mut horizontal = if dx > 0 {
            vec![ArrowRight; dx.abs() as usize]
        } else {
            vec![ArrowLeft; dx.abs() as usize]
        };

        let mut vertical = if dy < 0 {
            vec![ArrowBottom; dy.abs() as usize]
        } else {
            vec![ArrowTop; dy.abs() as usize]
        };

        let can_move_horizontal = self.get_key_at_pos(&(end_pos.0, start_pos.1)).is_some();
        let can_move_vertical = self.get_key_at_pos(&(start_pos.0, end_pos.1)).is_some();

        let mut output: Vec<Vec<Action>> = Vec::new();

        if can_move_horizontal {
            let mut o = horizontal.clone();
            o.append(&mut vertical.clone());
            o.push(Accept);
            output.push(o);
        }

        if can_move_vertical {
            let mut o = vertical.clone();
            o.append(&mut horizontal.clone());
            o.push(Accept);
            output.push(o);
        }

        output
    }

    fn get_position_of_key(&self, key: &Key) -> Option<Position> {
        for (p, k) in self.keys.clone() {
            if k == *key {
                return Some(p);
            }
        }
        None
    }

    fn get_key_at_pos(&self, pos: &Position) -> Option<Key> {
        for (p, k) in self.keys.clone() {
            if p == *pos {
                return Some(k);
            }
        }
        None
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
        for s in seq.clone() {
            let c = pad.get_sequences_to_enter_key(&s, &current_key);
            total_sequence_costs += c.get(0).unwrap().len() as Costs;
            current_key = s
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
            current_key = key.clone();
        }
    }
    cache.insert((seq.clone(), depth), total_sequence_costs);
    total_sequence_costs
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_numpad_gen() {
        let num_pad = AbstractKeyPad::new_numpad();
        let dir_pad = AbstractKeyPad::new_direction_pad();

        let input = vec!["029A", "980A", "179A", "456A", "379A"];
        let input = vec!["286A", "480A", "140A", "413A", "964A"];
        let input = input
            .iter()
            .map(|seq| {
                let i = seq
                    .chars()
                    .map(|ch| {
                        if ch.is_alphabetic() {
                            return Accept;
                        }
                        let digit = ch.to_digit(10).unwrap() as u8;
                        Number(digit)
                    })
                    .collect::<Vec<Key>>();
                let n = seq.replace("A", "").parse::<u32>().unwrap();
                return (i, n);
            })
            .collect::<Vec<(Vec<Key>, u32)>>();

        let sequences = input;

        let mut num_pad = vec![&num_pad];
        let mut dir_pads = vec![&dir_pad; 25];

        let mut pad: Vec<&AbstractKeyPad> = vec![];
        pad.append(&mut num_pad);
        pad.append(&mut dir_pads);

        let mut total_cost = 0;
        let mut cache: HashMap<(Vec<Key>, u8), Costs> = HashMap::new();
        for (sequence, number) in sequences {
            let c = get_costs_recursive(&pad, sequence.clone(), 0, &mut cache);
            println!("{:?} * {:?}", c, number);
            total_cost += c * number as u64;
        }
        println!("{:?}", total_cost);
    }
}
