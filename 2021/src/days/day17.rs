use serde_scan::scan;

const INPUT: &str = include_str!("../../inputs/day17.txt");

#[derive(Clone)]
struct Input {
    pub target: Target,
}

#[derive(Clone)]
struct Target {
    x: (isize, isize),
    y: (isize, isize),
}

struct Probe {
    position: (isize, isize),
    velocity: (isize, isize),
}

impl Probe {
    pub fn new(velocity: (isize, isize)) -> Self {
        Self {
            position: (0, 0),
            velocity,
        }
    }

    fn tick(&mut self) {
        let (x, y) = self.position;
        let (vx, vy) = self.velocity;
        self.position = (x + vx, y + vy);
        let nvx = match vx {
            x if x > 0 => x - 1,
            x if x < 0 => x + 1,
            _ => 0,
        };
        self.velocity = (nvx, vy - 1);
    }

    fn in_target(&self, target: &Target) -> bool {
        let (x, y) = self.position;
        x >= target.x.0 && x <= target.x.1 && y >= target.y.0 && y <= target.y.1
    }

    fn is_beyond_target(&self, target: &Target) -> bool {
        let (x, y) = self.position;
        y < target.y.0 || x > target.x.1
    }

    pub fn take_shot(&mut self, target: &Target) -> Result {
        let mut ticks = 1;
        self.tick();
        while !self.in_target(target) && !self.is_beyond_target(target) {
            self.tick();
            ticks += 1;
        }
        if self.in_target(target) {
            Result::Hit
        } else {
            Result::Miss(ticks)
        }
    }

    #[cfg(test)]
    fn hits_target(&mut self, target: &Target) -> bool {
        matches!(self.take_shot(target), Result::Hit)
    }
}

#[derive(Debug)]
enum Result {
    Hit,
    Miss(usize),
}

impl Input {
    pub fn new(input: &str) -> Input {
        let line = input.trim();
        let (x1, x2, y1, y2) = scan!("target area: x = {} .. {}, y = {} .. {}" <- line).unwrap();
        Input {
            target: Target {
                x: (x1, x2),
                y: (y1, y2),
            },
        }
    }
}

fn part1(input: &Input) -> isize {
    // Think only about the y coordinate
    // We must launch the probe with a positive y velocity, and it will gradually fall back down.
    // If launched at a velocity of Y, velocity progression = Y,Y-1,Y-2,Y-3,Y-4,...,Y-(t-1)
    // The y position at any time is the sum of this series = Y,2Y-1,3Y-3,4Y-6,5Y-10,..,tY-t(t-1)/2
    // Setting this to zero, the probe will ALWAYS fall back to a y position of 0 at time 2t + 1.
    // At this time, the probe has velocity of -Y-1.
    // The maximum height will be reached when the next step takes the probe to the bottom of the target box.
    // Therefore we know that the bottom of the target box, y1 = -Y-1, or Y = -y1-1
    let y: isize = -input.target.y.0 - 1;

    // Maximum height reached is y(y+1)/2
    (y * (y + 1)) / 2
}

fn part2(input: &Input) -> usize {
    // We actually have to do some work

    // Possible range for Y coordinate
    // It can't be less than the number needed to hit the lower bound of the target in one shot (as it would always miss below)
    let min_y = input.target.y.0;
    // It can't be more than the best value found above
    let max_y = -input.target.y.0 - 1;

    let mut hits = 0;
    for vy in min_y..=max_y {
        // Start x at one and keep increasing
        for vx in 1.. {
            let mut probe = Probe::new((vx, vy));
            match probe.take_shot(&input.target) {
                Result::Hit => hits += 1,
                Result::Miss(ticks) => {
                    // If we overshot in a single go, then we can break the x loop
                    if ticks == 1 {
                        break;
                    }
                }
            }
        }
    }

    hits
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
    const TEST_INPUT: &str = include_str!("../../inputs/test_day17.txt");

    #[test]
    pub fn checks() {
        let input = Input::new(TEST_INPUT);
        let target = input.target;
        assert!(Probe::new((7, 2)).hits_target(&target));
        assert!(Probe::new((6, 3)).hits_target(&target));
        assert!(Probe::new((9, 0)).hits_target(&target));
        assert!(!Probe::new((17, -4)).hits_target(&target));
    }

    #[test]
    pub fn examples() {
        let input = Input::new(TEST_INPUT);
        assert_eq!(part1(&input), 45);
        assert_eq!(part2(&input), 112);
    }

    #[test]
    pub fn answers() {
        let input = Input::new(INPUT);
        assert_eq!(part1(&input), 13203);
        assert_eq!(part2(&input), 5644);
    }
}
