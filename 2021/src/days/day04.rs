use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;
use regex::Regex;

const INPUT: &str = include_str!("../../inputs/day04.txt");

#[derive(Clone)]
struct Input {
    pub numbers: Vec<usize>,
    pub boards: Vec<Board>,
}

#[derive(Clone)]
struct Board {
    // This is a mapping from the numbers on the board, to the lines each of them is in the
    // Each number is part of two lines: horizontal and vertical
    numbers_data: HashMap<usize, [usize; 2]>,
    // There are 10 lines in the board: 5 horizontal and 5 vertical
    // We track the number of filled entries in each line
    found_count_per_line: [usize; 10],
    // Keep track of the numbers we have found
    found: HashSet<usize>,
    // True if the board is complete
    pub is_complete: bool,
}

impl Board {
    /// Initialise a board with a slice of 25 numbers
    pub fn new(input: &[usize]) -> Self {
        let mut numbers_data = HashMap::new();
        for (index, &n) in input.iter().enumerate() {
            // Find the lines this particular number is in, based on the index
            let first = index / 5; // integer divison
            let second = 5 + index % 5; // remainder
            numbers_data.insert(n, [first, second]);
        }
        Board {
            numbers_data,
            found: HashSet::new(),
            found_count_per_line: [0; 10],
            is_complete: false,
        }
    }

    /// Attempt to find a number on the board
    /// Returns true if this number completes a line
    pub fn update_and_check_if_complete(&mut self, number: usize) -> bool {
        // Ensure a number can only be found once per board
        if self.found.contains(&number) {
            return false;
        }

        let lines = self.numbers_data.get(&number);
        match lines {
            None => {
                // This number is not on the board
                false
            }
            Some(&[first, second]) => {
                // This number is on the board, and is part of these two lines

                // Add it to the found set
                self.found.insert(number);

                // If we have completed either line, return true
                if self.update_line_and_check_if_complete(first) {
                    self.is_complete = true;
                    return true;
                }
                if self.update_line_and_check_if_complete(second) {
                    self.is_complete = true;
                    return true;
                }

                false
            }
        }
    }

    fn update_line_and_check_if_complete(&mut self, line: usize) -> bool {
        // Increment the found count for this line
        self.found_count_per_line[line] += 1;

        // If it has 5 items found, then it's complete
        self.found_count_per_line[line] == 5
    }

    pub fn score(&self, number: usize) -> usize {
        let sum_of_unmarked: usize = self
            .numbers_data
            .keys()
            .filter(|n| !self.found.contains(n))
            .sum();

        sum_of_unmarked * number
    }
}

impl Input {
    pub fn new(input: &str) -> Input {
        lazy_static! {
            static ref RE: Regex = Regex::new("(\\d+)").unwrap();
        }
        let mut input_iter = input.lines();

        // The numbers are on the first line
        let numbers = input_iter
            .next()
            .unwrap()
            .split(',')
            .map(|line| line.parse::<usize>().unwrap())
            .collect();

        // Skip blank line
        input_iter.next();

        let mut boards = Vec::new();
        while let Some(mut row) = input_iter.next() {
            // Parse and insert a single 5x5 board
            let mut numbers = Vec::new();

            // Loop until we fill a board
            loop {
                for capture in RE.captures_iter(row) {
                    let num = capture[0].parse::<usize>().unwrap();
                    numbers.push(num);
                }

                // Board is full
                if numbers.len() == 25 {
                    break;
                }

                row = input_iter.next().unwrap();
            }

            // We have 25 numbers in the board, add it to the list
            boards.push(Board::new(&numbers));
        }

        Input { numbers, boards }
    }
}

fn part1(mut input: Input) -> usize {
    for number in input.numbers {
        for board in &mut input.boards {
            if board.update_and_check_if_complete(number) {
                // Return the score of the first board completed
                return board.score(number);
            }
        }
    }
    panic!("Failed to complete any boards");
}

fn part2(mut input: Input) -> usize {
    let number_of_boards = input.boards.len();
    let mut completed_boards: usize = 0;
    for number in input.numbers {
        for board in &mut input.boards {
            if !board.is_complete && board.update_and_check_if_complete(number) {
                completed_boards += 1;
                if completed_boards == number_of_boards {
                    // Return the score of the final board completed
                    return board.score(number);
                }
            }
        }
    }
    panic!("Failed to complete all boards");
}

pub fn main() {
    let input = Input::new(INPUT);
    let answer1 = part1(input.clone());
    println!("Part 1: {}", answer1);
    let answer2 = part2(input);
    println!("Part 2: {}", answer2);
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/test_day04.txt");

    #[test]
    pub fn examples() {
        let input = Input::new(TEST_INPUT);
        assert_eq!(part1(input.clone()), 4512);
        assert_eq!(part2(input), 1924);
    }

    #[test]
    pub fn answers() {
        let input = Input::new(INPUT);
        assert_eq!(part1(input.clone()), 10374);
        assert_eq!(part2(input), 24742);
    }
}
