const INPUT: &str = include_str!("../../inputs/day08.txt");

// One minimal step in each direction
const DELTAS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(Clone, Debug)]
struct Forest {
    data: Vec<Vec<Tree>>,
}

impl Forest {
    fn height(&self) -> usize {
        self.data.len()
    }

    fn width(&self) -> usize {
        self.data[0].len()
    }

    fn in_bounds(&self, i: usize, j: usize) -> bool {
        i < self.width() && j < self.height()
    }

    fn tree_at(&self, i: usize, j: usize) -> Option<Tree> {
        if self.in_bounds(i, j) {
            Some(self.data[i][j])
        } else {
            None
        }
    }

    fn trees_in_direction(
        &self,
        i: usize,
        j: usize,
        dx: isize,
        dy: isize,
    ) -> impl Iterator<Item = Tree> + '_ {
        (1..).map_while(move |step| {
            let x = i as isize + (dx * step);
            let y = j as isize + (dy * step);
            if x >= 0 && y >= 0 {
                self.tree_at(x as usize, y as usize)
            } else {
                None
            }
        })
    }

    fn visible_trees_in_direction(&self, i: usize, j: usize, dx: isize, dy: isize) -> usize {
        let trees = self.trees_in_direction(i, j, dx, dy);

        let mut total = 0;
        let treehouse = self.tree_at(i, j).unwrap();
        for height in trees {
            total += 1;
            if height >= treehouse {
                break;
            }
        }
        total
    }

    fn scenic_score(&self, i: usize, j: usize) -> usize {
        DELTAS
            .into_iter()
            .map(|(dx, dy)| self.visible_trees_in_direction(i, j, dx, dy))
            .product()
    }
}

#[derive(Clone, Debug, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Tree(u32);

#[derive(Clone, Debug)]
struct Input {
    forest: Forest,
}

impl Input {
    fn new(input: &str) -> Input {
        let data = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .map(Tree)
                    .collect()
            })
            .collect::<Vec<Vec<Tree>>>();

        let forest = Forest { data };

        Input { forest }
    }
}

fn part1(input: &Input) -> usize {
    let forest = &input.forest;

    let mut total_visible = 0;

    for j in 0..forest.width() {
        for i in 0..forest.height() {
            let coord_height = forest.tree_at(i, j).unwrap();
            // Walk from every tree to the edge of the grid to find out if they are visible
            let visible = DELTAS.iter().any(|&(dx, dy)| {
                // Walk along this direction until the edge of the grid
                forest
                    .trees_in_direction(i, j, dx, dy)
                    // Tree is visible if all trees before it are smaller
                    .all(|height| height < coord_height)
            });

            if visible {
                total_visible += 1;
            }
        }
    }

    total_visible
}

fn part2(input: &Input) -> usize {
    let forest = &input.forest;

    let mut max_score = 0;
    // Check every non-boundary location
    for j in 1..forest.width() - 1 {
        for i in 1..forest.height() - 1 {
            let score = forest.scenic_score(i, j);
            if score > max_score {
                max_score = score;
            }
        }
    }

    max_score
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
    const TEST_INPUT: &str = include_str!("../../inputs/test_day08.txt");

    #[test]
    fn examples() {
        let input = Input::new(TEST_INPUT);
        assert_eq!(part1(&input), 21);
        assert_eq!(part2(&input), 8);
    }

    #[test]
    fn answers() {
        let input = Input::new(INPUT);
        assert_eq!(part1(&input), 1801);
        assert_eq!(part2(&input), 209880);
    }
}
