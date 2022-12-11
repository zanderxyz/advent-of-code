use std::{cmp::Reverse, convert::Infallible, str::FromStr};

use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day11.txt");

#[derive(Clone, Debug)]
struct Input {
    monkeys: Vec<Monkey>,
}

impl Input {
    fn new(input: &str) -> Input {
        let lines = input.lines().collect_vec();
        let monkeys_str = lines.split(|line| line.is_empty());
        let monkeys = monkeys_str
            .map(|iter| {
                // Drop the first line, which is just the monkey number
                let mut iter = iter.iter().dropping(1);
                let items_str = iter.next().unwrap();
                let operation_str = iter.next().unwrap();
                let test_str = iter.next().unwrap();
                let true_str = iter.next().unwrap();
                let false_str = iter.next().unwrap();

                let items = items_str
                    .split(": ")
                    .dropping(1)
                    .next()
                    .unwrap()
                    .split(", ")
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect();

                let operation = operation_str
                    .split(" = ")
                    .dropping(1)
                    .next()
                    .unwrap()
                    .parse()
                    .unwrap();

                let test = test_str
                    .split("by ")
                    .dropping(1)
                    .next()
                    .unwrap()
                    .parse()
                    .unwrap();

                let if_true = true_str
                    .chars()
                    .rev()
                    .take(1)
                    .next()
                    .unwrap()
                    .to_digit(10)
                    .unwrap()
                    .try_into()
                    .unwrap();

                let if_false = false_str
                    .chars()
                    .rev()
                    .take(1)
                    .next()
                    .unwrap()
                    .to_digit(10)
                    .unwrap()
                    .try_into()
                    .unwrap();

                Monkey {
                    items,
                    operation,
                    divisor: test,
                    if_true,
                    if_false,
                }
            })
            .collect_vec();

        Input { monkeys }
    }
}

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    divisor: usize,
    if_true: usize,
    if_false: usize,
}

#[derive(Clone, Debug)]
enum Operation {
    Add(usize),
    Multiply(usize),
    Square,
}

impl FromStr for Operation {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "old * old" {
            Ok(Operation::Square)
        } else {
            let num = s
                .split(' ')
                .rev()
                .take(1)
                .next()
                .unwrap()
                .parse::<usize>()
                .unwrap();

            if s.starts_with("old *") {
                Ok(Operation::Multiply(num))
            } else {
                Ok(Operation::Add(num))
            }
        }
    }
}

impl Operation {
    fn apply(&self, item: usize) -> usize {
        match self {
            Operation::Add(x) => item + x,
            Operation::Multiply(x) => item * x,
            Operation::Square => item * item,
        }
    }
}

fn run_round(monkeys: &mut [Monkey], inspections: &mut [usize], lcm: usize, worry_relief: bool) {
    for i in 0..monkeys.len() {
        // Cloning here is lazy but lets us get around the fact we want multiple mutable borrows at once
        let monkey = monkeys[i].clone();
        for &item in &monkey.items {
            inspections[i] += 1;
            let new_item = {
                // We can always make the numbers smaller by taking the modulus wrt the LCM
                let new_item = monkey.operation.apply(item) % lcm;
                if worry_relief {
                    new_item / 3
                } else {
                    new_item
                }
            };
            let test_result = new_item % monkey.divisor == 0;
            if test_result {
                monkeys[monkey.if_true].items.push(new_item);
            } else {
                monkeys[monkey.if_false].items.push(new_item);
            }
        }

        // Remove all the items from the current monkey now they have been processed
        monkeys[i].items = Vec::new();
    }
}

fn calculate_monkey_business(monkeys: &mut [Monkey], rounds: usize, worry_relief: bool) -> usize {
    let n = monkeys.len();
    let mut inspections = vec![0_usize; n];

    // LCM is the product of the divisors, as they are all prime
    let lcm: usize = monkeys.iter().map(|m| m.divisor).product();

    for _ in 0..rounds {
        run_round(monkeys, &mut inspections, lcm, worry_relief);
    }

    inspections.sort_by_key(|&i| Reverse(i));
    inspections.iter().take(2).product()
}

fn part1(input: &Input) -> usize {
    calculate_monkey_business(&mut input.monkeys.clone(), 20, true)
}

fn part2(input: &Input) -> usize {
    calculate_monkey_business(&mut input.monkeys.clone(), 10_000, false)
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
    const TEST_INPUT: &str = include_str!("../../inputs/test_day11.txt");

    #[test]
    fn examples() {
        let input = Input::new(TEST_INPUT);
        assert_eq!(part1(&input), 10605);
        assert_eq!(part2(&input), 2713310158);
    }

    #[test]
    fn answers() {
        let input = Input::new(INPUT);
        assert_eq!(part1(&input), 95472);
        assert_eq!(part2(&input), 17926061332);
    }
}
