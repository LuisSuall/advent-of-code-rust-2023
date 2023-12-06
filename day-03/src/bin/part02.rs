use regex::Regex;

#[derive(Debug)]
struct NumberPosition {
    value: u32,
    line: usize,
    first_char: usize,
    last_char: usize,
}

impl NumberPosition {
    fn new(value: u32, line: usize, first_char: usize, last_char: usize) -> NumberPosition {
        Self {
            value,
            line,
            first_char,
            last_char
        }

    }

    fn is_next_to_point(&self, line: usize,char_position: usize) -> bool {
        let low_char_bound = if self.first_char == 0 { 0 } else { self.first_char-1 };
        let high_char_bound = self.last_char;
        let low_line_bound = if self.line == 0 { 0 } else { self.line-1 };
        let high_line_bound = self.line + 1;

        if low_line_bound <= line && line <= high_line_bound {
            if low_char_bound <= char_position && char_position <= high_char_bound {
                // println!("Found {} -- {}", self.line, self.first_char);
                return true
            }
        }
        false
    }
}

#[derive(Debug)]
struct Star {
    line: usize,
    char_pos : usize
}

impl Star {
    fn new(line: usize, char_pos: usize) -> Star {
        Self {
            line,
            char_pos
        }
    }

    fn gear_ratio(&self, numbers: &Vec<NumberPosition>) -> u32 {
        // println!("Searching for star {} - {}", self.line, self.char_pos);
        let related_numbers: Vec<&NumberPosition>= numbers
            .into_iter()
            .filter(|x| x.is_next_to_point(self.line, self.char_pos))
            .collect();
        
        if related_numbers.len() == 2 {
            return related_numbers[0].value * related_numbers[1].value;
        }
        0
    }
}


fn main() {
    let input = include_str!("../input.txt");
    println!("{}", sum_gear_ratios(input));
}

fn sum_gear_ratios(input: &str) -> u32 {
    let numbers = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| extract_numbers(line, i))
        .collect();
    input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| extract_stars(line, i))
        .map(|x| x.gear_ratio(&numbers))
        .sum()
}

fn extract_numbers(line: &str, line_number: usize) -> Vec<NumberPosition> {
    let num_re = Regex::new(r"(\d)+").unwrap();
    num_re
        .find_iter(line)
        .map(|x| NumberPosition::new(
            x.as_str().parse::<u32>().unwrap_or(0), 
            line_number, 
            x.start(), 
            x.end()
        ))
        .collect()
}

fn extract_stars(line: &str, line_number: usize) -> Vec<Star> {
    let num_re = Regex::new(r"\*").unwrap();
    num_re
        .find_iter(line)
        .map(|x| Star::new(
            line_number, 
            x.start() 
        ))
        .collect()
}

