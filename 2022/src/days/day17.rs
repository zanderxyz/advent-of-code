use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../../inputs/day17.txt");

const WIDTH: usize = 7;

#[derive(Clone, Debug)]
struct Input {
    actions: Vec<Action>,
}

impl Input {
    fn new(input: &str) -> Input {
        let actions = input
            .chars()
            .filter_map(|c| match c {
                '<' => Some(Action::Left),
                '>' => Some(Action::Right),
                _ => None,
            })
            .collect();

        Input { actions }
    }
}

type Point = (usize, usize);

#[derive(Debug, Clone, Copy)]
enum Action {
    Left,
    Right,
}

impl Action {
    fn reverse(&self) -> Self {
        match self {
            Action::Left => Action::Right,
            Action::Right => Action::Left,
        }
    }
}

#[derive(Debug, Clone)]
enum Block {
    Horizontal,
    Plus,
    Corner,
    Vertical,
    Square,
}

impl Block {
    fn iter() -> impl Iterator<Item = Block> + Clone {
        use Block::*;

        [Horizontal, Plus, Corner, Vertical, Square].into_iter()
    }

    fn pixels(&self) -> impl Iterator<Item = &Point> {
        match self {
            Block::Horizontal => [(0, 0), (1, 0), (2, 0), (3, 0)].iter(),
            Block::Plus => [(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)].iter(),
            Block::Corner => [(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)].iter(),
            Block::Vertical => [(0, 0), (0, 1), (0, 2), (0, 3)].iter(),
            Block::Square => [(0, 0), (1, 0), (0, 1), (1, 1)].iter(),
        }
    }

    fn width(&self) -> usize {
        match self {
            Block::Horizontal => 4,
            Block::Plus => 3,
            Block::Corner => 3,
            Block::Vertical => 1,
            Block::Square => 2,
        }
    }
}

#[derive(Debug, Clone)]
struct FallingBlock {
    block: Block,
    position: Point,
}

impl From<Block> for FallingBlock {
    fn from(block: Block) -> Self {
        Self {
            block,
            position: (2, 3),
        }
    }
}

impl FallingBlock {
    fn pixels(&self) -> impl Iterator<Item = Point> + '_ {
        self.block
            .pixels()
            .map(|p| (p.0 + self.position.0, p.1 + self.position.1))
    }

    fn apply(&mut self, action: &Action) -> bool {
        match action {
            Action::Left => {
                if self.position.0 == 0 {
                    false
                } else {
                    self.position.0 -= 1;
                    true
                }
            }
            Action::Right => {
                if self.position.0 + self.block.width() > WIDTH - 1 {
                    false
                } else {
                    self.position.0 += 1;
                    true
                }
            }
        }
    }

    fn fall_one(&mut self) -> bool {
        if self.position.1 > 0 {
            self.position.1 -= 1;
            true
        } else {
            false
        }
    }

    fn rise_one(&mut self) {
        self.position.1 += 1;
    }
}

#[derive(Debug, Clone, Default)]
struct Tetris<A: Iterator<Item = (usize, Action)>, B: Iterator<Item = (usize, Block)>> {
    pixels: HashSet<Point>,
    actions: A,
    blocks: B,
    height_base: usize,
    highest_pixels: [usize; WIDTH],
    highest_point: usize,
}

impl<A, B> Tetris<A, B>
where
    A: Iterator<Item = (usize, Action)>,
    B: Iterator<Item = (usize, Block)>,
{
    fn new(actions: A, blocks: B) -> Self {
        Self {
            pixels: HashSet::new(),
            actions,
            blocks,
            height_base: 0,
            highest_pixels: [0; WIDTH],
            highest_point: 0,
        }
    }

    fn final_height(&self) -> usize {
        self.height_base + self.height()
    }

    fn height(&self) -> usize {
        if self.pixels.is_empty() {
            0
        } else {
            self.highest_point + 1
        }
    }

    fn play(&mut self, number_of_blocks: usize) {
        // Action state, block state, & highest blocks in each column -> block count, height
        let mut seen: HashMap<(usize, usize, [usize; WIDTH]), (usize, usize)> = HashMap::new();

        let mut block_count = 0;
        let (mut block_index, mut block) = self.next_block();

        while block_count < number_of_blocks {
            // Action applies first
            let (action_index, action) = self.next_action();
            let moved = block.apply(&action);
            if moved && self.intersecting(&block) {
                // Hit something, reverse the movement
                block.apply(&action.reverse());
            }

            // Now the block falls
            let moved = block.fall_one();
            if moved {
                if self.intersecting(&block) {
                    // Hit something, reverse the movement
                    block.rise_one();
                } else {
                    // Moved without hitting anything, drop the next block
                    continue;
                }
            }

            // The block has landed
            block_count += 1;
            self.land_block(&block);

            let key = (action_index, block_index, self.depth_map());
            if let Some((prev_block_count, prev_height)) =
                seen.insert(key, (block_count, self.height()))
            {
                // We can fast-forward because we saw this state before
                let blocks_diff = block_count - prev_block_count;
                let height_diff = self.height() - prev_height;

                // We can leap forward by this many cycles, and increase our block count and height appropriately
                let cycles = (number_of_blocks - block_count) / blocks_diff;
                block_count += cycles * blocks_diff;
                self.height_base += cycles * height_diff;
            }

            (block_index, block) = self.next_block();
        }
    }

    fn intersecting(&self, block: &FallingBlock) -> bool {
        block.pixels().any(|p| self.pixels.contains(&p))
    }

    fn land_block(&mut self, block: &FallingBlock) {
        for p in block.pixels() {
            self.pixels.insert(p);
            self.update_highest_blocks(p);
        }
        self.highest_point = *self.highest_pixels.iter().max().unwrap();
    }

    fn update_highest_blocks(&mut self, p: Point) {
        self.highest_pixels[p.0] = self.highest_pixels[p.0].max(p.1)
    }

    fn depth_map(&self) -> [usize; WIDTH] {
        let lowest_highest_pixel = self.highest_pixels.iter().min().unwrap();
        self.highest_pixels.map(|x| x - lowest_highest_pixel)
    }

    fn next_block(&mut self) -> (usize, FallingBlock) {
        let (index, next) = self.blocks.next().unwrap();
        let mut block: FallingBlock = next.into();
        block.position.1 += self.height();
        (index, block)
    }

    fn next_action(&mut self) -> (usize, Action) {
        self.actions.next().unwrap()
    }
}

fn part1(input: &Input) -> usize {
    let actions = input.actions.iter().copied().enumerate().cycle();
    let block = Block::iter().enumerate().cycle();
    let mut tetris = Tetris::new(actions, block);
    tetris.play(2022);

    tetris.final_height()
}

fn part2(input: &Input) -> usize {
    let actions = input.actions.iter().copied().enumerate().cycle();
    let block = Block::iter().enumerate().cycle();
    let mut tetris = Tetris::new(actions, block);
    tetris.play(1_000_000_000_000);

    tetris.final_height()
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
    fn examples() {
        let input = Input::new(TEST_INPUT);
        assert_eq!(part1(&input), 3068);
        assert_eq!(part2(&input), 1514285714288);
    }

    #[test]
    fn answers() {
        let input = Input::new(INPUT);
        assert_eq!(part1(&input), 3141);
        assert_eq!(part2(&input), 1561739130391);
    }
}
