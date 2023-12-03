use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", compute_calibration(input));
}

fn compute_calibration(input: &str) -> u32 {
    input.lines().into_iter().map(|x| 10*first_number(x)+last_number(x)).sum()
}

fn first_number(line: &str) -> u32 {
    let re = Regex::new(r"[0-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let first_match_found = re.find_iter(line).map(|x| x.as_str()).next().unwrap_or("");
    match first_match_found {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        x => x.parse::<u32>().unwrap_or(0),
    }
}

fn last_number(line: &str) -> u32 {
    let re = Regex::new(r"[0-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();
    let binding = line.chars().rev().collect::<String>();
    let rev_line = binding.as_str();
    let first_match_found = re.find_iter(rev_line).map(|x| x.as_str()).next().unwrap_or("");
    match first_match_found {
        "eno" => 1,
        "owt" => 2,
        "eerht" => 3,
        "ruof" => 4,
        "evif" => 5,
        "xis" => 6,
        "neves" => 7,
        "thgie" => 8,
        "enin" => 9,
        x => x.parse::<u32>().unwrap_or(0),
    }
}

