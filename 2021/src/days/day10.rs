use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day10.txt");

#[derive(Clone)]
struct Input {
    pub lines: Vec<Vec<char>>,
}

impl Input {
    pub fn new(input: &str) -> Input {
        Input {
            lines: input.lines().map(|line| line.chars().collect()).collect(),
        }
    }
}

enum ParseResult {
    Incomplete(Vec<char>),
    Corrupted(char),
}

impl ParseResult {
    fn corrupted_score(self) -> Option<usize> {
        match self {
            ParseResult::Incomplete(_) => None,
            ParseResult::Corrupted(char) => {
                let score = corrupted_score(&char)
                    .unwrap_or_else(|| panic!("Unexpected illegal char {}", char));
                Some(score)
            }
        }
    }

    fn completion_score(self) -> Option<usize> {
        match self {
            ParseResult::Corrupted(_) => None,
            ParseResult::Incomplete(stack) => stack
                .iter()
                .rev()
                .map(|c| completion_score(c))
                .reduce(|acc, score| acc * 5 + score),
        }
    }
}

fn corrupted_score(char: &char) -> Option<usize> {
    let score = match char {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => return None,
    };
    Some(score)
}

fn completion_score(char: &char) -> usize {
    match char {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!("Unexpected char in stack"),
    }
}

fn parse(line: &[char]) -> ParseResult {
    let mut stack: Vec<char> = Vec::new();

    for char in line {
        // If char is a closing char, it must match the top of the stack
        if is_closing(char) {
            if stack.is_empty() {
                return ParseResult::Corrupted(*char);
            }
            if matching_char(char) == *stack.last().unwrap() {
                // Pop the top of the stack
                stack.pop();
            } else {
                return ParseResult::Corrupted(*char);
            }
        } else {
            stack.push(*char);
        }
    }

    ParseResult::Incomplete(stack)
}

fn is_closing(char: &char) -> bool {
    corrupted_score(char).is_some()
}

fn matching_char(char: &char) -> char {
    match char {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => panic!("Unexpected char {}", char),
    }
}

fn part1(input: &Input) -> usize {
    input
        .lines
        .iter()
        .map(|line| parse(line))
        .filter_map(|result| result.corrupted_score())
        .sum()
}

fn part2(input: &Input) -> usize {
    let sorted = input
        .lines
        .iter()
        .map(|line| parse(line))
        .filter_map(|result| result.completion_score())
        .sorted()
        .collect::<Vec<usize>>();
    let middle_index = (sorted.len() - 1) / 2;
    sorted[middle_index]
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
    const TEST_INPUT: &str = include_str!("../../inputs/test_day10.txt");

    #[test]
    pub fn examples() {
        let input = Input::new(TEST_INPUT);
        assert_eq!(part1(&input), 26397);
        assert_eq!(part2(&input), 288957);
    }

    #[test]
    pub fn answers() {
        let input = Input::new(INPUT);
        assert_eq!(part1(&input), 294195);
        assert_eq!(part2(&input), 3490802734);
    }
}
