use std::{convert::Infallible, str::FromStr};

use itertools::Itertools;
use serde_scan::scan;

const INPUT: &str = include_str!("../../inputs/day15.txt");

#[derive(Clone, Debug)]
struct Input {
    diamonds: Vec<Diamond>,
}

impl Input {
    fn new(input: &str) -> Input {
        let diamonds = input
            .lines()
            .map(|line| line.parse::<Observation>().unwrap())
            .map(|o| o.diamond())
            .collect();

        Input { diamonds }
    }
}

#[derive(Clone, Debug)]
struct Observation {
    sensor: Point,
    beacon: Point,
}

impl FromStr for Observation {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (sx, sy, bx, by) =
            scan!("Sensor at x={}, y={}: closest beacon is at x={}, y={}" <- s).unwrap();
        let sensor = Point::new(sx, sy);
        let beacon = Point::new(bx, by);
        Ok(Observation { sensor, beacon })
    }
}

impl Observation {
    fn manhattan_dist(&self) -> usize {
        self.sensor.manhattan_dist(&self.beacon)
    }

    fn diamond(&self) -> Diamond {
        Diamond {
            center: self.sensor,
            distance: self.manhattan_dist(),
        }
    }
}

#[derive(Clone, Debug)]
struct Diamond {
    center: Point,
    distance: usize,
}

impl Diamond {
    fn apex(&self) -> isize {
        self.center.y + (self.distance as isize)
    }

    fn zenith(&self) -> isize {
        self.center.y - (self.distance as isize)
    }

    fn contains_row(&self, row: isize) -> bool {
        self.apex() >= row && self.zenith() <= row
    }

    fn contains(&self, p: &Point) -> bool {
        self.center.manhattan_dist(p) <= self.distance
    }

    fn lines(&self) -> [Line; 4] {
        [
            // Upper left -> left & apex
            Line::new(1, self.apex() - self.center.x),
            // Lower right -> right & zenith
            Line::new(1, self.zenith() - self.center.x),
            // Upper right -> right & apex
            Line::new(-1, self.apex() + self.center.x),
            // Lower left -> right & zenith
            Line::new(-1, self.zenith() + self.center.x),
        ]
    }

    fn possible_uncovered_points(&self, other: &Self) -> Vec<Point> {
        // We want to find the points at which the lines of these two diamonds intersect
        // Check they are not too far away
        if self.center.manhattan_dist(&other.center) - 1 > self.distance + other.distance {
            return vec![];
        }

        let self_lines = self.lines();
        let other_lines = other.lines();

        // 8 possible points of intersection
        // The uncovered point would be one point away from an intersection
        vec![
            // Upper left & upper right -> point would be one above
            self_lines[0].intersection(&other_lines[2]).offset(0, 1),
            // Upper left & lower left -> point would be one left
            self_lines[0].intersection(&other_lines[3]).offset(-1, 0),
            // Lower right & upper right -> point would be one right
            self_lines[1].intersection(&other_lines[2]).offset(1, 0),
            // Lower right & lower left -> point would be one below
            self_lines[1].intersection(&other_lines[3]).offset(0, -1),
            // Upper right & upper left -> point would be one above
            self_lines[2].intersection(&other_lines[0]).offset(0, 1),
            // Upper right & lower right -> point would be one left
            self_lines[2].intersection(&other_lines[1]).offset(-1, 0),
            // Lower left & upper left -> point would be one right
            self_lines[3].intersection(&other_lines[0]).offset(1, 0),
            // Lower left & lower right -> point would be one below
            self_lines[3].intersection(&other_lines[1]).offset(0, -1),
        ]
    }

    fn range_in_row(&self, row: isize) -> (isize, isize) {
        let y_dist_from_center = self.center.y.abs_diff(row);
        let max_x_dist_from_center = self.distance - y_dist_from_center;
        let min_x = self.center.x - max_x_dist_from_center as isize;
        let max_x = self.center.x + max_x_dist_from_center as isize;
        (min_x, max_x)
    }
}

#[derive(Clone, Debug)]
struct Line {
    slope: isize,
    intercept: isize,
}

impl Line {
    fn new(slope: isize, intercept: isize) -> Self {
        Self { slope, intercept }
    }

    fn intersection(&self, other: &Self) -> Point {
        // Calculate the point at which two lines intersect
        let x = (other.intercept - self.intercept) / (self.slope - other.slope);
        let y = self.slope * x + self.intercept;
        Point::new(x, y)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    fn manhattan_dist(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    fn offset(&mut self, x: isize, y: isize) -> Self {
        Self::new(self.x + x, self.y + y)
    }
}

// Allows merging two sorted ranges that we KNOW overlap into a single range
fn merge(first: (isize, isize), second: (isize, isize)) -> (isize, isize) {
    if contains(first, second) {
        first
    } else if overlaps_right(first, second) {
        (first.0, second.1)
    } else {
        // This is not possible in the input we are given, since we sorted the ranges firsst
        unreachable!();
    }
}

fn contains(first: (isize, isize), second: (isize, isize)) -> bool {
    contains_num(first, second.0) && contains_num(first, second.1)
}

fn contains_num(first: (isize, isize), number: isize) -> bool {
    first.0 <= number && first.1 >= number
}

fn overlaps_right(first: (isize, isize), second: (isize, isize)) -> bool {
    contains_num(first, second.0)
}

fn part1(input: &Input, row: isize) -> isize {
    let range = input
        .diamonds
        .iter()
        // Look only at the diamonds passing through our target row
        .filter(|d| d.contains_row(row))
        // Get a range of points within that row
        .map(|d| d.range_in_row(row))
        // Sort them to ensure we can merge them
        .sorted()
        // The ranges here can be merged into a single range which contains all the points that are filled
        .reduce(merge)
        .unwrap();

    range.1 - range.0
}

fn part2(input: &Input, max_coord: isize) -> isize {
    // The missing point must be between 4 diamonds
    // Or, more precisely, it must be the central point between 2 pairs of parallel overlapping lines
    // If we compare all pairs of diamonds, and find the points where their lines intersect, then we should be able to find the missing spot
    let point = input
        .diamonds
        .iter()
        // Look at every pair of diamonds
        .tuple_combinations()
        // Get the possible points of intersection between their lines
        .flat_map(|(a, b)| a.possible_uncovered_points(b))
        // Get rid of duplicates
        .unique()
        // Ensure the points are inside the boundary
        .filter(|p| p.x >= 0 && p.x <= max_coord && p.y >= 0 && p.y <= max_coord)
        // Find the unique point that is not inside any of the other diamonds
        .find(|p| input.diamonds.iter().all(|d| !d.contains(p)))
        .unwrap();

    point.x * 4_000_000 + point.y
}

pub fn main() {
    let input = Input::new(INPUT);
    let answer1 = part1(&input, 2_000_000);
    println!("Part 1: {}", answer1);
    let answer2 = part2(&input, 4_000_000);
    println!("Part 2: {}", answer2);
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/test_day15.txt");

    #[test]
    fn examples() {
        let input = Input::new(TEST_INPUT);
        assert_eq!(part1(&input, 10), 26);
        assert_eq!(part2(&input, 20), 56_000_011);
    }

    #[test]
    fn answers() {
        let input = Input::new(INPUT);
        assert_eq!(part1(&input, 2_000_000), 6078701);
        assert_eq!(part2(&input, 4_000_000), 12567351400528);
    }
}
