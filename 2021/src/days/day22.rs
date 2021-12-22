use std::{collections::HashMap, convert::Infallible, ops::Neg, str::FromStr};

use serde_scan::scan;

use crate::helpers::increment::Increment;

const INPUT: &str = include_str!("../../inputs/day22.txt");

#[derive(Clone)]
struct Input {
    pub actions: Vec<Action>,
}

#[derive(Clone, Debug)]
struct Action {
    on_off: OnOff,
    x: (isize, isize),
    y: (isize, isize),
    z: (isize, isize),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Region {
    x: (isize, isize),
    y: (isize, isize),
    z: (isize, isize),
}

impl Region {
    fn new(action: &Action) -> Self {
        Self {
            x: action.x,
            y: action.y,
            z: action.z,
        }
    }

    // This is the region created by overlapping this region with an action
    // If the region and action do not overlap, then no region is returned
    fn overlapping_region(&self, action: &Action) -> Option<Region> {
        let region = Self {
            x: (self.x.0.max(action.x.0), self.x.1.min(action.x.1)),
            y: (self.y.0.max(action.y.0), self.y.1.min(action.y.1)),
            z: (self.z.0.max(action.z.0), self.z.1.min(action.z.1)),
        };

        if region.is_valid() {
            Some(region)
        } else {
            None
        }
    }

    fn is_valid(&self) -> bool {
        self.x.0 <= self.x.1 && self.y.0 <= self.y.1 && self.z.0 <= self.z.1
    }
}

struct Cube {
    // Track the on/off state for regions
    // Note that the value is only ever -1/0/1
    regions: HashMap<Region, isize>,
    max: Option<isize>,
}

impl Cube {
    pub fn new(max: Option<isize>) -> Self {
        Self {
            regions: HashMap::new(),
            max,
        }
    }

    pub fn apply(&mut self, action: &Action) {
        if !self.is_within(action) {
            return;
        }

        let mut region_changes = HashMap::new();

        if action.on_off == OnOff::On {
            // This new region can be turned on
            region_changes.increment(Region::new(action), 1);
        }

        // Check all current regions
        for (region, &sign) in &self.regions {
            // If they overlap with this action, then they are being toggled on or off
            if let Some(sub_region) = region.overlapping_region(action) {
                // This toggles the region, by offsetting it by the current sign
                region_changes.increment(sub_region, sign.neg())
            }
        }

        // Update the main state with these changes
        for (region, sign) in region_changes {
            self.regions.increment(region, sign);
        }
    }

    fn is_within(&self, action: &Action) -> bool {
        match self.max {
            None => true,
            Some(max) => {
                action.x.0.abs() <= max
                    && action.x.1.abs() <= max
                    && action.y.0.abs() <= max
                    && action.y.1.abs() <= max
                    && action.z.0.abs() <= max
                    && action.z.1.abs() <= max
            }
        }
    }

    pub fn len(self) -> isize {
        self.regions
            .into_iter()
            .map(|(region, sign)| {
                // For every region, count the number of cells and then multiply by the sign (which is +/- 1)
                (region.x.1 - region.x.0 + 1)
                    * (region.y.1 - region.y.0 + 1)
                    * (region.z.1 - region.z.0 + 1)
                    * sign
            })
            .sum()
    }
}

#[derive(Clone, Debug, PartialEq)]
enum OnOff {
    On,
    Off,
}

impl FromStr for OnOff {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "on" => Ok(OnOff::On),
            "off" => Ok(OnOff::Off),
            _ => unreachable!(),
        }
    }
}

impl Input {
    pub fn new(input: &str) -> Input {
        let actions = input
            .lines()
            .map(|line| {
                let (on_off_str, x1, x2, y1, y2, z1, z2) =
                    scan!("{}: x={} to {} , y={} to {} , z={} to {}" <- line).unwrap();

                let on_off = OnOff::from_str(on_off_str).unwrap();
                let x = (x1, x2);
                let y = (y1, y2);
                let z = (z1, z2);
                Action { on_off, x, y, z }
            })
            .collect();
        Input { actions }
    }
}

fn part1(input: &Input) -> isize {
    let mut cube = Cube::new(Some(50));
    for action in &input.actions {
        cube.apply(action);
    }
    cube.len()
}

fn part2(input: &Input) -> isize {
    let mut cube = Cube::new(None);
    for action in &input.actions {
        cube.apply(action);
    }
    cube.len()
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
    const TEST_INPUT_2: &str = include_str!("../../inputs/test_day22_2.txt");

    #[test]
    pub fn examples() {
        let input = Input::new(TEST_INPUT);
        assert_eq!(part1(&input), 590784);

        let input2 = Input::new(TEST_INPUT_2);
        assert_eq!(part2(&input2), 2758514936282235);
    }

    #[test]
    pub fn answers() {
        let input = Input::new(INPUT);
        assert_eq!(part1(&input), 588200);
        assert_eq!(part2(&input), 1207167990362099);
    }
}
