use std::{convert::Infallible, str::FromStr};

use im::HashSet;
use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day03.txt");

#[derive(Clone, Debug)]
struct Input {
    backpacks: Vec<Backpack>,
}

#[derive(Clone, Debug)]
struct Backpack {
    left: Compartment,
    right: Compartment,
}

impl Backpack {
    fn all_items(self) -> HashSet<Item> {
        self.left.items.union(self.right.items)
    }

    fn common_items(self) -> HashSet<Item> {
        self.left.items.intersection(self.right.items)
    }

    fn single_common_item(self) -> Item {
        let common = self.common_items();
        assert_eq!(common.len(), 1);
        common.into_iter().next().unwrap()
    }
}

#[derive(Clone, Debug)]
struct Compartment {
    items: HashSet<Item>,
}

type Item = char;

fn score(item: Item) -> u32 {
    let code = item as u8;
    let value = match code {
        b'a'..=b'z' => code - b'a' + 1,
        b'A'..=b'Z' => code - b'A' + 27,
        _ => panic!("Unexpected char {item}"),
    };
    value.into()
}

impl FromStr for Compartment {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items = s.chars().into_iter().collect();
        Ok(Self { items })
    }
}

impl Input {
    fn new(input: &str) -> Input {
        let backpacks = input
            .lines()
            .map(|line| {
                let len = line.len() / 2;
                let left = line[..len].parse().unwrap();
                let right = line[len..].parse().unwrap();
                Backpack { left, right }
            })
            .collect();
        Input { backpacks }
    }
}

fn part1(input: &Input) -> u32 {
    input
        .backpacks
        .clone()
        .into_iter()
        .map(|backpack| backpack.single_common_item())
        .map(score)
        .sum()
}

fn part2(input: &Input) -> u32 {
    input
        .backpacks
        .clone()
        .into_iter()
        .tuples()
        .map(|(a, b, c)| {
            a.all_items()
                .intersection(b.all_items())
                .intersection(c.all_items())
                .into_iter()
                .next()
                .unwrap()
        })
        .map(score)
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
    const TEST_INPUT: &str = include_str!("../../inputs/test_day03.txt");

    #[test]
    fn examples() {
        let input = Input::new(TEST_INPUT);
        assert_eq!(part1(&input), 157);
        assert_eq!(part2(&input), 70);
    }

    #[test]
    fn answers() {
        let input = Input::new(INPUT);
        assert_eq!(part1(&input), 8252);
        assert_eq!(part2(&input), 2828);
    }
}
