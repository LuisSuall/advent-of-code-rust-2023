use std::collections::HashSet;
use std::iter::zip;

use regex::Regex;

fn main() {
    let input = include_str!("../test01.txt");
    println!("{}", sum_prices(input));
}

fn sum_prices(input: &str) -> u32 {
    let winning_numbers = extract_winning_numbers(input);
    let own_numbers = extract_own_numbers(input);
    let mut points_cache: Vec<u32> = vec![0; own_numbers.len()];
    let winners_per_card: Vec<u32> = zip(winning_numbers, own_numbers)
        .map(|(winners,own)| own.iter().filter(|number| winners.contains(number)).count() as u32)
        .collect();
    (0..winners_per_card.len()).map(|card| calculate_points(card, &winners_per_card, &mut points_cache)).sum()
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

fn calculate_points(card: usize, winners_per_card: &Vec<u32>, cache: &mut Vec<u32>) -> u32 {
    if card >= winners_per_card.len() {
        return 0
    }

    if cache[card] != 0 {
        return cache[card]
    }

    1 + (0..winners_per_card[card]).map(|i| calculate_points(card+1+(i as usize), winners_per_card, cache)).sum::<u32>()
}

