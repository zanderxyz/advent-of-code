use std::collections::HashMap;

const INPUT: &str = include_str!("../../inputs/day22.txt");

#[derive(Clone, Debug)]
struct Input {
    grid: Grid,
    instructions: Vec<Instruction>,
}

// Separate module to avoid polluting the namespace with nom functions
mod parser {
    use super::*;
    use nom::{
        character::complete::{one_of, u8},
        combinator::{all_consuming, map},
        multi::many1,
        IResult, Parser,
    };

    pub(super) fn parse_instructions(i: &str) -> IResult<&str, Vec<Instruction>> {
        all_consuming(many1(
            map(u8, Instruction::Walk).or(map(parse_turn, Instruction::Turn)),
        ))(i)
    }

    fn parse_turn(i: &str) -> IResult<&str, Turn> {
        map(one_of("RL"), char_to_turn)(i)
    }

    fn char_to_turn(i: char) -> Turn {
        match i {
            'R' => Turn::Right,
            'L' => Turn::Left,
            _ => unreachable!(),
        }
    }
}

impl Input {
    fn new(input: &str) -> Input {
        let grid = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| ((x, y), c)))
            .filter(|(_, c)| *c == '.' || *c == '#')
            .map(|((x, y), c)| {
                let cell = match c {
                    '.' => Cell::Floor,
                    '#' => Cell::Wall,
                    _ => unreachable!(),
                };
                ((x as isize, y as isize), cell)
            })
            .collect();

        let last_line = input.lines().next_back().unwrap();
        let (_, instructions) = parser::parse_instructions(last_line).unwrap();

        Input { grid, instructions }
    }
}

type Grid = HashMap<(isize, isize), Cell>;

#[derive(Clone, Debug)]
enum Cell {
    Floor,
    Wall,
}

#[derive(Clone, Debug)]
enum Instruction {
    Walk(u8),
    Turn(Turn),
}

#[derive(Clone, Debug)]
enum Turn {
    Right,
    Left,
}

#[derive(Clone, Copy, Debug)]
enum Facing {
    Right,
    Down,
    Left,
    Up,
}

trait Wrapper {
    fn next(
        &self,
        grid: &Grid,
        position: &(isize, isize),
        facing: &Facing,
    ) -> ((isize, isize), Facing);
}

#[derive(Clone, Debug)]
struct SimpleWrap {}

impl Wrapper for SimpleWrap {
    fn next(
        &self,
        grid: &Grid,
        position: &(isize, isize),
        facing: &Facing,
    ) -> ((isize, isize), Facing) {
        let p = match facing {
            Facing::Up => grid.keys().filter(|(x, _)| *x == position.0).copied().max(),
            Facing::Down => grid.keys().filter(|(x, _)| *x == position.0).copied().min(),
            Facing::Left => grid.keys().filter(|(_, y)| *y == position.1).copied().max(),
            Facing::Right => grid.keys().filter(|(_, y)| *y == position.1).copied().min(),
        }
        .unwrap();
        (p, *facing)
    }
}

#[derive(Clone, Debug)]
struct CubeWrap {}

impl Wrapper for CubeWrap {
    fn next(
        &self,
        _grid: &Grid,
        position: &(isize, isize),
        facing: &Facing,
    ) -> ((isize, isize), Facing) {
        let x_rem = position.0 % 50;
        let y_rem = position.1 % 50;

        let x_div = position.0 / 50;
        let y_div = position.1 / 50;

        match (facing, x_div, y_div) {
            (Facing::Right, 0, 3) => ((50 + y_rem, 149), Facing::Up),
            (Facing::Right, 1, 1) => ((100 + y_rem, 49), Facing::Up),
            (Facing::Right, 1, 2) => ((149, 49 - y_rem), Facing::Left),
            (Facing::Right, 2, 0) => ((99, 149 - y_rem), Facing::Left),

            (Facing::Down, 0, 3) => ((100 + x_rem, 0), Facing::Down),
            (Facing::Down, 1, 2) => ((49, 150 + x_rem), Facing::Left),
            (Facing::Down, 2, 0) => ((99, 50 + x_rem), Facing::Left),

            (Facing::Left, 0, 2) => ((50, 49 - y_rem), Facing::Right),
            (Facing::Left, 0, 3) => ((50 + y_rem, 0), Facing::Down),
            (Facing::Left, 1, 0) => ((0, 149 - y_rem), Facing::Right),
            (Facing::Left, 1, 1) => ((y_rem, 100), Facing::Down),

            (Facing::Up, 0, 2) => ((50, 50 + x_rem), Facing::Right),
            (Facing::Up, 1, 0) => ((0, 150 + x_rem), Facing::Right),
            (Facing::Up, 2, 0) => ((x_rem, 199), Facing::Up),

            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug)]
struct Walker<W: Wrapper> {
    grid: Grid,
    position: (isize, isize),
    facing: Facing,
    wrapper: W,
}

impl<W: Wrapper> Walker<W> {
    fn new(grid: &Grid, wrapper: W) -> Self {
        let x = *grid
            .keys()
            .filter(|(_, y)| *y == 0)
            .map(|(x, _)| x)
            .min()
            .unwrap();

        let position = (x, 0_isize);
        Self {
            grid: grid.clone(),
            position,
            facing: Facing::Right,
            wrapper,
        }
    }

    fn apply(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Walk(distance) => self.walk(*distance),
            Instruction::Turn(turn) => self.turn(turn),
        }
    }

    fn walk(&mut self, distance: u8) {
        if distance > 0 {
            let next = self.next_position();
            match self.grid.get(&next) {
                Some(Cell::Floor) => {
                    self.position = next;
                    self.walk(distance - 1)
                }
                Some(Cell::Wall) => {}
                None => {
                    // Need to wrap around the object
                    self.wrap(distance);
                }
            }
        }
    }

    fn wrap(&mut self, distance: u8) {
        let (next, facing) = self.wrapper.next(&self.grid, &self.position, &self.facing);
        match self.grid.get(&next) {
            Some(Cell::Floor) => {
                self.position = next;
                self.facing = facing;
                self.walk(distance - 1)
            }
            Some(Cell::Wall) => {}
            None => unreachable!(),
        }
    }

    fn next_position(&mut self) -> (isize, isize) {
        match self.facing {
            Facing::Right => (self.position.0 + 1, self.position.1),
            Facing::Down => (self.position.0, self.position.1 + 1),
            Facing::Left => (self.position.0 - 1, self.position.1),
            Facing::Up => (self.position.0, self.position.1 - 1),
        }
    }

    fn turn(&mut self, turn: &Turn) {
        let new_facing = match turn {
            Turn::Right => match self.facing {
                Facing::Right => Facing::Down,
                Facing::Down => Facing::Left,
                Facing::Left => Facing::Up,
                Facing::Up => Facing::Right,
            },
            Turn::Left => match self.facing {
                Facing::Right => Facing::Up,
                Facing::Down => Facing::Right,
                Facing::Left => Facing::Down,
                Facing::Up => Facing::Left,
            },
        };

        self.facing = new_facing;
    }

    fn password(&self) -> usize {
        1000 * (self.position.1 as usize + 1)
            + 4 * (self.position.0 as usize + 1)
            + (self.facing as usize)
    }
}

fn part1(input: &Input) -> usize {
    let mut walker = Walker::new(&input.grid, SimpleWrap {});
    for inx in &input.instructions {
        walker.apply(inx);
    }
    walker.password()
}

fn part2(input: &Input) -> usize {
    let mut walker = Walker::new(&input.grid, CubeWrap {});
    for inx in &input.instructions {
        walker.apply(inx);
    }
    walker.password()
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
    const TEST_INPUT: &str = include_str!("../../inputs/test_day22.txt");

    #[test]
    fn examples() {
        let input = Input::new(TEST_INPUT);
        assert_eq!(part1(&input), 6032);
        // Solution above is hard-coded for the specific cube format used in the puzzle!
        // assert_eq!(part2(&input), 5031);
    }

    #[test]
    fn answers() {
        let input = Input::new(INPUT);
        assert_eq!(part1(&input), 190_066);
        assert_eq!(part2(&input), 134_170);
    }
}
