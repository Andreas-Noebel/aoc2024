pub fn solve() -> (String, String) {
    let (width, height, matches_char_at_pos) = parse_puzzle("./resources/day04/input");

    let mut sum_part_one = 0;
    let mut sum_part_two = 0;

    for y in 0..height {
        for x in 0..width {
            sum_part_one += check_position_for_word("XMAS", &matches_char_at_pos, (x, y));
            sum_part_two += check_position_for_mas_cross(&matches_char_at_pos, (x, y))
        }
    }

    (sum_part_one.to_string(), sum_part_two.to_string())
}

fn check_position_for_word(
    word: &str,
    check_pos_for_char: impl Fn(i32, i32, char) -> bool,
    pos: (i32, i32),
) -> i32 {
    let search_directions: Vec<fn((i32, i32)) -> (i32, i32)> = Vec::from([
        |(x, y)| (x + 1, y),     // Right
        |(x, y)| (x, y - 1),     // Down
        |(x, y)| (x + 1, y + 1), // Top-Right
        |(x, y)| (x + 1, y - 1), // Down-Right
        |(x, y)| (x - 1, y),     // Left
        |(x, y)| (x, y + 1),     // Top
        |(x, y)| (x - 1, y - 1), // Bot-Left
        |(x, y)| (x - 1, y + 1), // Top-Left
    ]);

    search_directions
        .iter()
        .filter(|direction| check_in_direction(word, pos, direction, &check_pos_for_char))
        .count() as i32
}

fn check_position_for_mas_cross(
    check_pos: impl Fn(i32, i32, char) -> bool,
    (x, y): (i32, i32),
) -> i32 {
    if !check_pos(x, y, 'A') {
        0
    } else {
        let diag_one = check_pos(x - 1, y + 1, 'M') && check_pos(x + 1, y - 1, 'S')
            || check_pos(x - 1, y + 1, 'S') && check_pos(x + 1, y - 1, 'M');
        let diag_two = check_pos(x - 1, y - 1, 'M') && check_pos(x + 1, y + 1, 'S')
            || check_pos(x - 1, y - 1, 'S') && check_pos(x + 1, y + 1, 'M');
        if diag_one && diag_two {
            1
        } else {
            0
        }
    }
}

fn check_in_direction(
    word: &str,
    start_position: (i32, i32),
    position_update: impl Fn((i32, i32)) -> (i32, i32),
    check_pos: impl Fn(i32, i32, char) -> bool,
) -> bool {
    if word.is_empty() {
        return true;
    }
    let mut chars = word.chars();
    let head = chars.next().unwrap();
    let tail = chars.as_str();

    if check_pos(start_position.0, start_position.1, head) == true {
        check_in_direction(
            tail,
            position_update(start_position),
            position_update,
            check_pos,
        )
    } else {
        false
    }
}

fn parse_puzzle(file_path: &str) -> (i32, i32, Box<dyn Fn(i32, i32, char) -> bool>) {
    let mut input = std::fs::read_to_string(file_path).unwrap();
    let width = (input.find('\n').unwrap() - 1) as i32;
    let height = input.len() as i32 / (width + 1);

    input.retain(|c| !c.is_whitespace());
    let flat_input: Vec<char> = input.chars().collect();

    let matches_char_at_pos = Box::new(move |x: i32, y: i32, c: char| {
        if x < 0 || x >= width || y < 0 || y >= height {
            false
        } else {
            flat_input[(y * width + x) as usize] == c
        }
    });

    (width, height, matches_char_at_pos)
}
