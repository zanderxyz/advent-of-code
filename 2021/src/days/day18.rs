use std::{fmt, ops::Add};

use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day18.txt");

#[derive(Clone)]
struct Input {
    pub numbers: Vec<SnailNumber>,
}

impl Input {
    pub fn new(input: &str) -> Input {
        let numbers = input
            .lines()
            .map(|line| SnailNumber::read(line.as_bytes()).0)
            .collect();

        Input { numbers }
    }
}

// This struct and enum basically describe a binary tree
#[derive(Clone, PartialEq)]
struct SnailNumber {
    left: SnailItem,
    right: SnailItem,
}

#[derive(Clone, PartialEq)]
enum SnailItem {
    Number(usize),
    // Has to be boxed to avoid an infinitely-sized type
    SnailNumber(Box<SnailNumber>),
}

// Useful for test failures
impl fmt::Debug for SnailNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{},{}]", self.left, self.right)
    }
}

impl fmt::Display for SnailNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{},{}]", self.left, self.right)
    }
}

enum ExplodeStatus {
    Exploding(usize),
    No,
    Done,
}

enum SplitStatus {
    Splitting(SnailNumber),
    No,
    Done,
}

impl SnailNumber {
    pub fn read(input: &[u8]) -> (SnailNumber, usize) {
        // The first char of any Snailfish number is a [
        assert_eq!(input[0] as char, '[');

        // Read the left hand side, then a comma
        let (left, left_read) = SnailItem::read(&input[1..]);
        assert_eq!(input[1 + left_read] as char, ',');

        // Read the right hand side, then a ]
        let (right, right_read) = SnailItem::read(&input[2 + left_read..]);
        assert_eq!(input[2 + left_read + right_read] as char, ']');

        let result = SnailNumber { left, right };
        (result, 3 + left_read + right_read)
    }

    pub fn reduce(&mut self) {
        if self.try_explode() {
            return self.reduce();
        }
        if self.try_split() {
            self.reduce();
        }
    }

    pub fn try_explode(&mut self) -> bool {
        !matches!(self.try_explode_rec(0, None), ExplodeStatus::No)
    }

    pub fn try_explode_rec(&mut self, depth: usize, prev: Option<&mut SnailItem>) -> ExplodeStatus {
        match self.left.try_explode(depth, prev) {
            ExplodeStatus::Exploding(right) => {
                // Add the right side of the exploding pair to the number to the right
                self.right.inc_left(right);

                if depth == 3 {
                    // Set the exploding pair to 0
                    self.left = SnailItem::Number(0);
                }

                return ExplodeStatus::Done;
            }
            ExplodeStatus::Done => return ExplodeStatus::Done,
            ExplodeStatus::No => {}
        }

        match self.right.try_explode(depth, Some(&mut self.left)) {
            ExplodeStatus::Exploding(right) => {
                if depth == 3 {
                    // Set the exploding pair to 0
                    self.right = SnailItem::Number(0);
                }

                // Add the right side of the exploding pair to the number to the right
                return ExplodeStatus::Exploding(right);
            }
            ExplodeStatus::Done => return ExplodeStatus::Done,
            ExplodeStatus::No => {}
        }

        ExplodeStatus::No
    }

    pub fn try_split(&mut self) -> bool {
        !matches!(self.try_split_rec(), SplitStatus::No)
    }

    pub fn try_split_rec(&mut self) -> SplitStatus {
        match self.left.try_split() {
            SplitStatus::Splitting(number) => {
                self.left = SnailItem::SnailNumber(number.into());
                return SplitStatus::Done;
            }
            SplitStatus::Done => return SplitStatus::Done,
            SplitStatus::No => {}
        }

        match self.right.try_split() {
            SplitStatus::Splitting(number) => {
                self.right = SnailItem::SnailNumber(number.into());
                return SplitStatus::Done;
            }
            SplitStatus::Done => return SplitStatus::Done,
            SplitStatus::No => {}
        }

        SplitStatus::No
    }

    pub fn mag(&self) -> usize {
        let left_mag = self.left.mag();
        let right_mag = self.right.mag();
        left_mag * 3 + right_mag * 2
    }
}

impl Add for SnailNumber {
    type Output = SnailNumber;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = SnailNumber {
            left: SnailItem::SnailNumber(self.into()),
            right: SnailItem::SnailNumber(rhs.into()),
        };
        result.reduce();
        result
    }
}

impl fmt::Debug for SnailItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{}", n),
            Self::SnailNumber(n) => write!(f, "{}", n),
        }
    }
}

impl fmt::Display for SnailItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{}", n)?,
            Self::SnailNumber(n) => write!(f, "{}", n)?,
        }
        Ok(())
    }
}

impl SnailItem {
    pub fn read(input: &[u8]) -> (SnailItem, usize) {
        match input[0] as char {
            '[' => {
                // It's a nested snail number
                let (snail_number, read) = SnailNumber::read(&input[0..]);
                let item = SnailItem::SnailNumber(snail_number.into());
                (item, read)
            }
            _ => {
                // It's a number
                // Due to the tests, we allow parsing more than one character here
                // Look for the next , or ] char
                let index = input
                    .iter()
                    .position(|&c| (c as char) == ']' || (c as char) == ',')
                    .unwrap();

                // Find the substring before the ending character
                let number_str = std::str::from_utf8(&input[0..index]).unwrap();

                // Turn it into an integer
                let number = number_str.parse::<usize>().unwrap();
                let item = SnailItem::Number(number);
                (item, index)
            }
        }
    }

    pub fn mag(&self) -> usize {
        match self {
            Self::Number(n) => *n,
            Self::SnailNumber(n) => n.mag(),
        }
    }

    pub fn get_number(&self) -> usize {
        match self {
            SnailItem::Number(number) => *number,
            SnailItem::SnailNumber(_) => unreachable!(),
        }
    }

    pub fn try_explode(&mut self, depth: usize, prev: Option<&mut SnailItem>) -> ExplodeStatus {
        match self {
            SnailItem::Number(_) => ExplodeStatus::No,
            SnailItem::SnailNumber(nested) => {
                if depth == 3 {
                    // The nested snail number will explode
                    let left = nested.left.get_number();
                    let right = nested.right.get_number();

                    // Add the left side to the number immediately to the left
                    match prev {
                        None => {}
                        Some(item) => {
                            item.inc_right(left);
                        }
                    }

                    // Return the right number, so it can be added by the parent of this
                    ExplodeStatus::Exploding(right)
                } else {
                    nested.try_explode_rec(depth + 1, prev)
                }
            }
        }
    }

    pub fn inc_left(&mut self, num: usize) {
        match self {
            SnailItem::Number(n) => *n += num,
            SnailItem::SnailNumber(nested) => nested.left.inc_left(num),
        }
    }

    pub fn inc_right(&mut self, num: usize) {
        match self {
            SnailItem::Number(n) => *n += num,
            SnailItem::SnailNumber(nested) => nested.right.inc_right(num),
        }
    }

    pub fn try_split(&mut self) -> SplitStatus {
        match self {
            SnailItem::Number(n) => {
                if *n > 9 {
                    let new_left = *n / 2;
                    let new_right = if *n % 2 == 0 { *n / 2 } else { *n / 2 + 1 };
                    let new = SnailNumber {
                        left: SnailItem::Number(new_left),
                        right: SnailItem::Number(new_right),
                    };
                    SplitStatus::Splitting(new)
                } else {
                    SplitStatus::No
                }
            }
            SnailItem::SnailNumber(n) => n.try_split_rec(),
        }
    }
}

fn part1(input: &Input) -> usize {
    input
        .numbers
        .clone()
        .into_iter()
        .reduce(|a, b| a + b)
        .unwrap()
        .mag()
}

fn part2(input: &Input) -> usize {
    input
        .numbers
        .clone()
        .into_iter()
        .tuple_combinations()
        .flat_map(|(a, b)| {
            let c = a.clone() + b.clone();
            let d = b + a;
            [c.mag(), d.mag()]
        })
        .max()
        .unwrap()
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
    const TEST_INPUT: &str = include_str!("../../inputs/test_day18.txt");

    #[test]
    pub fn parsing() {
        let (single, read) = SnailItem::read("1,2]".as_bytes());
        assert_eq!(single, SnailItem::Number(1));
        assert_eq!(read, 1);

        let (nested, read2) = SnailItem::read("[1,2],3]".as_bytes());
        let expected = SnailNumber {
            left: SnailItem::Number(1),
            right: SnailItem::Number(2),
        };
        assert_eq!(nested, SnailItem::SnailNumber(expected.into()));
        assert_eq!(read2, 5);
    }

    fn assert_mag(input: &str, expected: usize) {
        let (num, _) = SnailItem::read(input.as_bytes());
        assert_eq!(num.mag(), expected);
    }

    #[test]
    pub fn magnitude() {
        assert_mag("[[1,2],[[3,4],5]]", 143);
        assert_mag("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384);
        assert_mag(
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
            3488,
        );
        assert_mag(
            "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]",
            4140,
        );
    }

    fn parse(input: &str) -> SnailNumber {
        SnailNumber::read(input.as_bytes()).0
    }

    fn assert_explode(input: &str, expected: &str) {
        let mut number = parse(input);
        assert!(number.try_explode());
        assert_eq!(number, parse(expected));
    }

    #[test]
    pub fn explode() {
        assert_explode("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]");
        assert_explode("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]");
        assert_explode("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]");
        assert_explode(
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
        );
        assert_explode(
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        );
    }

    fn assert_split(input: &str, expected: &str) {
        let mut number = parse(input);
        assert!(number.try_split());
        assert_eq!(number, parse(expected));
    }

    #[test]
    pub fn split() {
        assert_split(
            "[[[[0,7],4],[15,[0,13]]],[1,1]]",
            "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
        );
        assert_split(
            "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
            "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]",
        );
    }

    fn assert_reduce(input: &str, expected: &str) {
        let mut number = parse(input);
        number.reduce();
        assert_eq!(number, parse(expected));
    }

    #[test]
    pub fn reduce() {
        assert_reduce(
            "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]",
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
        );
    }

    #[test]
    pub fn fails() {
        assert_reduce(
            "[[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]",
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
        );
    }

    fn assert_sum(inputs: &[&str], expected: &str) {
        let sum = inputs
            .iter()
            .map(|&input| parse(input))
            .reduce(|a, b| a + b)
            .unwrap();
        assert_eq!(sum, parse(expected));
    }

    #[test]
    pub fn fold() {
        assert_sum(
            &["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]"],
            "[[[[3,0],[5,3]],[4,4]],[5,5]]",
        );

        assert_sum(
            &[
                "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
                "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
                "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
                "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
                "[7,[5,[[3,8],[1,4]]]]",
                "[[2,[2,2]],[8,[8,1]]]",
                "[2,9]",
                "[1,[[[9,3],9],[[9,0],[0,7]]]]",
                "[[[5,[7,4]],7],1]",
                "[[[[4,2],2],6],[8,7]]",
            ],
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
        );
    }

    #[test]
    pub fn sum_part2() {
        assert_sum(
            &[
                "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
                "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
            ],
            "[[[[7,8],[6,6]],[[6,0],[7,7]]],[[[7,8],[8,8]],[[7,9],[0,6]]]]",
        );

        assert_mag(
            "[[[[7,8],[6,6]],[[6,0],[7,7]]],[[[7,8],[8,8]],[[7,9],[0,6]]]]",
            3993,
        );
    }

    #[test]
    pub fn examples() {
        let input = Input::new(TEST_INPUT);
        assert_eq!(part1(&input), 4140);
        assert_eq!(part2(&input), 3993);
    }

    #[test]
    pub fn answers() {
        let input = Input::new(INPUT);
        assert_eq!(part1(&input), 3793);
        assert_eq!(part2(&input), 4695);
    }
}
