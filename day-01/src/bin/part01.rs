fn main() {
    let input = include_str!("../input.txt");
    println!("{}", compute_calibration(input));
}

fn compute_calibration(input: &str) -> u32 {
    input.lines().into_iter().map(|x| 10*first_number(x)+last_number(x)).sum()
}

fn first_number(line: &str) -> u32 {
    let first_digit_char = line.chars().into_iter().filter(|x| x.is_digit(10)).next();
    match first_digit_char {
        Some(x) => x.to_digit(10).unwrap(),
        None => 0,
    }
}

fn last_number(line: &str) -> u32 {
    let first_digit_char = line.chars().into_iter().rev().filter(|x| x.is_digit(10)).next();
    match first_digit_char {
        Some(x) => x.to_digit(10).unwrap(),
        None => 0,
    }
}

