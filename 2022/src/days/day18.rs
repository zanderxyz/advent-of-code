use std::collections::{HashMap, HashSet, VecDeque};

use serde_scan::scan;

const INPUT: &str = include_str!("../../inputs/day18.txt");

#[derive(Clone, Debug)]
struct Input {
    droplet: Droplet,
}

impl Input {
    fn new(input: &str) -> Input {
        let cubes = input
            .lines()
            .map(|line| {
                let (x, y, z) = scan!("{},{},{}" <- line).unwrap();
                (x, y, z)
            })
            .collect();

        let droplet = Droplet { cubes };

        Input { droplet }
    }
}

#[derive(Clone, Debug)]
struct Droplet {
    cubes: HashSet<Coord>,
}

impl Droplet {
    fn count_uncovered_faces(&self, coord: Coord) -> usize {
        neighbouring_points(coord)
            .filter(|c| !self.cubes.contains(c))
            .count()
    }

    fn count_all_uncovered_faces(&self) -> usize {
        self.cubes
            .iter()
            .map(|c| self.count_uncovered_faces(*c))
            .sum()
    }

    fn count_external_faces(&self) -> usize {
        let min = self.cubes.iter().min().unwrap();
        let min_x = self.cubes.iter().map(|x| x.0).min().unwrap();
        let min_y = self.cubes.iter().map(|x| x.1).min().unwrap();
        let min_z = self.cubes.iter().map(|x| x.2).min().unwrap();
        let max_x = self.cubes.iter().map(|x| x.0).max().unwrap();
        let max_y = self.cubes.iter().map(|x| x.1).max().unwrap();
        let max_z = self.cubes.iter().map(|x| x.2).max().unwrap();

        // Build a map to track whether points are inside or outside
        let mut air: HashMap<Coord, bool> = HashMap::new();
        for x in (min_x - 1)..=(max_x + 1) {
            for y in (min_y - 1)..=(max_y + 1) {
                for z in (min_z - 1)..=(max_z + 1) {
                    if !self.cubes.contains(&(x, y, z)) {
                        // Start by assuming every point is inside
                        air.insert((x, y, z), false);
                    }
                }
            }
        }

        // BFS starting outside the cube
        let mut visited = HashSet::new();
        let mut queue: VecDeque<Coord> = VecDeque::new();
        queue.push_back(*min);
        while !queue.is_empty() {
            let coord = queue.pop_front().unwrap();

            // We managed to reach this point, so it is not part of the inside of the droplet
            air.insert(coord, true);

            // We can reach any neighbours that we haven't already visited
            let neighbours = neighbouring_points(coord).filter(|n| air.contains_key(n) && !air[n]);
            for n in neighbours {
                if !visited.contains(&n) {
                    queue.push_back(n);
                    visited.insert(n);
                }
            }
        }

        // Build a new inside droplet, and calculate the difference in uncovered faces
        let inside_cubes: HashSet<Coord> = air.keys().cloned().filter(|c| !air[c]).collect();
        let inner_droplet = Droplet {
            cubes: inside_cubes,
        };

        self.count_all_uncovered_faces() - inner_droplet.count_all_uncovered_faces()
    }
}

type Coord = (isize, isize, isize);

fn neighbouring_points(coord: Coord) -> impl Iterator<Item = Coord> {
    // Need signed integers to get around the bound checks cleanly
    let (x, y, z) = coord;
    [
        (x + 1, y, z),
        (x - 1, y, z),
        (x, y + 1, z),
        (x, y - 1, z),
        (x, y, z + 1),
        (x, y, z - 1),
    ]
    .into_iter()
}

fn part1(input: &Input) -> usize {
    input.droplet.count_all_uncovered_faces()
}

fn part2(input: &Input) -> usize {
    input.droplet.count_external_faces()
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
    const TEST_INPUT: &str = include_str!("../../inputs/test_day18.txt");

    #[test]
    fn examples() {
        let input = Input::new(TEST_INPUT);
        assert_eq!(part1(&input), 64);
        assert_eq!(part2(&input), 58);
    }

    #[test]
    fn answers() {
        let input = Input::new(INPUT);
        assert_eq!(part1(&input), 4504);
        assert_eq!(part2(&input), 2556);
    }
}
