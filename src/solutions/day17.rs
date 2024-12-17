pub fn solve(input_path: &str) -> (String, String) {
    let input = std::fs::read_to_string(input_path).unwrap();
    let mut program = parse(&input);
    let part_one = part_one(&mut program);
    let part_two = part_two(&mut program);
    (part_one, part_two)
}
#[derive(Clone)]
struct Program {
    register_a: u64,
    register_b: u64,
    register_c: u64,

    instruction_pointer: u64,
    instructions: Vec<u8>,
    output: Vec<u8>,
}

impl Program {
    fn run(&mut self) {
        while self.instruction_pointer < self.instructions.len() as u64 {
            self.step()
        }
    }
    #[rustfmt::skip]
    fn step(&mut self) {
        let op_code = self.instructions[self.instruction_pointer as usize];
        let literal = self.instructions[(self.instruction_pointer + 1) as usize] as u64;
        let combo_value = self.get_operand(literal as u8);

        match op_code {
            // ADV
            0 => { self.register_a = self.register_a >> combo_value; }
            // BXL
            1 => { self.register_b = self.register_b ^ literal; }
            // BST
            2 => self.register_b = combo_value & 0x7,
            // JNZ
            3 => {
                if self.register_a != 0 {
                    self.instruction_pointer = literal;
                    return;
                }
            }
            //BCX
            4 => self.register_b = self.register_b ^ self.register_c,
            // Out
            5 => { self.output.push((combo_value & 0x7) as u8); }
            // BDV
            6 => { self.register_b = self.register_a >> combo_value; }
            // CDV
            7 => { self.register_c = self.register_a >> combo_value; }
            _ => { println!("Unknown op code: {}", op_code); }
        }
        self.instruction_pointer += 2;
    }

    fn get_operand(&self, operand: u8) -> u64 {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            7 => 7,
            x => panic!("Invalid operand {}", x),
        }
    }

    #[allow(dead_code)]
    fn print_state(&self) {
        println!("Register A: {:?}", self.register_a);
        println!("Register B: {:?}", self.register_b);
        println!("Register C: {:?}", self.register_c);
        println!("Instruction Pointer: {:?}", self.instruction_pointer);
        println!("Instructions: {:?}", self.instructions);
        println!("Output: {:?}", self.output);
    }
}

fn part_one(program: &Program) -> String {
    let mut program = program.clone();
    program.run();
    format!("{:?}", program.output)
        .replace("[", "")
        .replace("]", "")
        .replace(" ", "")
}
fn part_two(program: &Program) -> String {
    let solution = match_output_recursively(program.clone(), 0, 1).unwrap();
    solution.to_string()
}

#[rustfmt::skip]
fn parse(input: &str) -> Program{

    let mut lines = input.lines();
    let reg_a = lines.next().unwrap().split_once(": ").unwrap().1.parse::<u64>().unwrap();
    let reg_b = lines.next().unwrap().split_once(": ").unwrap().1.parse::<u64>().unwrap();
    let reg_c = lines.next().unwrap().split_once(": ").unwrap().1.parse::<u64>().unwrap();
    lines.next();
    let program = lines.next().unwrap().split_once(": ").unwrap().1;
    let program = program.replace(",","").as_bytes().iter().map(|x|*x-48).collect();

    Program {
        register_a: reg_a,
        register_b: reg_b,
        register_c: reg_c,
        instruction_pointer: 0,
        instructions: program,
        output: Vec::new(),
    }

}

fn match_output_recursively(
    program: Program,
    initial_reg_a: u64,
    postfix_length: usize,
) -> Option<u64> {
    let postfix_to_match = &program.instructions[program.instructions.len() - postfix_length..];

    for octal_number in 0..8 {
        let mut p = program.clone();
        p.register_a = initial_reg_a + octal_number;
        p.run();

        if p.output.ends_with(&postfix_to_match) {
            if p.output.len() == program.instructions.len() {
                return Some(initial_reg_a + octal_number);
            } else {
                let recursive_solution = match_output_recursively(
                    program.clone(),
                    (initial_reg_a + octal_number) << 3,
                    postfix_length + 1,
                );
                if recursive_solution.is_some() {
                    return recursive_solution;
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = std::fs::read_to_string("./resources/day17/example.txt").unwrap();
        let mut program = parse(&input);
        program.run();
        assert_eq!(program.output, vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0])
    }

}
