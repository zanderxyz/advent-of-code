use std::{
    collections::{HashSet, VecDeque},
    fmt,
};

const INPUT: &str = include_str!("../../inputs/day11.txt");

const WIDTH: usize = 10;
const HEIGHT: usize = 10;

#[derive(Clone)]
struct Input {
    pub octopodes: Octopodes<WIDTH, HEIGHT>,
}

impl Input {
    pub fn new(input: &str) -> Input {
        let energy: [[u8; WIDTH]; HEIGHT] = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                    .collect::<Vec<u8>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<[u8; WIDTH]>>()
            .try_into()
            .unwrap();
        let octopodes = Octopodes { energy, flashes: 0 };
        Input { octopodes }
    }
}

#[derive(Clone, Debug)]
struct Octopodes<const W: usize, const H: usize> {
    energy: [[u8; W]; H],
    pub flashes: usize,
}

impl<const W: usize, const H: usize> fmt::Display for Octopodes<W, H> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..H {
            for col in 0..W {
                write!(f, "{}", self.energy[row][col])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<const W: usize, const H: usize> Octopodes<W, H> {
    pub fn tick(&mut self) -> bool {
        let mut ready_to_flash = VecDeque::new();

        for row in 0..H {
            for col in 0..W {
                self.energy[row][col] += 1;
                if self.energy[row][col] > 9 {
                    ready_to_flash.push_back((row, col));
                }
            }
        }

        // We need to do a BFS through the octopuses
        // Start with octopuses of energy 10 (already added to queue)
        let mut flashed = HashSet::new();
        while !ready_to_flash.is_empty() {
            let (row, col) = ready_to_flash.pop_front().unwrap();
            flashed.insert((row, col));
            for (r, c) in neighbouring_points(row, col, H, W) {
                if !flashed.contains(&(r, c)) && self.increase_energy_and_ready(r, c) {
                    ready_to_flash.push_back((r, c));
                }
            }
        }

        let number_flashes = flashed.len();
        self.flashes += number_flashes;

        // Set the energy of all flashes octopuses back to zero
        for (row, col) in flashed.into_iter() {
            self.energy[row][col] = 0;
        }

        // Have all octopuses flashed?
        number_flashes == W * H
    }

    /// Increase the energy of an octopus
    /// Returns true if this increase causes the octopus to flash
    fn increase_energy_and_ready(&mut self, row: usize, col: usize) -> bool {
        if self.energy[row][col] > 9 {
            // If it has already flashed, no need to increase energy further
            false
        } else {
            // Otherwise increase energy and check if it is ready to flash
            self.energy[row][col] += 1;
            self.energy[row][col] == 10
        }
    }
}

fn neighbouring_points(row: usize, col: usize, height: usize, width: usize) -> Vec<(usize, usize)> {
    // Need signed integers to get around the bound checks cleanly
    let x = row as isize;
    let y = col as isize;
    [
        (x - 1, y),
        (x + 1, y),
        (x, y - 1),
        (x, y + 1),
        (x - 1, y - 1),
        (x - 1, y + 1),
        (x + 1, y - 1),
        (x + 1, y + 1),
    ]
    .into_iter()
    // Filter out anything < 0 before converting back to unsigned integers
    .filter(|&(a, b)| a >= 0 && b >= 0)
    .map(|(a, b)| (a as usize, b as usize))
    // Then filter out anything too large
    .filter(|&(a, b)| a < height && b < width)
    .collect()
}

fn part1(input: &Input) -> Octopodes<WIDTH, HEIGHT> {
    let mut octopodes = input.octopodes.clone();
    for _ in 0..100 {
        octopodes.tick();
    }
    octopodes
}

fn part2(mut octopodes: Octopodes<WIDTH, HEIGHT>) -> usize {
    let mut steps = 100;
    while !octopodes.tick() {
        steps += 1;
    }
    steps + 1
}

pub fn main() {
    let input = Input::new(INPUT);
    let octopodes = part1(&input);
    println!("Part 1: {}", octopodes.flashes);
    let answer2 = part2(octopodes);
    println!("Part 2: {}", answer2);
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/test_day11.txt");

    #[test]
    pub fn examples() {
        let input = Input::new(TEST_INPUT);
        let octopodes = part1(&input);
        assert_eq!(octopodes.flashes, 1656);
        assert_eq!(part2(octopodes), 195);
    }

    #[test]
    pub fn answers() {
        let input = Input::new(INPUT);
        let octopodes = part1(&input);
        assert_eq!(octopodes.flashes, 1627);
        assert_eq!(part2(octopodes), 329);
    }
}
