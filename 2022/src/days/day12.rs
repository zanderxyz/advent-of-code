use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet, VecDeque},
};

use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day12.txt");

#[derive(Clone, Debug)]
struct Input {
    grid: Grid,
    start: (usize, usize),
    end: (usize, usize),
}

impl Input {
    fn new(input: &str) -> Input {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let points = input
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, char)| match char {
                        'S' => {
                            start = (row, col);
                            Height::Start
                        }
                        'E' => {
                            end = (row, col);
                            Height::End
                        }
                        c => Height::Other((c as u8) - b'a'),
                    })
                    .collect()
            })
            .collect();
        Input {
            grid: Grid { points },
            start,
            end,
        }
    }
}

#[derive(Clone, Debug)]
struct Grid {
    points: Vec<Vec<Height>>,
}

impl Grid {
    fn all_points(&self) -> impl Iterator<Item = Point> + '_ {
        (0..self.height())
            .cartesian_product(0..self.width())
            .map(|(row, col)| self.point(row, col))
    }

    fn height(&self) -> usize {
        self.points.len()
    }

    fn width(&self) -> usize {
        self.points[0].len()
    }

    fn point(&self, row: usize, col: usize) -> Point {
        let height = self.points[row][col].value();
        Point { row, col, height }
    }

    fn neighbours(&self, p: &Point) -> Vec<Point> {
        neighbouring_points(p.row, p.col, self.height(), self.width())
            .iter()
            .map(|&(r, c)| self.point(r, c))
            // Can only move to neighbours that are at most 1 step lower
            .filter(|dest| dest.height + 1 >= p.height)
            // Higher neighbours first
            .sorted_by_key(|p| Reverse(p.height))
            .collect()
    }
}

#[derive(Clone, Copy, Debug, Default, Hash)]
struct Point {
    row: usize,
    col: usize,
    height: u8,
}

impl Point {
    fn coords(&self) -> (usize, usize) {
        (self.row, self.col)
    }
}

#[derive(Clone, Debug)]
enum Height {
    Start,
    End,
    Other(u8),
}

impl Height {
    fn value(&self) -> u8 {
        match self {
            Height::Start => 0,
            Height::End => 26,
            Height::Other(x) => *x,
        }
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

// Calculates the shortest distance from the starting point to every other reachable point in the grid
fn shortest_distances(grid: &Grid, start: (usize, usize)) -> HashMap<(usize, usize), usize> {
    let mut best_distance_to = HashMap::with_capacity(grid.points.len());
    best_distance_to.insert(start, 0);

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let start_pt = grid.point(start.0, start.1);
    queue.push_back((start_pt, 0));
    visited.insert(start);

    // BFS through the grid will give us the shortest path to each point
    while let Some((point, distance)) = queue.pop_front() {
        let neighbours = grid.neighbours(&point);
        for neighbour in neighbours {
            let coords = neighbour.coords();
            if visited.contains(&coords) {
                continue;
            }

            let new_distance = distance + 1;
            let current_best_distance = *best_distance_to.entry(coords).or_insert(usize::MAX);

            if new_distance < current_best_distance {
                best_distance_to.insert(coords, new_distance);
                queue.push_back((neighbour, new_distance));
                visited.insert(coords);
            }
        }
    }

    best_distance_to
}

fn part1(input: &Input) -> usize {
    let shortest_distances = shortest_distances(&input.grid, input.end);
    *shortest_distances.get(&input.start).unwrap()
}

fn part2(input: &Input) -> usize {
    // Start the search from the end so we can do this in a single pass
    let shortest_distances = shortest_distances(&input.grid, input.end);

    // Check the shortest distance from each possible starting position
    *input
        .grid
        .all_points()
        .filter(|p| p.height == 0)
        .filter_map(|p| shortest_distances.get(&p.coords()))
        .min()
        .unwrap()
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
    const TEST_INPUT: &str = include_str!("../../inputs/test_day12.txt");

    #[test]
    fn examples() {
        let input = Input::new(TEST_INPUT);
        assert_eq!(part1(&input), 31);
        assert_eq!(part2(&input), 29);
    }

    #[test]
    fn answers() {
        let input = Input::new(INPUT);
        assert_eq!(part1(&input), 352);
        assert_eq!(part2(&input), 345);
    }
}
