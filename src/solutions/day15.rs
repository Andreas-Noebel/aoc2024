use std::collections::{HashMap, HashSet, VecDeque};

type Position = (i32, i32);

#[derive(Debug, Copy, Clone)]
enum Instruction {
    North,
    East,
    South,
    West,
}

#[derive(Clone)]
struct WareHouse {
    width: i32,
    height: i32,
    robot_position: Position,
    walls: HashSet<Position>,
    boxes: HashSet<Position>,
    box_width: usize,
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct WarehouseBox {
    id: i32,
    position: Position,
    width: i32,
    height: i32,
}

impl WarehouseBox {
    fn push(&mut self, direction: Instruction) {
        let (x, y) = self.position;
        let pos = match direction {
            Instruction::North => (x, y - 1),
            Instruction::East => (x + 1, y),
            Instruction::South => (x, y + 1),
            Instruction::West => (x - 1, y),
        };
        self.position = pos;
    }
}

struct AdvancedWareHouse {
    width: i32,
    height: i32,
    robot_position: Position,
    walls: HashSet<Position>,
    boxes: HashMap<i32, WarehouseBox>,
}

impl AdvancedWareHouse {
    fn move_robot(&mut self, instruction: Instruction) {
        let direction = match instruction {
            Instruction::North => (0, -1),
            Instruction::East => (1, 0),
            Instruction::South => (0, 1),
            Instruction::West => (-1, 0),
        };
        let next_position = (
            self.robot_position.0 + direction.0,
            self.robot_position.1 + direction.1,
        );

        // Next pos is a wall do nothing
        if self.walls.contains(&next_position) {
            return;
        }

        // Next pos has a box try to move it

        if self.get_box_at_pos(&next_position).is_some(){
            let b = self.get_box_at_pos(&next_position).unwrap();

            let mut boxes_to_push = vec![];

            if self.push_boxes(&b, &instruction, &mut boxes_to_push) {
                let mut updated_boxes = self.boxes.clone();
                for mut boxes_to_push in boxes_to_push {
                    //println!("Pushing {:?} in {instruction:?}", boxes_to_push);
                    boxes_to_push.push(instruction);
                    //println!("After: {:?} ", boxes_to_push);
                    updated_boxes.insert(boxes_to_push.id, boxes_to_push);
                }
                self.boxes = updated_boxes;
                self.robot_position = next_position;
            }

        }else {
            self.robot_position = next_position;
        }
    }
    fn visualise_state(&self) {
        println!("Robot position: {:?}", self.robot_position);
        for y in 0..self.height {
            for x in 0..self.width {
                let position = (x, y);
                if self.walls.contains(&position) {
                    print!("#");
                    continue;
                }
                if self.robot_position == position {
                    print!("@");
                    continue;
                }
                match self.get_box_at_pos(&position) {
                    Some(b) => print!("+"),
                    None => print!("."),
                }
            }
            println!();
        }
    }

    fn get_box_at_pos(&self, position: &Position) -> Option<&WarehouseBox> {
        for warehouse_box in self.boxes.values() {
            let (x, y) = warehouse_box.position;
            for sx in 0..warehouse_box.width {
                for sy in 0..warehouse_box.height {
                    if *position == (x + sx, y + sy) {
                        return Some(warehouse_box);
                    }
                }
            }
        }
        None
    }

    fn push_boxes(&self, start_box: &WarehouseBox, direction: &Instruction, boxes_to_update: &mut Vec<WarehouseBox>) -> bool {
        let mut boxes_to_push: VecDeque<&WarehouseBox> = VecDeque::new();
        let mut pushable_boxes: HashSet<WarehouseBox> = HashSet::new();

        boxes_to_push.push_back(start_box);
        while !boxes_to_push.is_empty() {
            let mut box_to_check = boxes_to_push.pop_front().unwrap();
            let (bx, by) = box_to_check.position;
            let (bw, bh) = (box_to_check.width, box_to_check.height);
            let positions_to_check = match direction {
                Instruction::North => vec![(bx, by - 1), (bx + 1, by - 1)],
                Instruction::East => vec![(bx + bw, by)],
                Instruction::South => vec![(bx, by + 1), (bx + 1, by + 1)],
                Instruction::West => vec![(bx - 1, by)],
            };

            for position in positions_to_check {
                if self.walls.contains(&position) {
                    return false;
                }
                match self.get_box_at_pos(&position) {
                    Some(b) => {
                        if !pushable_boxes.contains(&b) {
                            boxes_to_push.push_back(b)
                        }
                    }
                    None => {}
                }
            }
            pushable_boxes.insert(*box_to_check);
            boxes_to_update.push(*box_to_check);
        }

        true
    }

    fn sum_of_gps(&self) -> i32 {
        self.boxes.values().map(|b| b.position.1*100+b.position.0).sum()
    }
}

impl WareHouse {
    fn move_robot(&mut self, instruction: Instruction) {
        let direction = match instruction {
            Instruction::North => (0, -1),
            Instruction::East => (1, 0),
            Instruction::South => (0, 1),
            Instruction::West => (-1, 0),
        };
        let next_position = (
            self.robot_position.0 + direction.0,
            self.robot_position.1 + direction.1,
        );

        // Next pos is a wall do nothing
        if self.walls.contains(&next_position) {
            return;
        }

        // Next pos has a box try to move it
        if self.boxes.contains(&next_position) {
            // Check if boxes can be moved
            let mut next_box_position = next_position;
            // Find next free space in direction and move box
            while !self.walls.contains(&next_box_position) {
                // Found free space
                if !self.boxes.contains(&next_box_position) {
                    self.boxes.remove(&next_position);
                    self.boxes.insert(next_box_position);
                    self.robot_position = next_position;
                    return;
                } else {
                    next_box_position = (
                        next_box_position.0 + direction.0,
                        next_box_position.1 + direction.1,
                    );
                }
            }
            return;
        }

        // Next pos has no box and is no wall -> move
        self.robot_position = next_position;
    }

    /*
    fn scale_ware_house(&mut self) {
        // Scale walls
        let mut scaled_walls: HashSet<Position> = HashSet::new();
        self.walls.iter().for_each(|pos| {
            let left = (pos.0 * 2, pos.1);
            let right = (left.0 + 1, left.1);
            scaled_walls.insert(left);
            scaled_walls.insert(right);
        });

        // Scale boxes
        let mut scaled_boxes: HashSet<Position> = HashSet::new();
        self.boxes.iter().for_each(|&(x, y)| {
            scaled_boxes.insert((x * 2, y));
        });

        // Scale robot position
        let scaled_robot_pos = (self.robot_position.0 * 2, self.robot_position.1);

        self.boxes = scaled_boxes;
        self.walls = scaled_walls;
        self.robot_position = scaled_robot_pos;
        self.box_width = 2;
        self.width = self.width * 2;
    }
     */

    fn to_advanced_ware_house(&self) -> AdvancedWareHouse {
        let width = self.width * 2;
        let height = self.height;

        // Scale walls
        let mut scaled_walls: HashSet<Position> = HashSet::new();
        self.walls.iter().for_each(|pos| {
            let left = (pos.0 * 2, pos.1);
            let right = (left.0 + 1, left.1);
            scaled_walls.insert(left);
            scaled_walls.insert(right);
        });

        // Scale robot position
        let scaled_robot_pos = (self.robot_position.0 * 2, self.robot_position.1);

        // Scale boxes
        let mut boxes: HashMap<i32, WarehouseBox> = HashMap::new();
        self.boxes.iter().enumerate().for_each(|(id, pos)| {
            let b = WarehouseBox {
                id: id as i32,
                position: (pos.0 * 2, pos.1),
                width: 2,
                height: 1,
            };
            boxes.insert(id as i32, b);
        });

        AdvancedWareHouse {
            width,
            height,
            robot_position: scaled_robot_pos,
            walls: scaled_walls,
            boxes,
        }
    }

    fn sum_of_gps(&self) -> i32 {
        self.boxes
            .iter()
            .fold(0, |sum, (px, py)| sum + (py * 100 + px))
    }

    #[allow(dead_code)]
    fn visualise_state(&self) {
        println!("Robot position: {:?}", self.robot_position);
        for y in 0..self.height {
            let mut x = 0;
            while x < self.width {
                if self.walls.contains(&(x, y)) {
                    print!("#");
                } else if self.boxes.contains(&(x, y)) {
                    if self.box_width == 1 {
                        print!("0")
                    } else {
                        print!("[]");
                        x += 1;
                    }
                } else if self.robot_position == (x, y) {
                    print!("@");
                } else {
                    print!(".");
                }
                x += 1;
            }
            println!();
        }
    }
}

pub fn solve(input_file_path: &str) -> (String, String) {
    let input = std::fs::read_to_string(input_file_path).unwrap();

    let (w, i) = parse_input(&input);
    let solution_part_one = solve_part_one(w.clone(), &i);
    let solution_part_two = solve_part_two(w.clone(), &i);

    (solution_part_one, solution_part_two)
}

fn solve_part_one(mut ware_house: WareHouse, instructions: &Vec<Instruction>) -> String {
    for &instruction in instructions {
        ware_house.move_robot(instruction);
        //ware_house.visualise_state();
    }
    ware_house.visualise_state();
    let solution = ware_house.sum_of_gps();
    println!("{}", solution);
    solution.to_string()
}

fn solve_part_two(mut ware_house: WareHouse, instructions: &Vec<Instruction>) -> String {
    let mut aw = ware_house.to_advanced_ware_house();
    aw.visualise_state();

    for &instruction in instructions {
       // println!("{:?}", instruction);
        aw.move_robot(instruction);
        //aw.visualise_state();
    }

    let solution = aw.sum_of_gps();
    print!("Final State");
    aw.visualise_state();
    println!("{}", solution);
    solution.to_string()
}

fn parse_input(input: &str) -> (WareHouse, Vec<Instruction>) {
    let width = input.lines().next().unwrap().len() as i32;
    let mut height = 0;

    let mut lines = input.lines();

    let mut walls: HashSet<Position> = HashSet::new();
    let mut boxes: HashSet<Position> = HashSet::new();
    let mut robot_position: Position = (0, 0);

    // Parse map
    loop {
        match lines.next() {
            Some(line) => {
                if line.len() == 0 {
                    break;
                } else {
                    line.chars()
                        .into_iter()
                        .enumerate()
                        .for_each(|(x, c)| match c {
                            '#' => {
                                walls.insert((x as i32, height));
                            }
                            'O' => {
                                boxes.insert((x as i32, height));
                            }
                            '@' => {
                                robot_position = (x as i32, height);
                            }
                            _ => {}
                        });
                    height += 1;
                }
            }
            None => break,
        }
    }

    let mut instructions: Vec<Instruction> = Vec::new();

    // Parse Instructions
    loop {
        match lines.next() {
            Some(line) => {
                if line.len() == 0 {
                    break;
                } else {
                    line.chars()
                        .into_iter()
                        .enumerate()
                        .for_each(|(x, c)| match c {
                            '^' => {
                                instructions.push(Instruction::North);
                            }
                            '>' => {
                                instructions.push(Instruction::East);
                            }
                            'v' => {
                                instructions.push(Instruction::South);
                            }
                            '<' => {
                                instructions.push(Instruction::West);
                            }
                            _ => {}
                        })
                }
            }
            None => {
                break;
            }
        }
    }

    let ware_house = WareHouse {
        width,
        height,
        robot_position,
        walls,
        boxes,
        box_width: 1,
    };

    (ware_house, instructions)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_input() {
        let input = std::fs::read_to_string("./resources/day15/example.txt").unwrap();
        let (w, i) = parse_input(&input);
        let solution = solve_part_one(w, &i);
        assert_eq!(solution, "10092");
    }

    #[test]
    fn test_part_two() {
        let input = std::fs::read_to_string("./resources/day15/example.txt").unwrap();
        let (w, i) = parse_input(&input);
        let solution = solve_part_two(w, &i);
        //assert_eq!(solution, "10092");
    }
}
