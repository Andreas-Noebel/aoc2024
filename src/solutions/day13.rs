pub fn solve(input_file_path: &str) -> (String, String) {
    let input = std::fs::read_to_string(input_file_path).unwrap();
    let claws = parse_puzzle(&input);
    let solution_part_one = solve_part_one(&claws);
    let solution_part_two = solve_part_two(&claws);

    (solution_part_one, solution_part_two)
}

#[derive(Debug)]
struct Claw {
    price_position: (i64, i64),
    button_a: (i64, i64),
    button_b: (i64, i64),
}

fn solve_part_one(claws: &Vec<Claw>) -> String {
    let mut tokens = 0;
    for claw in claws {
        match solve_claw(&claw) {
            Ok((button_1, button_2)) => {
                if button_1 <= 100 && button_2 <= 100 {
                    tokens += 3 * button_1 + button_2;
                }
            }
            Err(_) => {}
        }
    }
    tokens.to_string()
}

fn solve_part_two(claws: &Vec<Claw>) -> String {
    let mut tokens = 0;
    for claw in claws {
        let offset: i64 = 10000000000000;
        let pos = (
            claw.price_position.0 + offset,
            claw.price_position.1 + offset,
        );
        let claw_update: Claw = Claw {
            price_position: pos,
            button_a: claw.button_a,
            button_b: claw.button_b,
        };
        match solve_claw(&claw_update) {
            Ok((button_1, button_2)) => {
                tokens += 3 * button_1 + button_2;
            }
            Err(_) => {}
        }
    }
    tokens.to_string()
}

fn solve_claw(claw: &Claw) -> Result<(i64, i64), String> {
    /*  Idea - Calculate button_click# by inverting a 2x2 matrix (**sad numeric noises**)

       Price := (p_x, p_y)^T
       Claw_1 := (a_x, a_y)^T
       Claw_2 := (b_x, b_y)^T

       Clicks := (c_1, c_2)^T

       (Claw_1 Claw_2) * Clicks = Price

       Clicks = (Claw_1 Claw_2)^-1 * Clicks

    */

    let (a_x, a_y) = claw.button_a;
    let (b_x, b_y) = claw.button_b;
    let (p_x, p_y) = claw.price_position;

    let det = a_x * b_y - b_x * a_y;

    if det == 0 {
        return Err("Not solvable".to_string());
    }

    // "Inverse" of A (not scaled by det)
    let (a, b, c, d) = (b_y, -b_x, -a_y, a_x);
    // Calculate unscaled solution
    let (c_1, c_2) = (a * p_x + b * p_y, c * p_x + d * p_y);
    // Check if scaled solution is an integer
    if c_1 % det == 0 && c_2 % det == 0 {
        Ok((c_1 / det, c_2 / det))
    } else {
        Err("Not solvable with int".to_string())
    }
}

fn parse_puzzle(input: &str) -> Vec<Claw> {
    let mut claws: Vec<Claw> = Vec::new();
    let mut lines = input.lines();

    let parse_button = |line: &str| -> (i64, i64) {
        let split = line.split_whitespace().collect::<Vec<&str>>();
        let x = split[2].replace("X+", "").replace(",", "");
        let y = split[3].replace("Y+", "");
        let x = x.parse::<i32>().unwrap() as i64;
        let y = y.parse::<i32>().unwrap() as i64;
        (x, y)
    };

    let parse_prize = |line: &str| -> (i64, i64) {
        let split = line.split_whitespace().collect::<Vec<&str>>();
        let x = split[1].replace("X=", "").replace(",", "");
        let y = split[2].replace("Y=", "");
        let x = x.parse::<i32>().unwrap() as i64;
        let y = y.parse::<i32>().unwrap() as i64;
        (x, y)
    };

    for _ in 0..input.lines().count() / 4 + 1 {
        let line_button_a = lines.next().unwrap();
        let line_button_b = lines.next().unwrap();
        let line_price = lines.next().unwrap();
        lines.next();

        let claw = Claw {
            price_position: parse_prize(line_price),
            button_a: parse_button(line_button_a),
            button_b: parse_button(line_button_b),
        };

        claws.push(claw)
    }
    claws
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = std::fs::read_to_string("./resources/day13/example.txt").unwrap();
        let claws = parse_puzzle(&input);
        let solution = solve_part_one(&claws);
        assert_eq!(solution, "480");
    }

    #[test]
    fn test_part_two() {
        let input = std::fs::read_to_string("./resources/day13/example.txt").unwrap();
        let claws = parse_puzzle(&input);
        let solution = solve_part_two(&claws);
        assert_eq!(solution, "875318608908");
    }
}
