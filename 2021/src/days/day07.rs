const INPUT: &str = include_str!("../../inputs/day07.txt");

#[derive(Clone)]
struct Input {
    pub positions: Vec<usize>,
}

impl Input {
    pub fn new(input: &str) -> Input {
        Input {
            positions: input
                .trim_end()
                .split(',')
                .map(|value| value.parse::<usize>().unwrap())
                .collect(),
        }
    }
}

fn unsigned_abs_diff(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn required_fuel_simple(positions: &[usize], chosen: usize) -> usize {
    positions
        .iter()
        .map(|&p| unsigned_abs_diff(p, chosen))
        .sum()
}

fn required_fuel_complex(positions: &[usize], chosen: usize) -> usize {
    positions
        .iter()
        .map(|&p| {
            let distance = unsigned_abs_diff(p, chosen);
            required_fuel_complex_distance(distance)
        })
        .sum()
}

fn required_fuel_complex_distance(distance: usize) -> usize {
    distance * (distance + 1) / 2
}

fn part1(input: &Input) -> usize {
    // Take the median value of the input, by sorting it and finding the middle
    let mut sorted = input.positions.clone();
    sorted.sort_unstable();

    let count = sorted.len();
    // If an even number, the median is one of two. If odd, there is a middle one
    let possible_median_indexes = if count % 2 == 0 {
        vec![count / 2 - 2, count / 2]
    } else {
        vec![(count - 1) / 2]
    };

    // The answer is the fuel used to get to the median value in the input
    possible_median_indexes
        .iter()
        .map(|&i| sorted[i])
        .map(|v| required_fuel_simple(&input.positions, v))
        .min()
        .unwrap()
}

fn part2(input: &Input) -> usize {
    // Calculate the mean value of the input
    let sum = input.positions.iter().sum::<usize>();
    let count = input.positions.len();
    let mean = sum / count;

    // Check the values from 1 below to 1 above the mean, and take the best
    (mean - 1..=mean + 1)
        .map(|v| required_fuel_complex(&input.positions, v))
        .min()
        .unwrap()
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
    const TEST_INPUT: &str = include_str!("../../inputs/test_day07.txt");

    #[test]
    pub fn examples() {
        let input = Input::new(TEST_INPUT);
        assert_eq!(part1(&input), 37);
        assert_eq!(part2(&input), 168);
    }

    #[test]
    pub fn answers() {
        let input = Input::new(INPUT);
        assert_eq!(part1(&input), 347449);
        assert_eq!(part2(&input), 98039527);
    }
}
