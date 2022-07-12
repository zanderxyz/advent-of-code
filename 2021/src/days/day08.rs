use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::helpers::digits::from_digits;

const INPUT: &str = include_str!("../../inputs/day08.txt");

const POSSIBLES: [u8; 8] = [0, 0, 1, 1, 1, 3, 3, 1];

type Segment = usize;
type Digit = HashSet<Segment>;

fn from_char(char: char) -> Segment {
    match char {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        _ => panic!("Unexpected char {}", char),
    }
}

#[derive(Clone)]
struct Sample {
    // Numbers 1-10 in some order
    pub inputs: [Digit; 10],
    pub outputs: [Digit; 4],
}

#[derive(Clone)]
struct Input {
    pub lines: Vec<Sample>,
}

impl Input {
    pub fn new(input: &str) -> Input {
        let lines = input
            .lines()
            .map(|line| {
                let mut split = line.split(" | ").map(|s| s.split(' '));
                let inputs = split
                    .next()
                    .unwrap()
                    .map(|s| s.chars().map(from_char).collect::<Digit>());

                let outputs = split
                    .next()
                    .unwrap()
                    .map(|s| s.chars().map(from_char).collect::<Digit>());

                assert!(split.next().is_none());

                let inputs = inputs.collect::<Vec<Digit>>().try_into().unwrap();
                let outputs = outputs.collect::<Vec<Digit>>().try_into().unwrap();

                Sample { inputs, outputs }
            })
            .collect();

        Input { lines }
    }
}

fn part1(input: &Input) -> usize {
    input
        .lines
        .iter()
        .flat_map(|line| {
            // For each line, only include the output digits that can only be a single number
            line.outputs.iter().filter(|s| POSSIBLES[s.len()] == 1)
        })
        .count()
}

fn digits() -> [Digit; 10] {
    /*
     * Segments that are turned on for each digit, numbered as follows:
     *
     *   000
     *  1   2
     *  1   2
     *   333
     *  4   5
     *  4   5
     *   666
     */
    [
        HashSet::from_iter([0, 1, 2, 4, 5, 6]),    // 0
        HashSet::from_iter([2, 5]),                // 1
        HashSet::from_iter([0, 2, 3, 4, 6]),       // 2
        HashSet::from_iter([0, 2, 3, 5, 6]),       // 3
        HashSet::from_iter([1, 2, 3, 5]),          // 4
        HashSet::from_iter([0, 1, 3, 5, 6]),       // 5
        HashSet::from_iter([0, 1, 3, 4, 5, 6]),    // 6
        HashSet::from_iter([0, 2, 5]),             // 7
        HashSet::from_iter([0, 1, 2, 3, 4, 5, 6]), // 8
        HashSet::from_iter([0, 1, 2, 3, 5, 6]),    // 9
    ]
}

// Count of digits in which each segment occurs
fn count_uses_of_each_segment(digits: &[Digit; 10]) -> [usize; 7] {
    (0usize..7)
        .map(|segment| {
            digits
                .iter()
                .filter(|&digit| digit.contains(&segment))
                .count()
        })
        .collect::<Vec<Segment>>()
        .try_into()
        .unwrap()
}

// Calculate a unique score for each digit
fn digit_unique_scores() -> [usize; 10] {
    let digits = digits();
    let uses_of_each_segment = count_uses_of_each_segment(&digits);
    let scores = score_each_digit(&digits, uses_of_each_segment);

    // Ensure each digit has a unique score
    assert!(scores.iter().unique().count() == 10);
    scores
}

// Sum the segment count for each segment in each digit
fn score_each_digit<const N: usize>(
    digits: &[Digit; N],
    uses_of_each_segment: [usize; 7],
) -> [usize; N] {
    digits
        .iter()
        .map(|digit| {
            digit
                .iter()
                .map(|&segment| uses_of_each_segment[segment])
                .sum()
        })
        .collect::<Vec<usize>>()
        .try_into()
        .unwrap()
}

fn digit_scores_map() -> HashMap<usize, usize> {
    let digit_scores = digit_unique_scores();

    // Invert this array, so we can look up a score to get the original digit
    let mut digit_scores_map = HashMap::new();
    for (digit, &score) in digit_scores.iter().enumerate() {
        digit_scores_map.insert(score, digit);
    }
    digit_scores_map
}

fn part2(input: &Input) -> i64 {
    // On each line, the input includes all 10 digits
    // We need to decode the outputs

    // First calculate a score for each digit. This is unique, which is lucky.
    // We score each segment based on how many digits use it, then score each digit based on the scores of each segment in it
    let digit_scores = digit_scores_map();

    input
        .lines
        .iter()
        .map(|line| {
            // Count the number of times each segment occurs in the input
            let uses_of_each_segment = count_uses_of_each_segment(&line.inputs);

            // Now we have enough information to decode any digit (but we only bother with the output)
            let digits = score_each_digit(&line.outputs, uses_of_each_segment).map(|score| {
                // This is the same unique score we got earlier, so we can just look it up to find the digit
                *digit_scores.get(&score).unwrap() as isize
            });

            from_digits(digits.into_iter())
        })
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
    const TEST_INPUT: &str = include_str!("../../inputs/test_day08.txt");

    #[test]
    pub fn examples() {
        let input = Input::new(TEST_INPUT);
        assert_eq!(part1(&input), 26);
        assert_eq!(part2(&input), 61229);
    }

    #[test]
    pub fn answers() {
        let input = Input::new(INPUT);
        assert_eq!(part1(&input), 397);
        assert_eq!(part2(&input), 1027422);
    }
}
