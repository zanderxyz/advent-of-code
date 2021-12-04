use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day01.txt");

#[derive(Clone)]
struct Input {
    pub numbers: Vec<usize>,
}

impl Input {
    pub fn new(input: &str) -> Input {
        Input {
            numbers: input
                .lines()
                .map(|line| line.parse::<usize>().unwrap())
                .collect(),
        }
    }
}

fn part1(input: &Input) -> usize {
    input
        .numbers
        .iter()
        .tuple_windows()
        .filter(|&(a, b)| b > a)
        .count()
}

fn part2(input: &Input) -> usize {
    input
        .numbers
        .iter()
        .tuple_windows()
        .filter(|&(a, _b, _c, d)| d > a)
        .count()
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
    const TEST_INPUT: &str = include_str!("../../inputs/test_day01.txt");

    #[test]
    pub fn examples() {
        let input = Input::new(TEST_INPUT);
        assert_eq!(part1(&input), 7);
        assert_eq!(part2(&input), 5);
    }

    #[test]
    pub fn answers() {
        let input = Input::new(INPUT);
        assert_eq!(part1(&input), 1754);
        assert_eq!(part2(&input), 1789);
    }
}
