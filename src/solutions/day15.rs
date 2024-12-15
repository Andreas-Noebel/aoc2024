use std::collections::{HashMap, HashSet, VecDeque};
pub fn solve(input_file_path: &str) -> (String, String) {
    let input = std::fs::read_to_string(input_file_path).unwrap();

    let (warehouse, i) = parse_input(&input);
    let solution_part_one = solve_part_one(warehouse.clone(), &i);
    let solution_part_two = solve_part_two(warehouse.clone(), &i);

    (solution_part_one, solution_part_two)
}

fn solve_part_one(mut ware_house: Warehouse, instructions: &Vec<Instruction>) -> String {
    for &instruction in instructions {
        ware_house.move_robot(instruction);
    }
    //ware_house.visualise_state();
    let solution = ware_house.sum_of_gps();
    solution.to_string()
}

fn solve_part_two(mut ware_house: Warehouse, instructions: &Vec<Instruction>) -> String {
    ware_house.scale();

    for &instruction in instructions {
        ware_house.move_robot(instruction);
    }

    let solution = ware_house.sum_of_gps();
    // ware_house.visualise_state();
    solution.to_string()
}

type Position = (i32, i32);
#[derive(Debug, Copy, Clone)]
enum Instruction {
    North,
    East,
    South,
    West,
}

impl Instruction {
    fn to_direction(&self) -> (i32, i32) {
        match *self {
            Instruction::North => (0, -1),
            Instruction::East => (1, 0),
            Instruction::South => (0, 1),
            Instruction::West => (-1, 0),
        }
    }
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
        let (dx, dy) = direction.to_direction();
        self.position = (x + dx, y + dy);
    }

    fn get_all_positions(&self, offset_x: i32, offset_y: i32) -> Vec<Position> {
        let mut positions = Vec::new();
        for sx in 0..self.width {
            for sy in 0..self.height {
                positions.push((
                    self.position.0 + sx + offset_x,
                    self.position.1 + sy + offset_y,
                ));
            }
        }
        positions
    }
}

#[derive(Clone)]
struct Warehouse {
    width: i32,
    height: i32,
    robot_position: Position,
    walls: HashSet<Position>,
    boxes: HashMap<i32, WarehouseBox>,
}

impl Warehouse {
    fn move_robot(&mut self, instruction: Instruction) {
        let direction = instruction.to_direction();
        let next_position = (
            self.robot_position.0 + direction.0,
            self.robot_position.1 + direction.1,
        );

        // Next pos is a wall do nothing
        if self.walls.contains(&next_position) {
            return;
        }

        // Next pos has a box try to move it
        if let Some(ware_house_box) = self.get_box_at_pos(&next_position) {
            let mut affected_boxes: HashSet<WarehouseBox> = HashSet::new();

            if self.is_box_pushable(&ware_house_box, &direction, &mut affected_boxes) {
                let mut updated_boxes = self.boxes.clone();

                for mut boxes_to_push in affected_boxes {
                    boxes_to_push.push(instruction);
                    updated_boxes.insert(boxes_to_push.id, boxes_to_push);
                }

                self.boxes = updated_boxes;
                self.robot_position = next_position;
            }
        } else {
            self.robot_position = next_position;
        }
    }
    #[allow(dead_code)]
    fn visualise_state(&self) {
        let mut grid = vec!['.'; (self.width * self.height) as usize];
        self.walls
            .iter()
            .for_each(|(x, y)| grid[(y * self.width + x) as usize] = '#');

        self.boxes.iter().for_each(|(_, b)| {
            let (x, y) = b.position;
            if b.width == 1 {
                grid[(y * self.width + x) as usize] = 'O';
            } else {
                grid[(y * self.width + x) as usize] = '[';
                grid[(y * self.width + x + 1) as usize] = ']';
            }
        });
        grid[(self.robot_position.1 * self.width + self.robot_position.0) as usize] = '@';

        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", grid[(y * self.width + x) as usize]);
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

    fn is_box_pushable(
        &self,
        start_box: &WarehouseBox,
        (dx, dy): &(i32, i32),
        affected_boxes: &mut HashSet<WarehouseBox>,
    ) -> bool {
        let mut neighbor_boxes: VecDeque<&WarehouseBox> = VecDeque::from([start_box]);

        while !neighbor_boxes.is_empty() {
            let box_to_check = neighbor_boxes.pop_front().unwrap();

            for position in box_to_check.get_all_positions(*dx, *dy) {
                if self.walls.contains(&position) {
                    return false;
                }

                if let Some(adjacent_box) = self.get_box_at_pos(&position) {
                    if !affected_boxes.contains(&adjacent_box) {
                        neighbor_boxes.push_back(adjacent_box)
                    }
                }
            }
            affected_boxes.insert(*box_to_check);
        }
        true
    }

    fn sum_of_gps(&self) -> i32 {
        self.boxes
            .values()
            .map(|b| b.position.1 * 100 + b.position.0)
            .sum()
    }

    fn scale(&mut self) {
        self.width *= 2;

        // Scale walls
        let mut scaled_walls: HashSet<Position> = HashSet::new();
        self.walls.iter().for_each(|pos| {
            let left = (pos.0 * 2, pos.1);
            let right = (left.0 + 1, left.1);
            scaled_walls.insert(left);
            scaled_walls.insert(right);
        });

        self.boxes = HashMap::from_iter(self.boxes.iter().map(|(id, b)| {
            let (bx, by) = b.position;
            let scaled_box = WarehouseBox {
                id: *id,
                position: (bx * 2, by),
                width: 2,
                height: 1,
            };
            (*id, scaled_box)
        }));

        self.robot_position = (self.robot_position.0 * 2, self.robot_position.1);
        self.walls = scaled_walls;
    }
}
#[rustfmt::skip]
fn parse_input(input: &str) -> (Warehouse, Vec<Instruction>) {
    let width = input.lines().next().unwrap().len() as i32;
    let mut height = 0;

    let mut lines = input.lines();

    // Parse map
    let mut walls: HashSet<Position> = HashSet::new();
    let mut boxes: HashSet<Position> = HashSet::new();
    let mut robot_position: Position = (0, 0);
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break
        }
        line.chars()
            .into_iter()
            .enumerate()
            .for_each(|(x, c)| match c {
                '#' => { walls.insert((x as i32, height)); }
                'O' => { boxes.insert((x as i32, height)); }
                '@' => { robot_position = (x as i32, height); }
                _ => {}
            });
        height += 1;
    }

    // Parse Instructions
    let mut instructions: Vec<Instruction> = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        line.chars().into_iter().for_each(|c| match c {
            '^' => { instructions.push(Instruction::North); }
            '>' => { instructions.push(Instruction::East); }
            'v' => { instructions.push(Instruction::South); }
            '<' => { instructions.push(Instruction::West); }
            _ => {}
        })

    }
    let boxes = HashMap::from_iter(boxes.iter().enumerate().map(|(id, pos)| {
        let warehouse_box = WarehouseBox {
            id: id as i32,
            position: (pos.0, pos.1),
            width: 1,
            height: 1,
        };
        (id as i32, warehouse_box)
    }));

    let ware_house = Warehouse {
        width,
        height,
        robot_position,
        walls,
        boxes,
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
        assert_eq!(solution, "9021");
    }
}
