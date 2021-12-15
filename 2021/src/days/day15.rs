use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

const INPUT: &str = include_str!("../../inputs/day15.txt");

#[derive(Clone, Eq, PartialEq, Debug)]
struct Point {
    distance: usize,
    position: Position,
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type Position = (usize, usize);
type Node = (usize, usize);

#[derive(Clone)]
struct Grid {
    grid: Vec<Vec<usize>>,
}

impl Grid {
    pub fn height(&self) -> usize {
        self.grid.len()
    }

    pub fn width(&self) -> usize {
        self.grid[0].len()
    }

    pub fn risk(&self, (row, col): Node) -> usize {
        let height = self.height();
        let width = self.width();

        let value = self.grid[row % height][col % width] + (row / height) + (col / width);

        // Adjust to be from 1-9
        (value - 1) % 9 + 1
    }
}

#[derive(Clone)]
struct Input {
    pub grid: Grid,
}

impl Input {
    pub fn new(input: &str) -> Input {
        let mut grid = Vec::new();
        for line in input.lines() {
            grid.push(
                line.chars()
                    .map(|char| char.to_digit(10).unwrap().try_into().unwrap())
                    .collect::<Vec<usize>>(),
            );
        }
        Input {
            grid: Grid { grid },
        }
    }
}

fn neighbouring_points(point: (usize, usize), height: usize, width: usize) -> Vec<(usize, usize)> {
    // Need signed integers to get around the bound checks cleanly
    let row = point.0;
    let col = point.1;
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

struct Dijkstra<'a> {
    grid: &'a Grid,
    end: Node,
    best_distance_to: HashMap<Node, usize>,
    queue: BinaryHeap<Point>,
    height: usize,
    width: usize,
}

impl<'a> Dijkstra<'a> {
    pub fn new(grid: &'a Grid, height: usize, width: usize) -> Self {
        let start = (0, 0);
        let end = (height - 1, width - 1);

        let mut best_distance_to = HashMap::with_capacity(height * width);
        best_distance_to.insert(start, 0);

        let mut next_best = BinaryHeap::with_capacity(height * width);
        next_best.push(Point {
            distance: 0,
            position: start,
        });

        Self {
            grid,
            end,
            best_distance_to,
            queue: next_best,
            height,
            width,
        }
    }

    pub fn cost_to_enter(&self, point: &Node) -> usize {
        self.grid.risk(*point)
    }

    pub fn current_best_distance(&mut self, point: &Node) -> usize {
        *self.best_distance_to.entry(*point).or_insert(usize::MAX)
    }

    pub fn update_best_distance(&mut self, point: Node, new_distance: usize) {
        self.best_distance_to.insert(point, new_distance);
        self.queue.push(Point {
            distance: new_distance,
            position: point,
        });
    }

    pub fn shortest_path(mut self) -> usize {
        // Dijkstra's algorithm
        while let Some(current) = self.queue.pop() {
            for neighbour in neighbouring_points(current.position, self.height, self.width) {
                let new_distance = current.distance + self.cost_to_enter(&neighbour);
                if new_distance < self.current_best_distance(&neighbour) {
                    self.update_best_distance(neighbour, new_distance);
                }
            }
        }

        *self.best_distance_to.get(&self.end).unwrap()
    }
}

fn shortest_distance_default(grid: &Grid) -> usize {
    shortest_distance(grid, grid.height(), grid.width())
}

fn shortest_distance(grid: &Grid, height: usize, width: usize) -> usize {
    let dijkstra = Dijkstra::new(grid, height, width);
    dijkstra.shortest_path()
}

fn part1(input: &Input) -> usize {
    shortest_distance_default(&input.grid)
}

fn part2(input: &Input) -> usize {
    let height = input.grid.height() * 5;
    let width = input.grid.width() * 5;
    shortest_distance(&input.grid, height, width)
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
    const TEST_INPUT: &str = include_str!("../../inputs/test_day15.txt");

    #[test]
    pub fn examples() {
        let input = Input::new(TEST_INPUT);
        assert_eq!(part1(&input), 40);
        assert_eq!(part2(&input), 315);
    }

    #[test]
    pub fn answers() {
        let input = Input::new(INPUT);
        assert_eq!(part1(&input), 811);
        assert_eq!(part2(&input), 3012);
    }
}
