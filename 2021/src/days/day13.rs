use std::{collections::HashSet, fmt};

use serde_scan::scan;

const INPUT: &str = include_str!("../../inputs/day13.txt");

#[derive(Clone)]
struct Input {
    pub points: Points,
    pub instructions: Vec<Fold>,
}

#[derive(Clone)]
struct Points(HashSet<(usize, usize)>);

/*
 * 0,0 represents the top-left coordinate.
 * The first value, x, increases to the right.
 * The second value, y, increases downward.
*/
impl Points {
    pub fn new() -> Self {
        Self(HashSet::new())
    }

    pub fn fold(&mut self, fold: &Fold) {
        // Fold the paper along the X or Y axis
        // We need a copy of the points so we can iterate over them while mutating the original
        let points_iter = self.0.clone();
        match fold {
            Fold::X(amount) => {
                // Folding along the X axis means folding the paper left. Any points that have a higher X value are reflected.
                for &point in points_iter.iter() {
                    if point.0 > *amount {
                        self.remove(&point);
                        let new_x = 2 * amount - point.0;
                        self.insert((new_x, point.1));
                    }
                }
            }
            Fold::Y(amount) => {
                // Folding along the Y axis means folding the paper up. Any points that have a higher Y value are reflected.
                for &point in points_iter.iter() {
                    if point.1 > *amount {
                        self.remove(&point);
                        let new_y = 2 * amount - point.1;
                        self.insert((point.0, new_y));
                    }
                }
            }
        }
    }

    pub fn insert(&mut self, point: (usize, usize)) {
        self.0.insert(point);
    }

    pub fn remove(&mut self, point: &(usize, usize)) {
        self.0.remove(point);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl fmt::Display for Points {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..7 {
            for x in 0..40 {
                if self.0.contains(&(x, y)) {
                    write!(f, "X")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone)]
enum Fold {
    X(usize),
    Y(usize),
}

impl Input {
    pub fn new(input: &str) -> Input {
        let mut points = Points::new();
        let mut instructions = Vec::new();
        for line in input.lines() {
            if let Ok((x, y)) = scan!("{},{}" <- line) {
                points.insert((x, y));
            } else if let Ok((axis, value)) = scan!("fold along {}={}" <- line) {
                let fold = match axis {
                    "x" => Fold::X,
                    "y" => Fold::Y,
                    _ => unreachable!(),
                };
                instructions.push(fold(value));
            }
        }
        Input {
            points,
            instructions,
        }
    }
}

fn part1(input: &Input) -> usize {
    let mut points = input.points.clone();
    let first_fold = input.instructions.first().unwrap();
    points.fold(first_fold);

    points.len()
}

fn part2(input: &Input) {
    let mut points = input.points.clone();
    for fold in &input.instructions {
        points.fold(fold);
    }

    println!("{}", points);
}

pub fn main() {
    let input = Input::new(INPUT);
    let answer1 = part1(&input);
    println!("Part 1: {}", answer1);
    println!("Part 2");
    part2(&input);
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/test_day13.txt");

    #[test]
    pub fn examples() {
        let input = Input::new(TEST_INPUT);
        assert_eq!(part1(&input), 17);
    }

    #[test]
    pub fn answers() {
        let input = Input::new(INPUT);
        assert_eq!(part1(&input), 781);
    }
}
