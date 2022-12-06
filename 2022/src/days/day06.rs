use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day06.txt");

#[derive(Clone, Debug)]
struct Input {
    signal: Vec<char>,
}

impl Input {
    fn new(input: &str) -> Input {
        Input {
            signal: input.chars().collect(),
        }
    }
}

// Returns the position of the end of the first run of n unique items
// The size of the data here is tiny, so we don't have to do anything clever
fn find_first_n_unique_items_index(input: &[char], n: usize) -> usize {
    let index = input
        .windows(n)
        .into_iter()
        .map(|window| window.iter().all_unique())
        .position(|b| b)
        .expect("no run of n unique items found");

    index + n
}

fn part1(input: &Input) -> usize {
    find_first_n_unique_items_index(&input.signal, 4)
}

fn part2(input: &Input) -> usize {
    find_first_n_unique_items_index(&input.signal, 14)
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
    const TEST_INPUT: &str = include_str!("../../inputs/test_day06.txt");

    #[test]
    fn examples() {
        let input = Input::new(TEST_INPUT);
        assert_eq!(part1(&input), 7);
        assert_eq!(part2(&input), 19);
    }

    #[test]
    fn answers() {
        let input = Input::new(INPUT);
        assert_eq!(part1(&input), 1848);
        assert_eq!(part2(&input), 2308);
    }
}
