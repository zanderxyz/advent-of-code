use serde_scan::scan;

const INPUT: &str = include_str!("../../inputs/day10.txt");

#[derive(Clone, Debug)]
struct Input {
    instructions: Vec<Instruction>,
}

impl Input {
    fn new(input: &str) -> Input {
        Input {
            instructions: input
                .lines()
                .map(|line| {
                    if line == "noop" {
                        Instruction::NoOp
                    } else {
                        let value = scan!("addx {}" <- line).unwrap();
                        Instruction::Add(value)
                    }
                })
                .collect(),
        }
    }
}

#[derive(Clone, Debug)]
enum Instruction {
    NoOp,
    Add(isize),
}

#[derive(Clone, Debug)]
struct Microcontroller {
    cycle: usize,
    x: isize,
}

impl Default for Microcontroller {
    fn default() -> Self {
        Self { cycle: 1, x: 1 }
    }
}

impl Microcontroller {
    fn apply(&mut self, instruction: &Instruction, mut action: impl FnMut(usize, isize)) {
        match instruction {
            Instruction::NoOp => {
                action(self.cycle, self.x);
                self.cycle += 1;
            }
            Instruction::Add(a) => {
                for _ in 0..2 {
                    action(self.cycle, self.x);
                    self.cycle += 1;
                }
                self.x += a;
            }
        }
    }
}

fn part1(input: &Input) -> isize {
    let mut micro = Microcontroller::default();
    let mut sum = 0;

    let targets: im::HashSet<usize> = im::HashSet::from_iter([20_usize, 60, 100, 140, 180, 220]);

    for instruction in &input.instructions {
        micro.apply(instruction, |cycle, x| {
            if targets.contains(&cycle) {
                let signal = x * (cycle as isize);
                sum += signal;
            }
        });
    }

    sum
}

#[derive(Debug, Clone)]
struct Crt {
    pixels: [bool; 40 * 6],
}

impl Default for Crt {
    fn default() -> Self {
        Self {
            pixels: [false; 40 * 6],
        }
    }
}

impl std::fmt::Display for Crt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..6 {
            for col in 0..40 {
                if self.pixels[row * 40 + col] {
                    write!(f, "#")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Crt {
    fn draw(&mut self, x: isize, cycle: usize) {
        let col = (cycle - 1) % 40;
        if (x - 1..=x + 1).contains(&(col as isize)) {
            self.pixels[cycle - 1] = true;
        }
    }
}

fn part2(input: &Input) {
    let mut micro = Microcontroller::default();
    let mut crt = Crt::default();

    for instruction in &input.instructions {
        micro.apply(instruction, |cycle, x| {
            crt.draw(x, cycle);
        });
    }

    println!("{}", crt);
}

pub fn main() {
    let input = Input::new(INPUT);
    let answer1 = part1(&input);
    println!("Part 1: {}", answer1);
    part2(&input);
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/test_day10.txt");

    #[test]
    fn examples() {
        let input = Input::new(TEST_INPUT);
        assert_eq!(part1(&input), 13140);
    }

    #[test]
    fn answers() {
        let input = Input::new(INPUT);
        assert_eq!(part1(&input), 13480);
    }
}
