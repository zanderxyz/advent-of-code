use std::collections::HashMap;

use crate::helpers::increment::Increment;
use itertools::Itertools;
use serde_scan::scan;

const INPUT: &str = include_str!("../../inputs/day14.txt");

type CountOfEachPair = HashMap<(char, char), usize>;

#[derive(Clone)]
struct Input {
    pub count_of_each_pair: CountOfEachPair,
    pub instructions: HashMap<(char, char), char>,
    pub first_char: char,
    pub last_char: char,
}

impl Input {
    pub fn new(input: &str) -> Input {
        let mut input_iter = input.lines();

        // The starting polymers are on the first line
        let polymers = input_iter.next().unwrap();
        let polymer_chars: Vec<char> = polymers.chars().collect();
        let first_char = *polymer_chars.first().unwrap();
        let last_char = *polymer_chars.last().unwrap();
        let pairs_count = polymers
            .chars()
            .tuple_windows()
            .fold(HashMap::new(), |mut acc, pair| {
                acc.increment(pair, 1);
                acc
            });

        // Skip blank line
        input_iter.next();

        let instructions = input_iter
            .map(|line| {
                let (text, c): (&str, char) = scan!("{} -> {}" <- line).unwrap();
                let char_vec: Vec<char> = text.chars().collect();
                let a = char_vec[0];
                let b = char_vec[1];
                ((a, b), c)
            })
            .collect();

        Input {
            count_of_each_pair: pairs_count,
            instructions,
            first_char,
            last_char,
        }
    }
}

fn apply_instructions(
    pairs_count: CountOfEachPair,
    instructions: &HashMap<(char, char), char>,
) -> CountOfEachPair {
    let mut new = CountOfEachPair::new();
    for (&(a, b), &count) in pairs_count.iter() {
        match instructions.get(&(a, b)) {
            Some(&char_to_insert) => {
                new.increment((a, char_to_insert), count);
                new.increment((char_to_insert, b), count);
            }
            None => {
                new.increment((a, b), count);
            }
        }
    }
    new
}

fn run_insertions(input: &Input, number_of_runs: usize) -> CountOfEachPair {
    // Run the instructions n times
    let mut count_of_each_pair = input.count_of_each_pair.clone();
    for _ in 0..number_of_runs {
        count_of_each_pair = apply_instructions(count_of_each_pair, &input.instructions);
    }
    count_of_each_pair
}

fn run_insertions_and_count(input: &Input, number_of_runs: usize) -> usize {
    let count_of_each_pair = run_insertions(input, number_of_runs);

    let mut count_of_each_char = HashMap::new();
    // This double counts all characters except the first and last
    for ((a, b), &c) in count_of_each_pair.iter() {
        count_of_each_char.increment(a, c);
        count_of_each_char.increment(b, c);
    }
    // Add one to the first and last character, so every character has been counted twice
    count_of_each_char.increment(&input.first_char, 1);
    count_of_each_char.increment(&input.last_char, 1);

    let most_frequent = count_of_each_char
        .iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .unwrap();
    let least_frequent = count_of_each_char
        .iter()
        .min_by(|a, b| a.1.cmp(b.1))
        .unwrap();

    // Halve the counts at the end
    most_frequent.1 / 2 - least_frequent.1 / 2
}

fn part1(input: &Input) -> usize {
    run_insertions_and_count(input, 10)
}

fn part2(input: &Input) -> usize {
    run_insertions_and_count(input, 40)
}

pub fn main() {
    let input = Input::new(INPUT);
    let answer1 = part1(&input);
    println!("Part 1: {}", answer1);
    let answer2 = part2(&input);
    println!("Part 2: {}", answer2);
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/test_day14.txt");

    #[test]
    pub fn examples() {
        let input = Input::new(TEST_INPUT);
        assert_eq!(part1(&input), 1588);
        assert_eq!(part2(&input), 2188189693529);
    }

    #[test]
    pub fn answers() {
        let input = Input::new(INPUT);
        assert_eq!(part1(&input), 2233);
        assert_eq!(part2(&input), 2884513602164);
    }
}
