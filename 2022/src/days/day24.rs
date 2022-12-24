use std::{
    collections::HashSet,
    ops::Add,
};

const INPUT: &str = include_str!("../../inputs/day24.txt");

#[derive(Clone, Debug)]
struct Input {
    grid: Grid,
}

impl Input {
    fn new(input: &str) -> Input {
        let height = input.lines().count();
        let width = input.lines().next().unwrap().len();
        let blizzards = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| (c, (x, y))))
            .filter(|(c, _)| *c != '#' && *c != '.')
            .map(|(c, (x, y))| {
                let direction = match c {
                    '>' => Direction::Right,
                    '<' => Direction::Left,
                    '^' => Direction::Up,
                    'v' => Direction::Down,
                    _ => unreachable!(),
                };
                Blizzard {
                    position: Point::new(x, y),
                    direction,
                }
            })
            .collect();

        Input {
            grid: Grid {
                width,
                height,
                blizzards,
            },
        }
    }
}

#[derive(Clone, Debug)]
struct Grid {
    width: usize,
    height: usize,
    blizzards: Vec<Blizzard>,
}

impl Grid {
    fn neighbours(&self, p: &Point) -> impl Iterator<Item = Point> + '_ {
        neighbouring_points(p.x, p.y)
            .map(|(x, y)| Point::new(x, y))
            .filter(|p| self.is_in_grid(p))
    }

    fn update_blizzards(&mut self) {
        for blizzard in self.blizzards.iter_mut() {
            move_point(
                &mut blizzard.position,
                &blizzard.direction,
                self.width,
                self.height,
            );
        }
    }

    fn is_in_grid(&self, p: &Point) -> bool {
        ((p.x > 0 && p.y > 0) && (p.x < self.width - 1 && p.y < self.height - 1)) 
            // Ending point
            || (p.x == self.width - 2 && p.y == self.height - 1)
            // Starting point
            || (p.x == 1 && p.y == 0)
    }

    fn update_player(
        &self,
        positions: HashSet<Point>,
        hurricanes: HashSet<Point>,
    ) -> HashSet<Point> {
        let mut new = HashSet::new();

        for p in positions {
            // We can wait here
            if !hurricanes.contains(&p) {
                new.insert(p);
            }

            // We can move to here
            for n in self.neighbours(&p) {
                if !hurricanes.contains(&n) {
                    new.insert(n);
                }
            }
        }

        new
    }
}

fn move_point(p: &mut Point, d: &Direction, width: usize, height: usize) {
    match d {
        Direction::Up => {
            if p.y == 1 {
                p.y = height - 2;
            } else {
                p.y -= 1;
            }
        }
        Direction::Down => {
            if p.y == height - 2 {
                p.y = 1;
            } else {
                p.y += 1;
            }
        }
        Direction::Left => {
            if p.x == 1 {
                p.x = width - 2;
            } else {
                p.x -= 1;
            }
        }
        Direction::Right => {
            if p.x == width - 2 {
                p.x = 1;
            } else {
                p.x += 1;
            }
        }
    }
}

fn neighbouring_points(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    // Need signed integers for the bounds checks
    let x = x as isize;
    let y = y as isize;
    [(x - 1, y), (x + 1, y), (x, y), (x, y - 1), (x, y + 1)]
        .into_iter()
        // Filter out anything < 0 before converting back to unsigned integers
        .filter(|&(a, b)| a >= 0 && b >= 0)
        .map(|(a, b)| (a as usize, b as usize))
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Blizzard {
    position: Point,
    direction: Direction,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Explorer {
    grid: Grid,
}

impl Explorer {
    fn new(grid: &Grid) -> Self {
        Self { grid: grid.clone() }
    }

    fn fastest_snow_walk(&mut self, start: Point, end: Point) -> usize {
        let mut minutes = 0;

        let mut positions = HashSet::new();
        positions.insert(start);

        while !positions.contains(&end) {
            self.grid.update_blizzards();
            let blizzards = self.grid.blizzards.iter().map(|b| b.position).collect();
            positions = self.grid.update_player(positions, blizzards);
            minutes += 1;
        }

        minutes
    }
}

fn part1(input: &Input) -> usize {
    let starting_point = Point::new(1, 0);
    let ending_point = Point::new(input.grid.width - 2, input.grid.height - 1);
    let mut explorer = Explorer::new(&input.grid);
    explorer.fastest_snow_walk(starting_point, ending_point)
}

fn part2(input: &Input) -> usize {
    let starting_point = Point::new(1, 0);
    let ending_point = Point::new(input.grid.width - 2, input.grid.height - 1);
    let mut explorer = Explorer::new(&input.grid);
    let first = explorer.fastest_snow_walk(starting_point, ending_point);
    let second = explorer.fastest_snow_walk(ending_point, starting_point);
    let third = explorer.fastest_snow_walk(starting_point, ending_point);
    first + second + third
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
    const TEST_INPUT: &str = include_str!("../../inputs/test_day24.txt");

    #[test]
    fn examples() {
        let input = Input::new(TEST_INPUT);
        assert_eq!(part1(&input), 18);
        assert_eq!(part2(&input), 54);
    }

    #[test]
    fn answers() {
        let input = Input::new(INPUT);
        assert_eq!(part1(&input), 271);
        assert_eq!(part2(&input), 813);
    }
}
