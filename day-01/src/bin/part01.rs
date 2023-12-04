fn main() {
    let input = include_str!("../input.txt");
    println!("{}", compute_calibration(input));
}

fn compute_calibration(input: &str) -> u32 {
    input.lines().map(|x| 10*first_number(x)+last_number(x)).sum()
}

fn first_number(line: &str) -> u32 {
    let first_digit_char = line.chars().find(|x| x.is_ascii_digit());
    match first_digit_char {
        Some(x) => x.to_digit(10).unwrap(),
        None => 0,
    }
}

fn last_number(line: &str) -> u32 {
    let first_digit_char = line.chars().rev().find(|x| x.is_ascii_digit());
    match first_digit_char {
        Some(x) => x.to_digit(10).unwrap(),
        None => 0,
    }
}

