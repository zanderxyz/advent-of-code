use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::Add,
};

const INPUT: &str = include_str!("../../inputs/day23.txt");

#[derive(Clone, Debug)]
struct Input {
    elves: Vec<Elf>,
}

impl Input {
    fn new(input: &str) -> Input {
        let elves = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| (c, (x, y))))
            .filter(|(c, _)| *c == '#')
            .map(|(_, (x, y))| Position {
                x: x as isize,
                y: -(y as isize),
            })
            .collect();

        Input { elves }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn neighbours(&self) -> [Position; 8] {
        [
            *self + Position::new(-1, -1),
            *self + Position::new(-1, 0),
            *self + Position::new(-1, 1),
            *self + Position::new(0, -1),
            *self + Position::new(0, 1),
            *self + Position::new(1, -1),
            *self + Position::new(1, 0),
            *self + Position::new(1, 1),
        ]
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

type Elf = Position;

#[derive(Copy, Clone, Debug)]
enum Direction {
    South,
    North,
    East,
    West,
}

impl Direction {
    fn positions(&self) -> [Position; 3] {
        match self {
            Direction::North => [
                Position::new(-1, 1),
                Position::new(0, 1),
                Position::new(1, 1),
            ],
            Direction::South => [
                Position::new(-1, -1),
                Position::new(0, -1),
                Position::new(1, -1),
            ],
            Direction::East => [
                Position::new(1, -1),
                Position::new(1, 0),
                Position::new(1, 1),
            ],
            Direction::West => [
                Position::new(-1, -1),
                Position::new(-1, 0),
                Position::new(-1, 1),
            ],
        }
    }

    fn position(&self) -> Position {
        match self {
            Direction::North => Position::new(0, 1),
            Direction::South => Position::new(0, -1),
            Direction::East => Position::new(1, 0),
            Direction::West => Position::new(-1, 0),
        }
    }
}

impl Elf {
    fn positions_in_direction(&self, direction: &Direction) -> [Position; 3] {
        direction.positions().map(|x| x + *self)
    }

    fn new_position(&self, direction: &Direction) -> Position {
        *self + direction.position()
    }
}

#[derive(Clone, Debug)]
struct Forest {
    elves: Vec<Elf>,
    set: Option<HashSet<Elf>>,
    directions: VecDeque<Direction>,
}

impl Forest {
    fn new(elves: &[Elf]) -> Self {
        Self {
            elves: elves.to_vec(),
            set: None,
            directions: VecDeque::from([
                Direction::North,
                Direction::South,
                Direction::West,
                Direction::East,
            ]),
        }
    }

    fn occupied_at(&self, p: &Position) -> bool {
        self.set.as_ref().unwrap().contains(p)
    }

    fn should_move(&self, elf: &Elf, direction: &Direction) -> bool {
        elf.positions_in_direction(direction)
            .iter()
            .all(|p| !self.occupied_at(p))
    }

    fn directions_queue(&mut self) -> VecDeque<Direction> {
        let directions = self.directions.clone();
        let direction = self.directions.pop_front().unwrap();
        self.directions.push_back(direction);
        directions
    }

    fn decide_action(&self, elf: &Elf, directions: &VecDeque<Direction>) -> Position {
        let no_neighbours = elf.neighbours().iter().all(|p| !self.occupied_at(p));
        if no_neighbours {
            return *elf;
        }

        for direction in directions {
            if self.should_move(elf, direction) {
                return elf.new_position(direction);
            }
        }
        *elf
    }

    fn tick(&mut self) -> bool {
        let directions = self.directions_queue();

        // Collect the elves into a set to speed up checks
        self.set = Some(self.elves.clone().into_iter().collect());

        // Each elf considers their next move
        let new_positions: Vec<Elf> = self
            .elves
            .iter()
            .map(|elf| self.decide_action(elf, &directions))
            .collect();

        // Count to find duplicates
        let mut count: HashMap<&Position, usize> = HashMap::new();
        for p in &new_positions {
            *count.entry(p).or_default() += 1;
        }

        let mut moved = false;
        for (position, new_position) in self.elves.iter_mut().zip(&new_positions) {
            if position != new_position && count[new_position] == 1 {
                moved = true;
                *position = *new_position;
            }
        }
        moved
    }

    fn empty_ground(&self) -> usize {
        let n = self.elves.iter().map(|p| p.y).max().unwrap();
        let s = self.elves.iter().map(|p| p.y).min().unwrap();
        let w = self.elves.iter().map(|p| p.x).min().unwrap();
        let e = self.elves.iter().map(|p| p.x).max().unwrap();

        let total_tiles = (n - s + 1) * (e - w + 1);
        total_tiles as usize - self.elves.len()
    }
}

fn part1(input: &Input) -> usize {
    let mut forest = Forest::new(&input.elves);
    for _ in 0..10 {
        forest.tick();
    }
    forest.empty_ground()
}

fn part2(input: &Input) -> usize {
    let mut round = 1;
    let mut forest = Forest::new(&input.elves);
    while forest.tick() {
        round += 1;
    }
    round
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
    const TEST_INPUT: &str = include_str!("../../inputs/test_day23.txt");

    #[test]
    fn examples() {
        let input = Input::new(TEST_INPUT);
        assert_eq!(part1(&input), 110);
        assert_eq!(part2(&input), 20);
    }

    #[test]
    fn answers() {
        let input = Input::new(INPUT);
        assert_eq!(part1(&input), 4288);
        assert_eq!(part2(&input), 940);
    }
}
