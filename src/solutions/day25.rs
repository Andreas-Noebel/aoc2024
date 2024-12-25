use std::iter::zip;

fn solve(input_file_path: &str){

    let input = std::fs::read_to_string(input_file_path).unwrap();
    let (lock,key) = parse_puzzle(&input);
    let part_one = solve_part_one(&lock, &key);

}

type Lock = Vec<u8>;
type Key = Vec<u8>;

fn parse_puzzle(input: &str) -> (Vec<Lock>, Vec<Key>){

    let key_width = input.lines().nth(0).unwrap().len();
    println!("Width of key and lock {}", key_width);

    let mut keys: Vec<Vec<u8>> = Vec::new();
    let mut locks: Vec<Vec<u8>> = Vec::new();

    let mut buff = String::new();
    let mut iter = input.split("\r\n");
    while let Some(line) = iter.next(){

        if line.trim().is_empty(){

            // Parse locks
            if buff.starts_with("#"){
                let mut lock = vec![0u8; key_width];
                buff.chars().skip(key_width).enumerate().for_each(|(i,x)| {
                    match x {
                        '#' => lock[i%key_width] += 1,
                        _ => {}
                    }
                });
                locks.push(lock)
            }

            // Parse keys
            if buff.starts_with('.'){
                let mut key = vec![0u8; key_width];
                buff.chars().skip(key_width).enumerate().for_each(|(i,x)| {
                    match x {
                        '#' => key[i%key_width] += 1,
                        _ => {}
                    }
                });
                key = key.iter().map(|v| v-1).collect();
                keys.push(key);
            }

            buff.clear();
        }
        buff.push_str(line);
    }
    println!("{}", buff);
    (locks, keys)


}

fn solve_part_one(locks: &Vec<Lock>, keys: &Vec<Key>) -> u32{

    let mut unique_combos = 0;
    for key in keys {
        for lock in locks {
            let fits = zip(key, lock).map(|(a,b)| *a+*b).all(|l| l <= 5);
            if fits {
                //println!("Found key {:?} in lock {:?}", key, lock);
                unique_combos += 1;
            }
        }
    }
    unique_combos

}

#[cfg(test)]
mod tests {
    use crate::solutions::day25::{parse_puzzle, solve_part_one};

    #[test]
    fn test_parse_puzzle(){
        let input = std::fs::read_to_string("./resources/day25/input.txt").unwrap();
        let (lock,key) = parse_puzzle(&input);
        let part_one = solve_part_one(&lock, &key);
        println!("Part One: {}", part_one);
    }
}