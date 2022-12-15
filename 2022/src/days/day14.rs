use std::{
    cmp::{max, min},
    collections::HashSet,
};

use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day14.txt");

#[derive(Clone, Debug)]
struct Input {
    lines: Vec<Line>,
}

impl Input {
    fn new(input: &str) -> Input {
        let lines = input
            .lines()
            .map(|line| parser::parse_line(line).unwrap().1)
            .collect();

        Input { lines }
    }
}

// Separate module to avoid polluting the namespace with nom functions
mod parser {
    use super::*;
    use nom::{
        bytes::complete::tag,
        character::complete::u16,
        combinator::{all_consuming, map},
        multi::separated_list0,
        sequence::separated_pair,
        IResult,
    };

    pub(super) fn parse_line(i: &str) -> IResult<&str, Line> {
        all_consuming(map(separated_list0(tag(" -> "), parse_coords), Line::new))(i)
    }

    fn parse_coords(i: &str) -> IResult<&str, Point> {
        map(separated_pair(u16, tag(","), u16), Point::new)(i)
    }
}

#[derive(Debug, Clone)]
struct Line {
    points: Vec<Point>,
}

impl Line {
    fn new(points: Vec<Point>) -> Self {
        Self { points }
    }

    fn to_points(&self) -> HashSet<Point> {
        let mut points: HashSet<Point> = HashSet::new();

        for (point1, point2) in self.points.iter().tuple_windows() {
            let min_x = min(point1.x, point2.x);
            let max_x = max(point1.x, point2.x);

            let min_y = min(point1.y, point2.y);
            let max_y = max(point1.y, point2.y);

            let points_in_line = (min_x..=max_x)
                .cartesian_product(min_y..=max_y)
                .map(Point::new);

            for p in points_in_line {
                points.insert(p);
            }
        }

        points
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(c: (impl Into<usize>, impl Into<usize>)) -> Self {
        Self {
            x: c.0.into(),
            y: c.1.into(),
        }
    }

    fn below(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn below_left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y + 1,
        }
    }

    fn below_right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y + 1,
        }
    }
}

#[derive(Debug, Clone)]
struct Cave {
    // We track which points in the cave are filled with an object
    data: HashSet<Point>,
    // Max depth of the cave - anything below this is the abyss
    depth: usize,
    // Does the cave end in an abyss, or floor?
    mode: Mode,
}

impl Cave {
    fn new(lines: &[Line], mode: Mode) -> Self {
        let data: HashSet<Point> = lines.iter().flat_map(|line| line.to_points()).collect();
        let depth = data.iter().map(|p| p.y).max().unwrap();

        Self { data, depth, mode }
    }

    fn is_filled(&self, p: &Point) -> bool {
        if p.y >= self.depth + 2 {
            true
        } else {
            self.data.contains(p)
        }
    }

    fn is_abyss(&self, p: &Point) -> bool {
        self.mode == Mode::Abyss && p.y > self.depth
    }

    // Drop in sand, and returns true if the sand stays in the cave
    fn drop_sand(&mut self) -> bool {
        let initial = Point::new((500_usize, 0_usize));
        let mut p = initial;

        // Drop the sand until it cannot fall further down
        loop {
            match self.find_next_state(&p) {
                SandState::Abyss => {
                    return false;
                }
                SandState::Rest(new) => {
                    // Insert this into the grid, so we know this spot is now blocked
                    self.data.insert(new);

                    // If the sand comes to rest at the initial state, we have blocked the cave
                    if new == initial {
                        return false;
                    }

                    // The sand has come to rest elsewhere
                    return true;
                }
                SandState::Falling(new) => {
                    p = new;
                }
            }
        }
    }

    fn find_next_state(&self, p: &Point) -> SandState {
        if self.is_abyss(p) {
            return SandState::Abyss;
        }

        let below = self.is_filled(&p.below());
        let left = self.is_filled(&p.below_left());
        let right = self.is_filled(&p.below_right());

        match (below, left, right) {
            (true, true, true) => SandState::Rest(*p),
            (true, true, false) => SandState::Falling(p.below_right()),
            (true, false, _) => SandState::Falling(p.below_left()),
            (false, _, _) => SandState::Falling(p.below()),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum Mode {
    Abyss,
    Floor,
}

#[derive(Clone, Debug)]
enum SandState {
    Abyss,
    Rest(Point),
    Falling(Point),
}

fn part1(input: &Input) -> usize {
    let mut grid = Cave::new(&input.lines, Mode::Abyss);
    let mut sand_dropped = 0;
    while grid.drop_sand() {
        sand_dropped += 1;
    }
    sand_dropped
}

// Given we know the shape that we will end up with here (a diagonal tree), it is possible to do this without simulating every piece
fn part2(input: &Input) -> usize {
    let mut grid = Cave::new(&input.lines, Mode::Floor);
    let mut sand_dropped = 0;
    while grid.drop_sand() {
        sand_dropped += 1;
    }
    // Increment by one, because the next piece of sand cannot be dropped
    // It's after that piece is blocked that we have come to rest
    sand_dropped + 1
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
    const TEST_INPUT: &str = include_str!("../../inputs/test_day14.txt");

    #[test]
    fn examples() {
        let input = Input::new(TEST_INPUT);
        assert_eq!(part1(&input), 24);
        assert_eq!(part2(&input), 93);
    }

    #[test]
    fn answers() {
        let input = Input::new(INPUT);
        assert_eq!(part1(&input), 808);
        assert_eq!(part2(&input), 26625);
    }
}
