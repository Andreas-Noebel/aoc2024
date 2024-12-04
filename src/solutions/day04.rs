pub fn solve() -> (String, String) {
    let (i, w, h) = parse_puzzle("./resources/day04/input");

    let mut sum_part_one = 0;
    let mut sum_part_two = 0;

    let check_char_at_pos = |x: i32, y: i32, c: char| {
        if x < 0 || x >= w || y < 0 || y >= h {
            false
        } else {
            i[(y * w + x) as usize] == c
        }
    };

    for y in 0..h {
        for x in 0..w {
            let xmas_at_pos = check_position_for_word("XMAS", check_char_at_pos, (x, y));
            let samx_at_pos = check_position_for_word("SAMX", check_char_at_pos, (x, y));
            sum_part_one += xmas_at_pos + samx_at_pos;

            if check_position_for_xmas_cross(check_char_at_pos, (x, y)) {
                sum_part_two += 1;
            }
        }
    }

    (sum_part_one.to_string(), sum_part_two.to_string())
}

fn check_position_for_word(
    word: &str,
    check_pos_for_char: impl Fn(i32, i32, char) -> bool,
    pos: (i32, i32),
) -> i32 {
    let mut count = 0;

    // Check left to right
    if check_in_direction(word, pos, |p| (p.0 + 1, p.1), &check_pos_for_char) {
        count += 1;
    }
    // Check top to down
    if check_in_direction(word, pos, |p| (p.0, p.1 - 1), &check_pos_for_char) {
        count += 1;
    }
    // Diag right ascending
    if check_in_direction(word, pos, |p| (p.0 + 1, p.1 + 1), &check_pos_for_char) {
        count += 1;
    }

    // Diag right descending
    if check_in_direction(word, pos, |p| (p.0 + 1, p.1 - 1), &check_pos_for_char) {
        count += 1;
    }

    return count;
}

fn check_position_for_xmas_cross(
    check_pos_for_char: impl Fn(i32, i32, char) -> bool,
    pos: (i32, i32),
) -> bool {
    if !check_pos_for_char(pos.0, pos.1, 'A') {
        false
    } else {
        let diag_one = check_pos_for_char(pos.0 - 1, pos.1 + 1, 'M')
            && check_pos_for_char(pos.0 + 1, pos.1 - 1, 'S')
            || check_pos_for_char(pos.0 - 1, pos.1 + 1, 'S')
                && check_pos_for_char(pos.0 + 1, pos.1 - 1, 'M');
        let diag_two = check_pos_for_char(pos.0 - 1, pos.1 - 1, 'M')
            && check_pos_for_char(pos.0 + 1, pos.1 + 1, 'S')
            || check_pos_for_char(pos.0 - 1, pos.1 - 1, 'S')
                && check_pos_for_char(pos.0 + 1, pos.1 + 1, 'M');
        diag_one && diag_two
    }
}

fn check_in_direction(
    word: &str,
    pos: (i32, i32),
    pos_increment: impl Fn((i32, i32)) -> (i32, i32),
    check: impl Fn(i32, i32, char) -> bool,
) -> bool {
    if word.is_empty() {
        return true;
    }
    let char = word.chars().nth(0).unwrap();

    if check(pos.0, pos.1, char) == true {
        let mut chars = word.chars();
        chars.next();
        check_in_direction(chars.as_str(), pos_increment(pos), pos_increment, check)
    } else {
        false
    }
}

fn parse_puzzle(file_path: &str) -> (Vec<char>, i32, i32) {
    let mut input = std::fs::read_to_string(file_path).unwrap();
    let width = input.find('\n').unwrap() - 1;
    let height = input.len() / (width + 1);
    input.retain(|c| !c.is_whitespace());
    (input.chars().collect(), width as i32, height as i32)
}
