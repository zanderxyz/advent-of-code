use std::str::FromStr;

const INPUT: &str = include_str!("../../inputs/day02.txt");

#[derive(Clone)]
pub struct Input {
    pub actions: Vec<Action>,
}

impl Input {
    pub fn new(input: &str) -> Input {
        Input {
            actions: input
                .lines()
                .map(|line| line.parse::<Action>().unwrap())
                .collect(),
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    InvalidDirection,
    InvalidDistance,
}

impl FromStr for Action {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir_str, dist_str) = s.split_once(" ").expect("Missing whitespace");

        let direction = str_to_direction(&dir_str).ok_or(ParseError::InvalidDirection)?;
        let distance = dist_str
            .parse::<isize>()
            .map_err(|_| ParseError::InvalidDistance)?;

        Ok(Action {
            direction,
            distance,
        })
    }
}

fn str_to_direction(c: &str) -> Option<Direction> {
    match c {
        "forward" => Some(Direction::Forward),
        "down" => Some(Direction::Down),
        "up" => Some(Direction::Up),
        _ => None,
    }
}

#[derive(Default)]
struct Submarine {
    pub position: isize,
    pub depth: isize,
    pub aim: isize,
}

impl Submarine {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn travel(&mut self, action: &Action) {
        match action.direction {
            Direction::Down => self.depth += action.distance,
            Direction::Up => self.depth -= action.distance,
            Direction::Forward => self.position += action.distance,
        }
    }

    pub fn apply(&mut self, action: &Action) {
        match action.direction {
            Direction::Down => self.aim += action.distance,
            Direction::Up => self.aim -= action.distance,
            Direction::Forward => {
                self.position += action.distance;
                self.depth += self.aim * action.distance;
            }
        }
    }
}

#[derive(Clone)]
pub enum Direction {
    Forward,
    Down,
    Up,
}

#[derive(Clone)]
pub struct Action {
    pub direction: Direction,
    pub distance: isize,
}

fn run_actions_on_submarine(
    actions: &Vec<Action>,
    update_submarine: fn(&mut Submarine, &Action),
) -> isize {
    let mut submarine = Submarine::new();
    for action in actions {
        update_submarine(&mut submarine, action);
    }
    submarine.position * submarine.depth
}

pub fn part1(input: &Input) -> isize {
    run_actions_on_submarine(&input.actions, |sub, action| sub.travel(action))
}

pub fn part2(input: &Input) -> isize {
    run_actions_on_submarine(&input.actions, |sub, action| sub.apply(action))
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
    pub fn examples() {
        let input = Input::new(TEST_INPUT);
        assert_eq!(part1(&input), 150);
        assert_eq!(part2(&input), 900);
    }

    #[test]
    pub fn answers() {
        let input = Input::new(INPUT);
        assert_eq!(part1(&input), 1746616);
        assert_eq!(part2(&input), 1741971043);
    }
}
