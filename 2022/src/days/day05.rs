use itertools::Itertools;
use serde_scan::scan;

const INPUT: &str = include_str!("../../inputs/day05.txt");

#[derive(Clone, Debug, Default)]
struct Stack(Vec<char>);

impl Stack {
    fn add(&mut self, value: char) {
        self.0.push(value)
    }

    fn remove(&mut self) -> Option<char> {
        self.0.pop()
    }

    fn peek(&self) -> Option<char> {
        if self.0.is_empty() {
            None
        } else {
            Some(self.0[self.0.len() - 1])
        }
    }
}

#[derive(Clone, Debug)]
struct Instruction {
    source: usize,
    target: usize,
    count: usize,
}

impl Instruction {
    fn apply_part_1(&self, stacks: &mut [Stack]) {
        let mut i = 0;
        while i < self.count {
            // Remove an element from the source stack
            let source = &mut stacks[self.source];
            let value = source.remove().unwrap();

            // Add the element to the target stack
            let target = &mut stacks[self.target];
            target.add(value);

            i += 1;
        }
    }

    fn apply_part_2(&self, stacks: &mut [Stack]) {
        // For part 2, use a temporary stack to avoid having to change the Stack interface
        // We move the elements to the temp stack one by one, and then move them all to the target stack in the reverse order
        // This has the same effect as picking them up and placing them in the original order
        let mut temp_stack = Stack::default();
        let source = &mut stacks[self.source];

        for _ in 0..self.count {
            // Remove an element from the source stack
            let value = source.remove().unwrap();

            // Add the element to the temp stack
            temp_stack.add(value);
        }

        // Move the items to the target stack
        let target = &mut stacks[self.target];
        while let Some(value) = temp_stack.remove() {
            target.add(value);
        }
    }
}

#[derive(Clone, Debug)]
struct Input {
    stacks: Vec<Stack>,
    instructions: Vec<Instruction>,
}

impl Input {
    fn new(input: &str) -> Input {
        // The two distinct parts of the input are split by a double newline
        let mut input_iters = input.split("\n\n");
        let stacks_str = input_iters.next().unwrap();
        let instructions_str = input_iters.next().unwrap();

        let stacks = parse_stacks(stacks_str);
        let instructions = parse_instructions(instructions_str);

        Input {
            stacks,
            instructions,
        }
    }
}

fn parse_stacks(str: &str) -> Vec<Stack> {
    let mut lines_iter = str.lines().rev();
    // Skip the first line of the reversed iterator, since it is just the stack numbers
    // Only use it to figure out how many stacks there are
    let numbers = lines_iter.next().unwrap();
    let stacks_count = (numbers.len() + 1) / 4;

    // Build our empty stacks
    let mut stacks = vec![Stack::default(); stacks_count];

    // Now iterate through the stack input and add elements to stacks
    for line in lines_iter {
        let chars = line.as_bytes();
        let mut i = 0;
        while i < stacks_count {
            // For each stack, we just need to look at one single char value, offset 4 from one another
            let index = 4 * i + 1;
            // The input has empty space characters at the end, but this ensures no panic if that were not the case
            if chars.len() > index {
                let value = chars[index] as char;
                if value != ' ' {
                    // If there is a value set, add it to the relevant stack
                    stacks[i].add(value);
                }
            }
            i += 1;
        }
    }

    stacks
}

fn parse_instructions(str: &str) -> Vec<Instruction> {
    str.lines()
        .map(|line| {
            let (count, source, target): (_, usize, usize) =
                scan!("move {} from {} to {}" <- line).unwrap();
            Instruction {
                // Ensure stacks are zero indexed
                source: source - 1,
                target: target - 1,
                count,
            }
        })
        .collect_vec()
}

fn part1(input: &Input) -> String {
    let mut stacks = input.stacks.clone();
    for inx in &input.instructions {
        inx.apply_part_1(&mut stacks);
    }

    stacks.iter().map(|stack| stack.peek().unwrap()).collect()
}

fn part2(input: &Input) -> String {
    let mut stacks = input.stacks.clone();
    for inx in &input.instructions {
        inx.apply_part_2(&mut stacks);
    }

    stacks.iter().map(|stack| stack.peek().unwrap()).collect()
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
    const TEST_INPUT: &str = include_str!("../../inputs/test_day05.txt");

    #[test]
    fn examples() {
        let input = Input::new(TEST_INPUT);
        assert_eq!(part1(&input), "CMZ");
        assert_eq!(part2(&input), "MCD");
    }

    #[test]
    fn answers() {
        let input = Input::new(INPUT);
        assert_eq!(part1(&input), "SPFMVDTZT");
        assert_eq!(part2(&input), "ZFSJBPRFP");
    }
}
