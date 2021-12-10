use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day09.txt");

#[derive(Clone)]
struct Input {
    pub grid: Grid,
}

#[derive(Clone, Debug)]
struct Grid {
    pub height: usize,
    pub width: usize,
    pub values: Vec<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    row: usize,
    col: usize,
    value: usize,
}

impl Grid {
    fn point(&self, row: usize, col: usize) -> Point {
        let value = self.values[row * self.width + col];
        Point { row, col, value }
    }

    fn neighbours(&self, p: &Point) -> Vec<Point> {
        neighbouring_points(p.row, p.col, self.height, self.width)
            .iter()
            .map(|&(r, c)| self.point(r, c))
            .collect()
    }

    fn iter(&self) -> GridIter {
        self.into_iter()
    }

    fn basin_size(&self, p: &Point) -> usize {
        let mut visited: HashSet<Point> = HashSet::new();
        let mut queue: VecDeque<Point> = VecDeque::from([p.clone()]);

        // BFS through the points
        while !queue.is_empty() {
            let point = queue.pop_front().unwrap();
            visited.insert(point.clone());
            for neighbour in self.neighbours(&point) {
                // Stop at points of height 9, and skip if we've already seen them
                if neighbour.value < 9 && !visited.contains(&neighbour) {
                    queue.push_back(neighbour);
                }
            }
        }

        visited.len()
    }
}

fn neighbouring_points(row: usize, col: usize, height: usize, width: usize) -> Vec<(usize, usize)> {
    // Need signed integers to get around the bound checks cleanly
    let x = row as isize;
    let y = col as isize;
    [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
        .into_iter()
        // Filter out anything < 0 before converting back to unsigned integers
        .filter(|&(a, b)| a >= 0 && b >= 0)
        .map(|(a, b)| (a as usize, b as usize))
        // Then filter out anything too large
        .filter(|&(a, b)| a < height && b < width)
        .collect()
}

impl<'a> IntoIterator for &'a Grid {
    type Item = Point;

    type IntoIter = GridIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        GridIter {
            grid: self,
            index: 0,
        }
    }
}

struct GridIter<'a> {
    grid: &'a Grid,
    index: usize,
}

impl<'a> Iterator for GridIter<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.grid.height * self.grid.width {
            let i = self.index;
            self.index += 1;

            let col = i % self.grid.width;
            let row = (i - col) / self.grid.width;

            Some(self.grid.point(row, col))
        } else {
            None
        }
    }
}

impl Input {
    pub fn new(input: &str) -> Input {
        let mut width: usize = 0;
        let mut height: usize = 0;
        let iter = input.lines();
        let values: Vec<usize> = iter
            .flat_map(|line| {
                if width == 0 {
                    width = line.len();
                }
                height += 1;
                line.chars()
                    .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                    .collect::<Vec<usize>>()
            })
            .collect();

        let grid = Grid {
            height,
            width,
            values,
        };

        Input { grid }
    }
}

fn part1(input: &Input) -> (usize, Vec<Point>) {
    let low_points: Vec<Point> = input
        .grid
        .iter()
        .filter(|p| input.grid.neighbours(p).iter().all(|n| n.value > p.value))
        .collect();

    let score = low_points.iter().map(|p| p.value + 1).sum();

    (score, low_points)
}

fn part2(input: &Input, low_points: &[Point]) -> usize {
    low_points
        .iter()
        .map(|low_point| input.grid.basin_size(low_point))
        // Take the three largest basins, by inverting and taking the three smallest, then inverting again
        .map(|size| -(size as isize))
        .k_smallest(3)
        .map(|size| (-size) as usize)
        .product()
}

pub fn main() {
    let input = Input::new(INPUT);
    let (answer1, low_points) = part1(&input);
    println!("Part 1: {}", answer1);
    let answer2 = part2(&input, &low_points);
    println!("Part 2: {}", answer2);
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/test_day09.txt");

    #[test]
    pub fn examples() {
        let input = Input::new(TEST_INPUT);
        let (answer1, low_points) = part1(&input);
        assert_eq!(answer1, 15);
        assert_eq!(part2(&input, &low_points), 1134);
    }

    #[test]
    pub fn answers() {
        let input = Input::new(INPUT);
        let (answer1, low_points) = part1(&input);
        assert_eq!(answer1, 456);
        assert_eq!(part2(&input, &low_points), 1047744);
    }
}
