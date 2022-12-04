use std::{convert::Infallible, str::FromStr};

use serde_scan::scan;

const INPUT: &str = include_str!("../../inputs/day04.txt");

#[derive(Clone, Debug)]
struct Input {
    pairs: Vec<Pair>,
}

#[derive(Clone, Debug)]
struct Pair {
    left: Range,
    right: Range,
}

impl Pair {
    fn one_contains_other(&self) -> bool {
        self.left.contains(&self.right) || self.right.contains(&self.left)
    }

    fn one_overlaps_with(&self) -> bool {
        self.left.overlaps(&self.right)
    }
}

impl FromStr for Pair {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end, start2, end2): (usize, usize, usize, usize) =
            scan!("{}-{},{}-{}" <- s).unwrap();

        let left = Range(start, end);
        let right = Range(start2, end2);

        Ok(Self { left, right })
    }
}

#[derive(Clone, Debug)]
struct Range(usize, usize);

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }

    fn overlaps(&self, other: &Range) -> bool {
        self.0 <= other.1 && self.1 >= other.0
    }
}

impl Input {
    fn new(input: &str) -> Input {
        let pairs = input.lines().map(|line| line.parse().unwrap()).collect();
        Input { pairs }
    }
}

fn part1(input: &Input) -> usize {
    input
        .pairs
        .iter()
        .filter(|pair| pair.one_contains_other())
        .count()
}

fn part2(input: &Input) -> usize {
    input
        .pairs
        .iter()
        .filter(|pair| pair.one_overlaps_with())
        .count()
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
    const TEST_INPUT: &str = include_str!("../../inputs/test_day04.txt");

    #[test]
    fn examples() {
        let input = Input::new(TEST_INPUT);
        assert_eq!(part1(&input), 2);
        assert_eq!(part2(&input), 4);
    }

    #[test]
    fn answers() {
        let input = Input::new(INPUT);
        assert_eq!(part1(&input), 569);
        assert_eq!(part2(&input), 936);
    }
}
