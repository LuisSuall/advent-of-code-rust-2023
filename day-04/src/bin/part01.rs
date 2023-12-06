use std::collections::HashSet;
use std::iter::zip;

use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", sum_prices(input));
}

fn sum_prices(input: &str) -> u32 {
    let winning_numbers = extract_winning_numbers(input);
    let own_numbers = extract_own_numbers(input);
    let base: u32 = 2;
    zip(winning_numbers, own_numbers)
        .map(|(winners,own)| own.iter().filter(|number| winners.contains(number)).count() as u32)
        .map(|winner_found| if winner_found == 0 {0} else {base.pow(winner_found-1)})
        .sum()
}

fn extract_winning_numbers(input: &str) -> Vec<HashSet<u32>> {
    input
        .lines()
        .map(|line| line.split_once(':').unwrap().1)
        .map(|line| line.split_once('|').unwrap().0)
        .map(extract_numbers)
        .map(|numbers| HashSet::from_iter(numbers.iter().cloned()))
        .collect()
}

fn extract_own_numbers(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.split_once('|').unwrap().1)
        .map(extract_numbers)
        .collect()
}

fn extract_numbers(numbers_str: &str) -> Vec<u32> {
    let num_re = Regex::new(r"(\d)+").unwrap();
    num_re
        .find_iter(numbers_str)
        .map(|x| x.as_str().parse::<u32>().unwrap_or(0))
        .collect()
}
