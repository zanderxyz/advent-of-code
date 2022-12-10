use std::collections::HashSet;

use serde_scan::scan;

const INPUT: &str = include_str!("../../inputs/day09.txt");

#[derive(Clone, Debug)]
struct Input {
    actions: Vec<Direction>,
}

impl Input {
    fn new(input: &str) -> Input {
        Input {
            actions: input
                .lines()
                .flat_map(|line| {
                    let (dir, distance): (char, usize) = scan!("{} {}" <- line).unwrap();
                    let direction = dir.into();
                    vec![direction; distance]
                })
                .collect(),
        }
    }
}

#[derive(Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'R' => Self::Right,
            'L' => Self::Left,
            'U' => Self::Up,
            'D' => Self::Down,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn apply(&mut self, action: &Direction) {
        match action {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }

    fn is_touching(&self, other: &Self) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }

    fn catch_up(&mut self, head: &Self) {
        // Move one step closer in both dimensions
        let x_diff = self.x - head.x;
        let y_diff = self.y - head.y;
        match x_diff {
            -1 | -2 => self.apply(&Direction::Right),
            1 | 2 => self.apply(&Direction::Left),
            _ => {}
        }
        match y_diff {
            -1 | -2 => self.apply(&Direction::Up),
            1 | 2 => self.apply(&Direction::Down),
            _ => {}
        }
    }
}

#[derive(Debug)]
struct Rope<const N: usize> {
    points: [Position; N],
    visited: HashSet<Position>,
}

impl<const N: usize> Rope<N> {
    fn new() -> Self {
        Self {
            points: [Position::default(); N],
            visited: HashSet::from_iter([Position::default()]),
        }
    }

    fn apply(&mut self, action: &Direction) {
        // First move the head
        self.points[0].apply(action);

        for i in 0..N - 1 {
            let point = self.points[i];
            let next_point = &mut self.points[i + 1];
            if !point.is_touching(next_point) {
                // Catch the tail up to the head
                next_point.catch_up(&point);
            }
            if i == N - 2 {
                self.visited.insert(*next_point);
            }
        }
    }

    fn visited(&self) -> usize {
        self.visited.len()
    }
}

fn part1(input: &Input) -> usize {
    let mut rope = Rope::<2>::new();
    for action in input.actions.iter() {
        rope.apply(action);
    }
    rope.visited()
}

fn part2(input: &Input) -> usize {
    let mut rope = Rope::<10>::new();
    for action in input.actions.iter() {
        rope.apply(action);
    }

    rope.visited()
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
    const TEST_INPUT: &str = include_str!("../../inputs/test_day09.txt");
    const TEST_INPUT_2: &str = include_str!("../../inputs/test_day09_2.txt");

    #[test]
    fn examples() {
        let input = Input::new(TEST_INPUT);
        assert_eq!(part1(&input), 13);
        assert_eq!(part2(&input), 1);
    }

    #[test]
    fn examples_2() {
        let input = Input::new(TEST_INPUT_2);
        assert_eq!(part2(&input), 36);
    }

    #[test]
    fn answers() {
        let input = Input::new(INPUT);
        assert_eq!(part1(&input), 6642);
        assert_eq!(part2(&input), 2765);
    }
}
