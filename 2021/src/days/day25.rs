const INPUT: &str = include_str!("../../inputs/day25.txt");

type Grid = Vec<Vec<Option<Cucumber>>>;

#[derive(Clone)]
struct Input {
    pub grid: Grid,
}

#[derive(Clone, PartialEq, Eq)]
enum Cucumber {
    Right,
    Down,
}

impl Input {
    pub fn new(input: &str) -> Input {
        Input {
            grid: input
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '.' => None,
                            '>' => Some(Cucumber::Right),
                            'v' => Some(Cucumber::Down),
                            _ => {
                                println!("{}", c);
                                unreachable!();
                            }
                        })
                        .collect()
                })
                .collect(),
        }
    }
}

fn change_positions(grid: &mut Grid, width: usize, height: usize) -> bool {
    let mut something_moved = false;

    // Move right
    for row in grid.iter_mut() {
        // Check the right hand column first
        let crossed_boundary = if row[width - 1] == Some(Cucumber::Right) && row[0] == None {
            row[width - 1] = None;
            row[0] = Some(Cucumber::Right);
            something_moved = true;
            1
        } else {
            0
        };

        // If anything crossed over to the LHS, we do not need to check the first column
        let mut col = crossed_boundary;
        while col < width - crossed_boundary - 1usize {
            if row[col] == Some(Cucumber::Right) && row[col + 1] == None {
                row[col + 1] = Some(Cucumber::Right);
                row[col] = None;
                col += 1;
                something_moved = true;
            }
            col += 1;
        }
    }

    // Move south.
    for col in 0..width {
        // Check the bottom row first
        let crossed_boundary =
            if grid[height - 1][col] == Some(Cucumber::Down) && grid[0][col] == None {
                grid[height - 1][col] = None;
                grid[0][col] = Some(Cucumber::Down);
                something_moved = true;
                1
            } else {
                0
            };

        // If anything crossed over to the top row, we do not need to check the first row
        let mut row = crossed_boundary;
        while row < height - crossed_boundary - 1 {
            if grid[row][col] == Some(Cucumber::Down) && grid[row + 1][col] == None {
                grid[row + 1][col] = Some(Cucumber::Down);
                grid[row][col] = None;
                row += 1;
                something_moved = true;
            }
            row += 1;
        }
    }

    something_moved
}

fn run_until_fixed(mut grid: Grid) -> usize {
    let mut tick = 1;
    let height = grid.len();
    let width = grid[0].len();
    while change_positions(&mut grid, width, height) {
        tick += 1;
    }
    tick
}

fn part1(input: Input) -> usize {
    run_until_fixed(input.grid)
}

pub fn main() {
    let input = Input::new(INPUT);
    let answer1 = part1(input);
    println!("Part 1: {}", answer1);
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/test_day25.txt");

    #[test]
    pub fn examples() {
        let input = Input::new(TEST_INPUT);
        assert_eq!(part1(input), 58);
    }

    #[test]
    pub fn answers() {
        let input = Input::new(INPUT);
        assert_eq!(part1(input), 549);
    }
}
