use itertools::Itertools;
use num_digitize::FromDigits;
use serde_scan::scan;

const INPUT: &str = include_str!("../../inputs/day24.txt");

// All this parsing turned out to be entirely unnecessary
#[derive(Debug, Clone, Copy)]
enum Register {
    W = 0,
    X,
    Y,
    Z,
}

impl Register {
    fn new(s: char) -> Self {
        Register::maybe(s).unwrap()
    }

    fn maybe(s: char) -> Option<Self> {
        match s {
            'w' => Some(Register::W),
            'x' => Some(Register::X),
            'y' => Some(Register::Y),
            'z' => Some(Register::Z),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Entry {
    Register(Register),
    Number(isize),
}

impl Entry {
    fn new(s: &str) -> Self {
        let c = s.chars().next().unwrap();
        if let Some(register) = Register::maybe(c) {
            Entry::Register(register)
        } else {
            let n = s.parse::<isize>().unwrap();
            Entry::Number(n)
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Inp(Register),
    Add(Register, Entry),
    Mul(Register, Entry),
    Div(Register, Entry),
    Mod(Register, Entry),
    Eql(Register, Entry),
}

impl From<&str> for Instruction {
    fn from(line: &str) -> Self {
        if line.starts_with("inp") {
            let reg = scan!("inp {}" <- line).unwrap();
            Instruction::Inp(Register::new(reg))
        } else if let Ok((a, b)) = scan!("add {} {}" <- line) {
            Instruction::Add(Register::new(a), Entry::new(b))
        } else if let Ok((a, b)) = scan!("mul {} {}" <- line) {
            Instruction::Mul(Register::new(a), Entry::new(b))
        } else if let Ok((a, b)) = scan!("div {} {}" <- line) {
            Instruction::Div(Register::new(a), Entry::new(b))
        } else if let Ok((a, b)) = scan!("mod {} {}" <- line) {
            Instruction::Mod(Register::new(a), Entry::new(b))
        } else if let Ok((a, b)) = scan!("eql {} {}" <- line) {
            Instruction::Eql(Register::new(a), Entry::new(b))
        } else {
            unreachable!()
        }
    }
}

#[derive(Clone)]
struct Input {
    pub instructions: Vec<Instruction>,
}

impl Input {
    pub fn new(input: &str) -> Input {
        Input {
            instructions: input.lines().map(|line| line.into()).collect(),
        }
    }
}

fn extract_add_number(instruction: &Instruction) -> isize {
    match instruction {
        Instruction::Add(_, entry) => {
            if let Entry::Number(n) = entry {
                *n
            } else {
                unreachable!()
            }
        }
        _ => unreachable!(),
    }
}

fn get_variables(instructions: &[Instruction]) -> Vec<(usize, isize)> {
    // The input is grouped into sections of 14 groups of 18 instructions
    // There are two variables that matter in each group of instructions
    let mut level: usize = 0;

    (0..14)
        .map(|group| {
            let x = extract_add_number(&instructions[18 * group + 5]);
            let y = extract_add_number(&instructions[18 * group + 15]);
            if x > 0 {
                level += 1;
                (level, y)
            } else {
                level -= 1;
                (level + 1, x)
            }
        })
        .collect()
}

fn process_variables(variables: Vec<(usize, isize)>) -> Vec<(usize, usize, isize)> {
    variables
        .into_iter()
        .enumerate()
        .sorted_by(|(_, (level1, _)), (_, (level2, _))| level1.cmp(level2))
        .map(|(i, (_, x))| (i, x))
        .chunks(2)
        .into_iter()
        .map(|mut iter| {
            let (left, x) = iter.next().unwrap();
            let (right, y) = iter.next().unwrap();
            (left, right, x + y)
        })
        .collect()
}

fn part1(input: &Input) -> i64 {
    let variables = get_variables(&input.instructions);
    let processed = process_variables(variables);

    processed
        .into_iter()
        .flat_map(|(left, right, d)| {
            if d > 0 {
                [(left, 9 - d), (right, 9)]
            } else {
                [(left, 9), (right, 9 + d)]
            }
        })
        .sorted()
        .map(|(_, y)| y)
        .from_digits()
}

fn part2(input: &Input) -> i64 {
    let variables = get_variables(&input.instructions);
    let processed = process_variables(variables);

    processed
        .into_iter()
        .flat_map(|(left, right, d)| {
            if d > 0 {
                [(left, 1), (right, 1 + d)]
            } else {
                [(left, 1 - d), (right, 1)]
            }
        })
        .sorted()
        .map(|(_, y)| y)
        .from_digits()
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

    #[test]
    pub fn answers() {
        let input = Input::new(INPUT);
        assert_eq!(part1(&input), 99999795919456);
        assert_eq!(part2(&input), 45311191516111);
    }
}
