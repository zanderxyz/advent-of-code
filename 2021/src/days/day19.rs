use std::collections::{HashMap, HashSet};

use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day19.txt");

#[derive(Clone)]
struct Input<const D: usize> {
    pub scanners: Vec<Scanner<D>>,
}

#[derive(Clone, Debug)]
struct Scanner<const D: usize> {
    pub points: Vec<Point<D>>,
    coords: Option<Point<D>>,
    distances: HashMap<Point<D>, HashSet<Point<D>>>,
}

impl<const D: usize> Scanner<D> {
    pub fn new(points: Vec<Point<D>>) -> Self {
        Self::new_with_coords(points, None)
    }

    pub fn new_with_coords(points: Vec<Point<D>>, coords: Option<Point<D>>) -> Self {
        let distances = points.iter().fold(HashMap::new(), |mut acc, point| {
            // For each pair of points, record the absolute sorted distance between them as a vector
            // This allows us to identify if an overlap exists without needing to check every single rotation
            let set = points
                .iter()
                .map(|other| point.abs_distance(*other))
                .filter(|&p| p != [0; D])
                .collect();

            acc.insert(*point, set);
            acc
        });

        Self {
            points,
            coords,
            distances,
        }
    }

    fn place(&mut self, coords: Point<D>) {
        self.coords = Some(coords);
    }

    fn rotate_and_move(&self, rotation: &Rotation<D>, offset: &Point<D>) -> Self {
        let points: Vec<_> = self
            .points
            .iter()
            .map(|r| r.rotate(rotation).offset(offset))
            .collect();

        Self::new_with_coords(points, Some(*offset))
    }

    // Calculate if two scanners overlap, and return the required rotation and offset to move the second scanner in line with the first
    fn find_match_with(&self, other: &Self) -> Option<Scanner<D>> {
        for &beacon in &self.points {
            let abs_distances = self.distances.get(&beacon).unwrap();
            for &other_beacon in &other.points {
                let other_distances = other.distances.get(&other_beacon).unwrap();

                if abs_distances.intersection(other_distances).count() >= REQUIRED_OVERLAPS - 1 {
                    // There is sufficient overlap, we now need to find the right rotation
                    let self_dists: HashSet<_> =
                        self.points.iter().map(|p| p.distance(beacon)).collect();

                    let matching_rotation = Rotation::all()
                        .find_map(|rotation| {
                            // Rotate the distances between points using the rotation
                            let points: HashSet<_> = other
                                .points
                                .iter()
                                .map(|p| p.distance(other_beacon).rotate(&rotation))
                                .collect();

                            // Check if there are sufficient intersections
                            if self_dists.intersection(&points).count() >= REQUIRED_OVERLAPS - 1 {
                                Some(rotation)
                            } else {
                                None
                            }
                        })
                        .expect("No matching rotation found, this should be impossible");

                    let final_scanner = other.rotate_and_move(
                        &matching_rotation,
                        &beacon.distance(other_beacon.rotate(&matching_rotation)),
                    );
                    return Some(final_scanner);
                }
            }
        }
        None
    }
}

type Point<const D: usize> = [isize; D];

trait PointHelpers<const D: usize> {
    fn distance(&self, other: Self) -> Self;
    fn abs_distance(self, other: Self) -> Self;
    fn rotate(self, rotation: &Rotation<D>) -> Self;
    fn offset(self, offset: &Self) -> Self;
}

impl<const D: usize> PointHelpers<D> for Point<D> {
    // Find the offset between two points
    fn distance(&self, other: Self) -> Self {
        let mut offsets = [0; D];
        for i in 0..D {
            offsets[i] = self[i] - other[i];
        }
        offsets
    }

    fn abs_distance(self, other: Self) -> Self {
        let mut offsets = [0; D];
        for i in 0..D {
            offsets[i] = (self[i] - other[i]).abs();
        }
        offsets.sort_unstable();
        offsets
    }

    fn rotate(self, rotation: &Rotation<D>) -> Self {
        let mut new = [0; D];
        for (i, row) in rotation.iter().enumerate() {
            for (j, elem) in self.iter().enumerate() {
                new[i] += row[j] * elem;
            }
        }
        new
    }

    fn offset(self, other: &Self) -> Self {
        let mut new = [0; D];
        for i in 0..D {
            new[i] = self[i] + other[i];
        }
        new
    }
}

// A rotation matrix in D dimensions
#[derive(Clone, Debug)]
struct Rotation<const D: usize> {
    matrix: Matrix<D>,
}

type Matrix<const D: usize> = [[isize; D]; D];

impl<const D: usize> Rotation<D> {
    pub fn new(matrix: Matrix<D>) -> Self {
        Self { matrix }
    }

    pub fn iter(&self) -> impl Iterator<Item = &[isize; D]> {
        self.matrix.iter()
    }

    pub fn all() -> impl Iterator<Item = Self> {
        (0..D).permutations(D).flat_map(|permutation| {
            // Each permutation is a rotation of the D axes
            (-1isize..=1)
                .step_by(2)
                .cycle()
                .take(2 * D)
                .combinations(3)
                .unique()
                .map(move |combination| {
                    // Each combination is a vec of D -1/+1s
                    let mut matrix = [[0isize; D]; D];
                    for d in 0..D {
                        // D elements are non-zero
                        matrix[d][permutation[d]] = combination[d];
                    }
                    Rotation::new(matrix)
                })
                .collect::<Vec<Rotation<D>>>()
        })
    }
}

impl<const D: usize> Input<D> {
    pub fn new(input: &str) -> Self {
        let mut scanners = Vec::new();
        let mut points: Vec<Point<D>> = Vec::new();
        for line in input.lines() {
            if line.starts_with("---") {
                // This is a new scanner
            } else if line.is_empty() {
                // Finished a scanner
                let scanner = Scanner::new(points.clone());
                scanners.push(scanner);
                points.clear();
            } else {
                // This is a new point in the current scanner
                let coords: [isize; D] = line
                    .split(',')
                    .map(|coord| coord.parse::<isize>().unwrap())
                    .collect::<Vec<isize>>()
                    .try_into()
                    .unwrap();

                points.push(coords);
            }
        }

        // Add the final scanner
        let scanner = Scanner::new(points);
        scanners.push(scanner);

        Input { scanners }
    }
}

const REQUIRED_OVERLAPS: usize = 12;

fn part1(input: Input<3>) -> (Vec<Scanner<3>>, usize) {
    let mut located_scanners = Vec::with_capacity(input.scanners.len());

    // Start by placing the first scanner at the origin
    let mut starting = input.scanners[0].clone();
    starting.place([0; 3]);
    located_scanners.push(starting);

    let mut located_already = vec![false; input.scanners.len()];
    located_already[0] = true;

    // We can locate at least one scanner on each loop
    while located_scanners.len() < input.scanners.len() {
        for (i, scanner) in input.scanners.iter().enumerate() {
            if located_already[i] {
                continue;
            }
            for found_scanner in &located_scanners {
                if let Some(final_scanner) = found_scanner.find_match_with(scanner) {
                    located_scanners.push(final_scanner);
                    located_already[i] = true;
                    break;
                }
            }
        }
    }

    let mut beacon_positions = HashSet::new();
    for r in &located_scanners {
        beacon_positions.extend(r.points.iter().copied());
    }

    (located_scanners, beacon_positions.len())
}

fn part2(scanners: Vec<Scanner<3>>) -> isize {
    scanners
        .iter()
        .tuple_combinations()
        .map(|(left, right)| {
            // Calculate the manhattan distance for each pair of scanners
            left.coords
                .unwrap()
                .abs_distance(right.coords.unwrap())
                .iter()
                .sum()
        })
        .max()
        .unwrap()
}

pub fn main() {
    let input = Input::new(INPUT);
    let (scanners, answer1) = part1(input);
    println!("Part 1: {}", answer1);
    let answer2 = part2(scanners);
    println!("Part 2: {}", answer2);
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/test_day19.txt");

    #[test]
    pub fn rotations() {
        let all = Rotation::<3>::all();
        assert_eq!(all.count(), 48);
    }

    #[test]
    pub fn examples() {
        let input = Input::<3>::new(TEST_INPUT);
        assert_eq!(input.scanners.len(), 5);
        let (scanners, answer1) = part1(input);
        assert_eq!(answer1, 79);
        assert_eq!(part2(scanners), 3621);
    }

    #[test]
    pub fn answers() {
        let input = Input::new(INPUT);
        let (scanners, answer1) = part1(input);
        assert_eq!(answer1, 313);
        assert_eq!(part2(scanners), 10656);
    }
}
