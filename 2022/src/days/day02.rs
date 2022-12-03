use std::{cmp::Ordering, convert::Infallible, str::FromStr};

use serde_scan::scan;

const INPUT: &str = include_str!("../../inputs/day02.txt");

#[derive(Clone)]
struct Input {
    rounds: Vec<RoundInput>,
}

#[derive(Clone)]
struct RoundInput {
    opponent: Throw,
    response: Response,
}

impl RoundInput {
    // Part 1: Convert the response to a throw directly
    fn to_round_simple(&self) -> Round {
        self.to_round_with_response(self.response.to_throw())
    }

    // Part 2: Make a decision on what to throw based on the opponent's throw
    fn to_round_decision(&self) -> Round {
        self.to_round_with_response(self.response.decide_throw(&self.opponent))
    }

    fn to_round_with_response(&self, response: Throw) -> Round {
        Round {
            opponent: self.opponent.clone(),
            response,
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
enum Response {
    X,
    Y,
    Z,
}

impl Response {
    fn to_throw(&self) -> Throw {
        match self {
            Response::X => Throw::Rock,
            Response::Y => Throw::Paper,
            Response::Z => Throw::Scissors,
        }
    }

    fn decide_throw(&self, opponent: &Throw) -> Throw {
        match self {
            Response::X => Throw::losing_to(opponent),
            Response::Y => opponent.clone(),
            Response::Z => Throw::beating(opponent),
        }
    }
}

impl FromStr for Response {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Response::X),
            "Y" => Ok(Response::Y),
            "Z" => Ok(Response::Z),
            s => panic!("Unexpected input for response {}", s),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Throw {
    Rock,
    Paper,
    Scissors,
}

impl Throw {
    fn score(&self) -> usize {
        match self {
            Throw::Rock => 1,
            Throw::Paper => 2,
            Throw::Scissors => 3,
        }
    }

    fn beats(&self, opponent: &Throw) -> bool {
        *self == Throw::beating(opponent)
    }

    // Give us the throw that beats the opponent's throw
    fn beating(opponent: &Throw) -> Throw {
        match opponent {
            Throw::Rock => Throw::Paper,
            Throw::Paper => Throw::Scissors,
            Throw::Scissors => Throw::Rock,
        }
    }

    // Give us the throw that loses to the opponent's throw
    fn losing_to(opponent: &Throw) -> Throw {
        match opponent {
            Throw::Rock => Throw::Scissors,
            Throw::Paper => Throw::Rock,
            Throw::Scissors => Throw::Paper,
        }
    }
}

impl PartialOrd for Throw {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ord = if self == other {
            Ordering::Equal
        } else if self.beats(other) {
            Ordering::Greater
        } else {
            Ordering::Less
        };

        Some(ord)
    }
}

impl Ord for Throw {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl FromStr for Throw {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Throw::Rock),
            "B" => Ok(Throw::Paper),
            "C" => Ok(Throw::Scissors),
            s => panic!("Unexpected input for throw {}", s),
        }
    }
}

#[derive(Clone, Debug)]
struct Round {
    opponent: Throw,
    response: Throw,
}

impl Round {
    fn score(&self) -> usize {
        let round = match self.response.cmp(&self.opponent) {
            Ordering::Less => 0,
            Ordering::Equal => 3,
            Ordering::Greater => 6,
        };
        let throw = self.response.score();
        round + throw
    }
}

impl Input {
    fn new(input: &str) -> Input {
        Input {
            rounds: input
                .lines()
                .map(|line| {
                    let (opponent_str, response_str): (&str, &str) =
                        scan!("{} {}" <- line).unwrap();
                    let opponent = opponent_str.parse().unwrap();
                    let response = response_str.parse().unwrap();
                    RoundInput { opponent, response }
                })
                .collect(),
        }
    }
}

fn part1(input: &Input) -> usize {
    input
        .rounds
        .iter()
        .map(|r| {
            let round = r.to_round_simple();
            round.score()
        })
        .sum()
}

fn part2(input: &Input) -> usize {
    input
        .rounds
        .iter()
        .map(|r| {
            let round = r.to_round_decision();
            round.score()
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
    const TEST_INPUT: &str = include_str!("../../inputs/test_day02.txt");

    #[test]
    fn examples() {
        let input = Input::new(TEST_INPUT);
        assert_eq!(part1(&input), 15);
        assert_eq!(part2(&input), 12);
    }

    #[test]
    fn answers() {
        let input = Input::new(INPUT);
        assert_eq!(part1(&input), 12679);
        assert_eq!(part2(&input), 14470);
    }
}
