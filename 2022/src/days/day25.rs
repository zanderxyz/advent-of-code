use std::collections::VecDeque;

const INPUT: &str = include_str!("../../inputs/day25.txt");

#[derive(Clone, Debug)]
struct Input {
    numbers: Vec<isize>,
}

impl Input {
    fn new(input: &str) -> Input {
        let numbers = input
            .lines()
            .map(|line| {
                let mut number = 0;
                let length = line.len() - 1;
                let mut current_col = 5_isize.pow(length.try_into().unwrap());
                let digits = line.chars().map(SnafuDigit::from_char);
                for c in digits {
                    number += c.to_digit() * current_col;
                    current_col /= 5;
                }
                number
            })
            .collect();

        Input { numbers }
    }
}

#[derive(Clone, Debug)]
enum SnafuDigit {
    Two,
    One,
    Zero,
    Minus,
    DoubleMinus,
}

impl SnafuDigit {
    fn from_char(c: char) -> Self {
        match c {
            '2' => SnafuDigit::Two,
            '1' => SnafuDigit::One,
            '0' => SnafuDigit::Zero,
            '-' => SnafuDigit::Minus,
            '=' => SnafuDigit::DoubleMinus,
            _ => unreachable!(),
        }
    }

    fn to_char(&self) -> char {
        match self {
            SnafuDigit::Two => '2',
            SnafuDigit::One => '1',
            SnafuDigit::Zero => '0',
            SnafuDigit::Minus => '-',
            SnafuDigit::DoubleMinus => '=',
        }
    }

    fn from_digit(i: isize) -> Self {
        match i {
            -2 => SnafuDigit::DoubleMinus,
            -1 => SnafuDigit::Minus,
            0 => SnafuDigit::Zero,
            1 => SnafuDigit::One,
            2 => SnafuDigit::Two,
            _ => unreachable!(),
        }
    }

    fn to_digit(&self) -> isize {
        match self {
            SnafuDigit::Two => 2,
            SnafuDigit::One => 1,
            SnafuDigit::Zero => 0,
            SnafuDigit::Minus => -1,
            SnafuDigit::DoubleMinus => -2,
        }
    }
}

fn decimal_to_snafu(number: isize) -> VecDeque<SnafuDigit> {
    let mut output = VecDeque::new();
    decimal_to_snafu_rec(number, &mut output);
    output
}

fn decimal_to_snafu_rec(number: isize, output: &mut VecDeque<SnafuDigit>) {
    let div = (number + 2) / 5;
    let rem = (number + 2) % 5 - 2;
    output.push_front(SnafuDigit::from_digit(rem));
    if div > 0 {
        decimal_to_snafu_rec(div, output)
    }
}

fn part1(input: &Input) -> String {
    let decimal = input.numbers.iter().sum::<isize>();
    let snafu: String = decimal_to_snafu(decimal)
        .iter()
        .map(SnafuDigit::to_char)
        .collect();
    snafu
}

pub fn main() {
    let input = Input::new(INPUT);
    let answer1 = part1(&input);
    println!("Part 1: {}", answer1);
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/test_day25.txt");

    #[test]
    fn examples() {
        let input = Input::new(TEST_INPUT);
        assert_eq!(part1(&input), "2=-1=0".to_owned());
    }

    #[test]
    fn answers() {
        let input = Input::new(INPUT);
        assert_eq!(part1(&input), "122-0==-=211==-2-200".to_owned());
    }
}
