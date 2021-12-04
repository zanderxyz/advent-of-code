const INPUT: &str = include_str!("../../inputs/day03.txt");

#[derive(Clone, Debug)]
// Any excuse to use const generics
// This type is parametrised over the length of each binary in the input file
// In retrospect it would probably have been better to just use a Vec and pass the length as a second input parameter
pub struct Input<const N: usize> {
    pub numbers: Vec<[bool; N]>,
}

impl<const N: usize> Input<N> {
    pub fn new(input: &str) -> Input<N> {
        Input {
            numbers: input
                .lines()
                .map(|line| {
                    // Initialise the array of bools to false
                    let mut array: [bool; N] = [false; N];
                    // For each char in the string, if the value is 1, we update the array value to true
                    line.chars().enumerate().for_each(|(i, char)| match char {
                        '0' => {}
                        '1' => array[i] = true,
                        _ => panic!("Unexpected non-binary character in input"),
                    });
                    array
                })
                .collect(),
        }
    }
}

pub fn part1<const N: usize>(input: &Input<N>) -> usize {
    let input_length = input.numbers.len();

    // Count the number of 1s in each column
    let counts = input
        .numbers
        .iter()
        .fold([0usize; N], |mut counts, &number| {
            for (i, value) in number.iter().enumerate() {
                if *value {
                    counts[i] += 1;
                }
            }
            counts
        });

    // Build gamma as a binary array
    let gamma_binary =
        counts
            .into_iter()
            .enumerate()
            .fold([false; N], |mut gamma_binary, (i, count)| {
                gamma_binary[i] = count >= input_length / 2;
                gamma_binary
            });

    let gamma = binary_to_integer(gamma_binary);

    let mask = 2usize.pow(N.try_into().unwrap()) - 1;
    let epsilon = mask - gamma;
    gamma * epsilon
}

pub fn part2<const N: usize>(input: &Input<N>) -> usize {
    let oxygen_generator = filter_numbers(input.numbers.clone(), true);
    let co2_scrubber = filter_numbers(input.numbers.clone(), false);

    oxygen_generator * co2_scrubber
}

fn filter_numbers<const N: usize>(numbers: Vec<[bool; N]>, more_ones_than_zeros: bool) -> usize {
    // Iterate through columns
    let mut column: usize = 0;
    let mut current_input = numbers;
    while column < N {
        // Count the number of 1s in this column
        let count = current_input.iter().fold(0usize, |mut count, &number| {
            if number[column] {
                count += 1;
            }
            count
        });

        // Work out the target count (half of the remaining size, rounded up)
        let target_count = if current_input.len() % 2 == 0 {
            current_input.len() / 2
        } else {
            (current_input.len() + 1) / 2
        };

        let target = (count >= target_count) == more_ones_than_zeros;

        // Filter the existing input based on this target
        current_input = current_input
            .into_iter()
            .filter(|&number| number[column] == target)
            .collect();

        // If only one left, we are done
        if current_input.len() == 1 {
            break;
        }

        column += 1;
    }

    // We have only one of the input numbers left
    assert!(current_input.len() == 1);

    binary_to_integer(*current_input.first().unwrap())
}

fn binary_to_integer<const N: usize>(binary: [bool; N]) -> usize {
    binary
        .into_iter()
        .enumerate()
        .fold(0usize, |mut acc, (i, value)| {
            if value {
                acc += 2usize.pow((N - (i + 1)).try_into().unwrap())
            }
            acc
        })
}

pub fn main() {
    let input = Input::<12>::new(INPUT);
    let answer1 = part1(&input);
    println!("Part 1: {}", answer1);
    let answer2 = part2(&input);
    println!("Part 2: {}", answer2);
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/test_day03.txt");

    #[test]
    pub fn examples() {
        let input = Input::<5>::new(TEST_INPUT);
        assert_eq!(part1(&input), 198);
        assert_eq!(part2(&input), 230);
    }

    #[test]
    pub fn answers() {
        let input = Input::<12>::new(INPUT);
        assert_eq!(part1(&input), 845186);
        assert_eq!(part2(&input), 4636702);
    }
}
