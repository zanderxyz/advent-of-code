use std::cmp::Reverse;

use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day01.txt");

#[derive(Clone)]
struct Input {
    elves: Vec<Vec<usize>>,
}

impl Input {
    fn new(input: &str) -> Input {
        let mut elves = Vec::new();
        let mut current_elf: Vec<usize> = Vec::new();
        for line in input.lines() {
            if line.is_empty() {
                elves.push(current_elf);
                current_elf = Vec::new();
            } else {
                let number = line.parse::<usize>().unwrap();
                current_elf.push(number);
            }
        }
        elves.push(current_elf);

        Input { elves }
    }
}

fn part1(input: &Input) -> usize {
    input
        .elves
        .iter()
        .map(|elf| elf.iter().sum())
        .max()
        .unwrap()
}

fn part2(input: &Input) -> usize {
    input
        .elves
        .iter()
        .map(|elf| elf.iter().sum::<usize>())
        .sorted_by_key(|&x| Reverse(x))
        .take(3)
        .sum()
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
    const TEST_INPUT: &str = include_str!("../../inputs/test_day01.txt");

    #[test]
    fn examples() {
        let input = Input::new(TEST_INPUT);
        assert_eq!(part1(&input), 24000);
        assert_eq!(part2(&input), 45000);
    }

    #[test]
    fn answers() {
        let input = Input::new(INPUT);
        assert_eq!(part1(&input), 69528);
        assert_eq!(part2(&input), 206152);
    }
}
