use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::{max, min};
use std::collections::HashSet;

const INPUT: &str = include_str!("../../inputs/day05.txt");

#[derive(Clone)]
struct Input {
    pub lines: Vec<Line>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Clone)]
enum Line {
    // Row : Col From : Col To
    Horizontal(isize, isize, isize),
    // Col : Row From : Row To
    Vertical(isize, isize, isize),
    // From : To
    Diagonal((isize, isize), (isize, isize)),
}

impl Line {
    pub fn points_orthogonal(&self) -> HashSet<Point> {
        match *self {
            Line::Horizontal(y, from_col, to_col) => {
                (from_col..=to_col).map(|x| Point { x, y }).collect()
            }
            Line::Vertical(x, from_row, to_row) => {
                (from_row..=to_row).map(|y| Point { x, y }).collect()
            }
            Line::Diagonal(_, _) => HashSet::new(),
        }
    }

    pub fn points_all(&self) -> HashSet<Point> {
        match *self {
            Line::Horizontal(_, _, _) => self.points_orthogonal(),
            Line::Vertical(_, _, _) => self.points_orthogonal(),
            Line::Diagonal((from_x, from_y), (to_x, to_y)) => {
                // Count the number of points on the diagonal and then step down it
                let number_of_points = max(from_y, to_y) - min(from_y, to_y);
                let x_step = (to_x - from_x) / number_of_points;
                let y_step = (to_y - from_y) / number_of_points;
                (0..=number_of_points)
                    .map(|n| Point {
                        x: from_x + x_step * n,
                        y: from_y + y_step * n,
                    })
                    .collect()
            }
        }
    }
}

impl Input {
    pub fn new(input: &str) -> Input {
        let lines = input
            .lines()
            .map(|line| {
                let (from_x, from_y, to_x, to_y) = get_coords_from_string(line);
                if from_x == to_x {
                    Line::Vertical(from_x, min(from_y, to_y), max(from_y, to_y))
                } else if from_y == to_y {
                    Line::Horizontal(from_y, min(from_x, to_x), max(from_x, to_x))
                } else if (from_x - to_x).abs() == (from_y - to_y).abs() {
                    // This is a 45 degree diagonal
                    Line::Diagonal((from_x, from_y), (to_x, to_y))
                } else {
                    panic!("Unexpected non-45 degree diagonal")
                }
            })
            .collect();

        Input { lines }
    }
}

fn get_coords_from_string(line: &str) -> (isize, isize, isize, isize) {
    lazy_static! {
        static ref RE: Regex = Regex::new("(\\d+),(\\d+) -> (\\d+),(\\d+)").unwrap();
    }

    let captures = RE.captures(line).unwrap();
    // Capture 0 is the whole string
    let from_x = captures.get(1).unwrap().as_str().parse::<isize>().unwrap();
    let from_y = captures.get(2).unwrap().as_str().parse::<isize>().unwrap();
    let to_x = captures.get(3).unwrap().as_str().parse::<isize>().unwrap();
    let to_y = captures.get(4).unwrap().as_str().parse::<isize>().unwrap();

    (from_x, from_y, to_x, to_y)
}

fn count_intersections(input: &Input, get_points_for_line: fn(&Line) -> HashSet<Point>) -> usize {
    let mut visited = HashSet::new();
    let mut intersections = HashSet::new();
    for line in &input.lines {
        for point in get_points_for_line(line) {
            if visited.contains(&point) && !intersections.contains(&point) {
                intersections.insert(point);
            } else {
                visited.insert(point);
            }
        }
    }
    intersections.len()
}

fn part1(input: &Input) -> usize {
    count_intersections(input, Line::points_orthogonal)
}

fn part2(input: &Input) -> usize {
    count_intersections(input, Line::points_all)
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
    const TEST_INPUT: &str = include_str!("../../inputs/test_day05.txt");

    #[test]
    pub fn examples() {
        let input = Input::new(TEST_INPUT);
        assert_eq!(part1(&input), 5);
        assert_eq!(part2(&input), 12);
    }

    #[test]
    pub fn answers() {
        let input = Input::new(INPUT);
        assert_eq!(part1(&input), 6189);
        assert_eq!(part2(&input), 19164);
    }
}
