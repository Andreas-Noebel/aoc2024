use std::collections::HashMap;
use crate::solutions::day24::SignalSource::{Gate, Init};

pub fn solve(file_path: &str) -> (String, String) {

    let input = std::fs::read_to_string(file_path).unwrap();
    let (circuit,output_cable) = pares_input(&input);

    // part one
    let mut sorted_output = output_cable;
    sorted_output.sort();
    sorted_output.reverse();
    let solution_one = evaluate_output(&circuit, sorted_output);

    (solution_one.to_string(),"".to_string())
}


type Signal = bool;
type Id = String;
type Cable = Id;
#[derive(Debug, Eq, PartialEq, Hash)]
enum Operation {
    AND,
    OR,
    XOR
}

#[derive(Debug, Eq, PartialEq, Hash)]
enum SignalSource {
    Gate(Cable, Cable, Operation),
    Init(Signal)
}

type Circuit = HashMap<Cable, SignalSource>;

fn pares_input(input: &str) -> (Circuit, Vec<Cable>) {

    let mut circuit : Circuit = HashMap::new();
    let mut output_cables: Vec<Cable> = Vec::new();
    // Parse cables
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break
        }
        let (cable_id,cable_value) = line.split_once(" ").unwrap();
        let cable_id = cable_id.replace(":","");
        let cable_value = match cable_value { "1" => true, _ => false  };
        println!("{:?} -> {}", cable_id, cable_value);
        let cable: Cable = cable_id;
        if circuit.contains_key(&*cable) {
            panic!()
        }

        circuit.insert(cable, Init(cable_value));

    }

    // Parse Gates
    while let Some(line) = lines.next() {
        let (gate,gate_output) = line.split_once("->").unwrap();
        let gate_output = gate_output.trim().to_string() as Cable;

        let gate = gate.trim().split(' ').collect::<Vec<&str>>();
        let gate_input_1: Cable = gate[0].to_string() as Cable;
        let gate_input_2 = gate[2].to_string();
        let gate_operation = match gate[1] {
            "XOR" => {Operation::XOR}
            "AND" => {Operation::AND}
            "OR" => {Operation::OR}
            _ => { panic!()}
        };
        println!("{:?} {:?} {:?} -> {:?}", gate_input_1,gate_operation, gate_input_2, gate_output);
        let gate = Gate(gate_input_1, gate_input_2, gate_operation);

        if circuit.contains_key(&*gate_output) {
            panic!()
        }

        if gate_output.starts_with("z") {
            output_cables.push(gate_output.clone());
        }

        circuit.insert(gate_output, gate);
    }
    (circuit,output_cables)
}

fn evaluate_cable(cable: &Cable, circuit: &Circuit) -> Signal {

    if !circuit.contains_key(cable) {
        panic!()
    }

    match circuit.get(cable).unwrap() {
        Gate(i1, i2, op) => {
            return match op {
                Operation::XOR => {
                    evaluate_cable(i1, circuit) ^ evaluate_cable(i2, circuit)
                }
                Operation::AND => {
                    evaluate_cable(i1, circuit) && evaluate_cable(i2, circuit)
                }
                Operation::OR => {
                    evaluate_cable(i1, circuit) || evaluate_cable(i2, circuit)
                }
            }
        }
        Init(x) => *x,
    }

}

fn evaluate_output(circuit: &Circuit, output: Vec<Cable>) -> u64{

    let mut bitset: u64 = 0;
    for output_cable in output {
        bitset <<= 1;
        let output_cable_value = evaluate_cable(&output_cable, &circuit);
        bitset |= output_cable_value as u64;
    }
    bitset

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = std::fs::read_to_string("./resources/day24/input.txt").unwrap();
        let (circuit,output_cable) = pares_input(&input);
        let mut sorted_output = output_cable;
        sorted_output.sort();
        sorted_output.reverse();

        let solution = evaluate_output(&circuit, sorted_output);
        println!("{:?}",solution)


    }
}