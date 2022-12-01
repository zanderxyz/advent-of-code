use std::{collections::HashSet, fmt};

const INPUT: &str = include_str!("../../inputs/day20.txt");

#[derive(Clone)]
struct Input {
    pub algorithm: Vec<bool>,
    pub image: Image,
}

#[derive(Clone, Debug)]
struct Image {
    data: HashSet<(isize, isize)>,
    min_r: isize,
    min_c: isize,
    max_r: isize,
    max_c: isize,
    background_black: bool,
}

impl Image {
    fn is_lit(&self, position: &(isize, isize)) -> usize {
        if position.0 < self.min_r
            || position.0 >= self.max_r
            || position.1 < self.min_c
            || position.1 >= self.max_c
        {
            // This cell is out of bounds
            return if self.background_black { 0 } else { 1 };
        }
        usize::from(self.data.contains(position))
    }

    fn total_capacity(&self) -> usize {
        ((self.max_r - self.min_r) * (self.max_c - self.min_c)) as usize
    }

    fn next_capacity(&self) -> usize {
        ((self.max_r - self.min_r + 2) * (self.max_c - self.min_c + 2)) as usize
    }

    fn total_lit(&self) -> usize {
        if self.background_black {
            self.data.len()
        } else {
            // All pixels outside the range are lit
            40000 - self.total_capacity() + self.data.len()
        }
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in self.min_r..self.max_r {
            for c in self.min_c..self.max_c {
                if self.data.contains(&(r, c)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Input {
    pub fn new(input: &str) -> Input {
        let mut iter = input.lines();
        let algorithm = iter
            .next()
            .unwrap()
            .chars()
            .map(|char| match char {
                '#' => true,
                '.' => false,
                _ => unreachable!(),
            })
            .collect();

        // Skip empty line
        iter.next();

        let mut data = HashSet::new();
        let lines = iter.clone().count() as isize;
        let mut cols: isize = 0;
        for (r, line) in iter.enumerate() {
            if cols == 0 {
                cols = line.len() as isize;
            }
            for (c, char) in line.chars().enumerate() {
                if char == '#' {
                    data.insert((r as isize, c as isize));
                }
            }
        }
        let image = Image {
            data,
            min_r: 0,
            max_r: lines,
            min_c: 0,
            max_c: cols,
            background_black: true,
        };
        Input { algorithm, image }
    }
}

fn run_algorithm(image: &Image, algorithm: &[bool]) -> Image {
    let mut data = HashSet::with_capacity(image.next_capacity());
    for r in (image.min_r - 1)..(image.max_r + 1) {
        for c in (image.min_c - 1)..(image.max_c + 1) {
            let index = image.is_lit(&(r - 1, c - 1)) << 8
                | image.is_lit(&(r - 1, c)) << 7
                | image.is_lit(&(r - 1, c + 1)) << 6
                | image.is_lit(&(r, c - 1)) << 5
                | image.is_lit(&(r, c)) << 4
                | image.is_lit(&(r, c + 1)) << 3
                | image.is_lit(&(r + 1, c - 1)) << 2
                | image.is_lit(&(r + 1, c)) << 1
                | image.is_lit(&(r + 1, c + 1));

            if algorithm[index] {
                data.insert((r, c));
            }
        }
    }
    Image {
        data,
        min_r: image.min_r - 1,
        max_r: image.max_r + 1,
        min_c: image.min_c - 1,
        max_c: image.max_c + 1,
        background_black: if algorithm[0] {
            // If the first bit of the algorithm is set, then the background flips back and forth each run
            !image.background_black
        } else {
            image.background_black
        },
    }
}

fn run_times(image: &Image, algorithm: &[bool], number: usize) -> Image {
    let mut image = image.clone();
    for _ in 0..number {
        image = run_algorithm(&image, algorithm);
    }
    image
}

fn part1(input: &Input) -> usize {
    let result = run_times(&input.image, &input.algorithm, 2);
    result.total_lit()
}

fn part2(input: &Input) -> usize {
    let result = run_times(&input.image, &input.algorithm, 50);
    result.total_lit()
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
    const TEST_INPUT: &str = include_str!("../../inputs/test_day20.txt");

    #[test]
    pub fn examples() {
        let input = Input::new(TEST_INPUT);
        assert_eq!(part1(&input), 35);
        assert_eq!(part2(&input), 3351);
    }

    #[test]
    pub fn answers() {
        let input = Input::new(INPUT);
        assert_eq!(part1(&input), 5619);
        assert_eq!(part2(&input), 20122);
    }
}
