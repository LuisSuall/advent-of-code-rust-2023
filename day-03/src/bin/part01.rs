use std::cmp;

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
    fn is_part_number(&self, input: &Vec<&str>) -> bool {
        let max_line_len = input[self.line].len();
        let first_char = if self.first_char == 0 { 0 } else { self.first_char-1 };
        let last_char = cmp::min(self.last_char+1, max_line_len);

        if self.line != 0{
            if has_special_char(&input.get(self.line-1).expect("")[(first_char)..(last_char)]){
                return true
            }
        }
        if self.line != input.len() {
            if has_special_char(&input.get(self.line+1).expect("")[(first_char)..(last_char)]){
                return true
            }
        }
        if self.first_char != 0 {
            if has_special_char(&input[self.line][first_char..first_char+1]) {
                return true
            }
        }
        if self.last_char != max_line_len {
            if has_special_char(&input[self.line][last_char-1..last_char]) {
                return true
            }
        }

        false
    }
}

fn has_special_char(values: &str) -> bool {
    values.chars().into_iter().any(|x| x != '.')
}

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", sum_part_numbers(input));
}

fn sum_part_numbers(input: &str) -> u32 {
    let input_by_line: Vec<&str> = input.lines().collect();
    input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| extract_numbers(line, i))
        .filter(|x| x.is_part_number(&input_by_line))
        .map(|x| x.value)
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

