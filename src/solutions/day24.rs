use crate::solutions::day24::Operation::OR;
use std::cmp::PartialEq;
use Operation::{AND, XOR};

pub fn solve(file_path: &str) -> (String, String) {
    let input = std::fs::read_to_string(file_path).unwrap();
    let mut circuit = parse_circuit(&input);

    let part_one = circuit.evaluate(circuit.default_x, circuit.default_y).unwrap();
    let part_two = solve_part_two(&mut circuit);

    (part_one.to_string(), part_two.to_string())
}
fn solve_part_two(circuit: &mut Circuit) -> String {
    let mut swaps: Vec<String> = vec![];
    loop {
        match circuit.parse_full_adder(0, None) {
            Ok(_) => {
                break;
            }
            Err((_, quick_fix)) => match quick_fix {
                None => break,
                Some((a, b)) => {
                    swaps.push(a.clone());
                    swaps.push(b.clone());
                    circuit.swap_outputs(a, b);
                }
            },
        }
    }
    swaps.sort();
    swaps.join(",")
}

fn parse_circuit(input: &str) -> Circuit {
    let mut lines = input.lines();

    let mut x = 0u64;
    let mut y = 0u64;
    let mut x_length = 0;
    let mut y_length = 0;
    let mut z_length = 0;

    let mut gates: Vec<Gate> = Vec::new();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let (input_id, value) = line.split_once(": ").unwrap();
        let (input_register, register_index) = input_id.split_at(1);

        let input_value = value.parse::<u64>().unwrap();
        let register_index = register_index.parse::<usize>().unwrap();
        match input_register {
            "x" => {
                x |= input_value << register_index;
                x_length += 1;
            }
            "y" => {
                y |= input_value << register_index;
                y_length += 1;
            }
            _ => {}
        }
    }

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let gate_line = line.split_whitespace().collect::<Vec<&str>>();
        let input_a = gate_line[0];
        let input_b = gate_line[2];
        let output = gate_line[4];
        let op = match gate_line[1] {
            "XOR" => XOR,
            "AND" => AND,
            "OR" => OR,
            _ => panic!("{}", format!("Unknown operation: {:?}", gate_line[1])),
        };

        let gate = Gate {
            operation: op,
            input_a: input_a.to_string(),
            input_b: input_b.to_string(),
            output: output.to_string(),
        };

        if output.contains("z") {
            z_length += 1;
        }
        gates.push(gate);
    }

    Circuit {
        gates,
        register_x_length: x_length,
        register_y_length: y_length,
        output_length: z_length,
        default_x: x,
        default_y: y,
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
enum Operation {
    AND,
    OR,
    XOR,
}
type CableId = String;
type SwapSuggestion = (CableId, CableId);
type ParseError = (String, Option<SwapSuggestion>);
#[derive(Debug, Clone)]
struct Gate {
    operation: Operation,
    input_a: CableId,
    input_b: CableId,
    output: CableId,
}
#[derive(Debug, Clone)]
struct Circuit {
    gates: Vec<Gate>,
    register_x_length: usize,
    register_y_length: usize,
    output_length: usize,
    default_x: u64,
    default_y: u64,
}

impl Circuit {
    fn evaluate(&self, x: u64, y: u64) -> Result<u64, String> {
        let mut output = 0u64;

        for i in 0..self.output_length {
            let output_register_index = format!("z{:0>2}", i);
            let value = self.evaluate_cable_recursively(output_register_index, x, y, 100);
            if value.is_err() {
                return Err(value.unwrap_err());
            }
            let value = value?;
            output |= (value as u64) << i;
        }

        Ok(output)
    }

    fn evaluate_cable_recursively(
        &self,
        cable_id: CableId,
        x: u64,
        y: u64,
        max_recursion: u64,
    ) -> Result<bool, String> {
        if max_recursion == 0 {
            return Err("Max depth reached".to_string());
        }

        if cable_id.contains("x") {
            let index = cable_id.split_at(1).1;
            let index = index.parse::<usize>().unwrap();
            return Ok(x >> index & 1 == 1);
        }

        if cable_id.contains("y") {
            let index = cable_id.split_at(1).1;
            let index = index.parse::<usize>().unwrap();
            return Ok(y >> index & 1 == 1);
        }

        match self.gates.iter().find(|gate| gate.output == cable_id) {
            None => return Ok(false),
            Some(gate) => match gate.operation {
                XOR => {
                    let a = self.evaluate_cable_recursively(
                        gate.input_a.clone(),
                        x,
                        y,
                        max_recursion - 1,
                    );
                    let b = self.evaluate_cable_recursively(
                        gate.input_b.clone(),
                        x,
                        y,
                        max_recursion - 1,
                    );
                    if a.is_ok() && b.is_ok() {
                        Ok(a.unwrap() ^ b.unwrap())
                    } else {
                        Err("".to_string())
                    }
                }
                AND => {
                    let a = self.evaluate_cable_recursively(
                        gate.input_a.clone(),
                        x,
                        y,
                        max_recursion - 1,
                    );
                    let b = self.evaluate_cable_recursively(
                        gate.input_b.clone(),
                        x,
                        y,
                        max_recursion - 1,
                    );
                    if a.is_ok() && b.is_ok() {
                        Ok(a? & b?)
                    } else {
                        Err("".to_string())
                    }
                }
                OR => {
                    let a = self.evaluate_cable_recursively(
                        gate.input_a.clone(),
                        x,
                        y,
                        max_recursion - 1,
                    );
                    let b = self.evaluate_cable_recursively(
                        gate.input_b.clone(),
                        x,
                        y,
                        max_recursion - 1,
                    );
                    if a.is_ok() && b.is_ok() {
                        Ok(a? | b?)
                    } else {
                        Err("".to_string())
                    }
                }
            },
        }
    }

    fn parse_full_adder(&self, index: u8, c_in: Option<CableId>) -> Result<(), ParseError> {
        if index >= (self.output_length - 1) as u8 {
            return Ok(());
        }

        let x_register = format!("x{:0>2}", index);
        let y_register = format!("y{:0>2}", index);

        if index == 0 {
            let ha = self.parse_half_adder(x_register, y_register);

            if ha.is_err() {
                return Err(ha.unwrap_err());
            }

            let (_, ha_c_out) = ha?;
            return self.parse_full_adder(index + 1, Some(ha_c_out.clone()));
        }

        let c_in = c_in.unwrap();

        let ha_1 = self.parse_half_adder(x_register, y_register);
        if ha_1.is_err() {
            return Err(ha_1.unwrap_err());
        }
        let (result_ha_1, carry_ha_1) = ha_1?;

        let ha_2 = self.parse_half_adder(result_ha_1.clone(), c_in.clone());
        if ha_2.is_err() {
            return Err(ha_2.unwrap_err());
        }
        let (result_ha_2, carry_ha_2) = ha_2?;

        // Check output
        let output_register_index = format!("z{:0>2}", index);
        if result_ha_2 != output_register_index {
            return Err((
                format!(
                    "Expected {} as output but found {}",
                    output_register_index, result_ha_2
                ),
                Some((output_register_index, result_ha_2)),
            ));
        }

        // Check carry
        let c_out = self.get_gate_by_inputs_and_op(&carry_ha_1, &carry_ha_2, OR);
        if c_out.is_none() {
            let one_input_match = self
                .gates
                .iter()
                .find(|gate| gate.input_a == carry_ha_1 || gate.input_b == carry_ha_2)
                .map(|gate| {
                    if carry_ha_1 == gate.input_a {
                        return (gate.input_b.clone(), carry_ha_2.clone());
                    }
                    if carry_ha_1 == gate.input_b {
                        return (gate.input_a.clone(), carry_ha_2.clone());
                    }
                    if carry_ha_2 == gate.input_a {
                        return (gate.input_b.clone(), carry_ha_1.clone());
                    }
                    if carry_ha_2 == gate.input_b {
                        return (gate.input_a.clone(), carry_ha_1.clone());
                    }
                    panic!("Unknown gate found");

                });

            return match one_input_match {
                Some((alt, m)) => Err((
                    format!("Expected or gate for {}, {}", carry_ha_1, carry_ha_2),
                    Some((alt, m.clone())),
                )),
                None => Err((
                    format!("Expected or gate for {}, {}", carry_ha_1, carry_ha_2),
                    None,
                )),
            };
        }
        let c_out = c_out.unwrap();

        self.parse_full_adder(index + 1, Some(c_out.output.clone()))
    }

    fn parse_half_adder(&self, x: CableId, y: CableId) -> Result<(CableId, CableId), ParseError> {
        // Check AND Gate
        let and = self.get_gate_by_inputs_and_op(&x, &y, AND);
        let xor = self.get_gate_by_inputs_and_op(&x, &y, XOR);

        // 'XOR' and 'AND' have the same inputs
        if and.is_none() || xor.is_none() {
            let one_input_match = self
                .gates
                .iter()
                .find(|gate| gate.input_a == x || gate.input_b == y)
                .map(|gate| {
                    if x == gate.input_a {
                        return (gate.input_b.clone(), y.clone());
                    }
                    if x == gate.input_b {
                        return (gate.input_a.clone(), y.clone());
                    }
                    if y == gate.input_a {
                        return (gate.input_b.clone(), x.clone());
                    }
                    if y == gate.input_b {
                        return (gate.input_a.clone(), x.clone());
                    }
                    panic!("Unknown gate found")
                });

            return match one_input_match {
                Some((alt, m)) => Err((
                    format!("Expected XOR/AND gate for {} {} [Fix available]", x, y).to_string(),
                    Some((alt.clone(), m.clone())),
                )),
                None => Err((
                    format!("Expected XOR/AND gate for {} {}", x, y).to_string(),
                    None,
                )),
            };
        }
        let and = and.unwrap();
        let xor = xor.unwrap();

        let output = xor.output.clone();
        let c_out = and.output.clone();
        Ok((output, c_out))
    }

    fn get_gate_by_inputs_and_op(
        &self,
        input_a: &CableId,
        input_b: &CableId,
        operation: Operation,
    ) -> Option<&Gate> {
        if let Some(g) = self.gates.iter().find(|gate| {
            gate.input_a == *input_a && gate.input_b == *input_b && gate.operation == operation
        }) {
            Some(g)
        } else if let Some(g) = self.gates.iter().find(|gate| {
            gate.input_a == *input_b && gate.input_b == *input_a && gate.operation == operation
        }) {
            Some(g)
        } else {
            None
        }
    }

    fn get_gate_by_input(&self, input_a: &CableId, input_b: &CableId) -> Option<&Gate> {
        self.gates.iter().find(|gate| {
            (gate.input_a == *input_a && gate.input_b == *input_b)
                || (gate.input_b == *input_a && gate.input_a == *input_b)
        })
    }
    fn get_gate_by_output(&self, output_a: &CableId) -> Option<&Gate> {
        self.gates.iter().find(|gate| gate.output == *output_a)
    }

    fn get_gates_matching_any_input(&self, input_a: &CableId, input_b: &CableId) -> Vec<&Gate> {
        self.gates
            .iter()
            .filter(|gate| gate.input_a == *input_a || gate.input_b == *input_b)
            .collect()
    }

    fn print_mermaid(&self) {
        println!("graph TD");
        for (index, gate) in self.gates.iter().enumerate() {
            let a = &gate.input_a;
            let b = &gate.input_b;

            println!("  {} --> {index}[{:?}]", a, gate.operation);
            println!("  {} --> {index}[{:?}]", b, gate.operation);
            println!("  {index}[{:?}] --> {}", gate.operation, gate.output)
        }
    }

    fn evaluate_circuit(&self) -> u8 {
        let mut x: u64 = 1 << self.output_length - 2;
        let mut faulty_bits = 0;

        while x > 0 {
            let expected = x;
            let actual = self.evaluate(x, 0);
            if actual.is_err() {
                return u8::MAX;
            }
            let actual = actual.unwrap();
            let bit = expected ^ actual;
            if actual != expected {
                faulty_bits += 1;
            }
            x = x >> 1;
        }
        faulty_bits
    }

    fn swap_outputs(&mut self, output_a: CableId, output_b: CableId) {
        self.gates = self
            .gates
            .iter()
            .map(|g| {
                return if g.output == output_a {
                    let mut g = g.clone();
                    g.output = output_b.clone();
                    g
                } else if g.output == output_b {
                    let mut g = g.clone();
                    g.output = output_a.clone();
                    g
                } else {
                    g.clone()
                };
            })
            .collect::<Vec<Gate>>();
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::day24::{parse_circuit, CableId, ParseError, SwapSuggestion};

    #[test]
    fn test_solve() {
        let input = std::fs::read_to_string("./resources/day24/input.txt").unwrap();
        let mut circuit = parse_circuit(&input);

        println!(
            "{:?}",
            circuit.evaluate(circuit.default_x, circuit.default_y)
        );

        println!("{:?}", circuit.parse_full_adder(0, None));
        println!("{:?}", circuit.evaluate_circuit(),);

        return;
    }
}
