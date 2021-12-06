const INPUT: &str = include_str!("../../inputs/day06.txt");

const NEW_TIMER: usize = 8;
const AGE_AFTER_BIRTH: usize = 6;

// Array where index = fish timer, value = number of fish with this timer
type Fish = [usize; NEW_TIMER + 1];

#[derive(Clone)]
struct Input {
    pub fish_timers: Vec<usize>,
}

impl Input {
    pub fn new(input: &str) -> Input {
        Input {
            fish_timers: input
                .trim_end()
                .split(',')
                .map(|value| value.parse::<usize>().unwrap())
                .collect(),
        }
    }
}

fn group_fish_by_timer(input: &Input) -> Fish {
    let mut fish: Fish = [0; NEW_TIMER + 1];
    for &age in input.fish_timers.iter() {
        fish[age] += 1;
    }
    fish
}

fn tick(state: &mut Fish) {
    let number_of_zeros = state[0];
    rotate(state);
    add_new_fish(state, number_of_zeros);
}

fn rotate(state: &mut Fish) {
    state.rotate_left(1);
    state[AGE_AFTER_BIRTH] += state[NEW_TIMER];
}

fn add_new_fish(state: &mut Fish, new_fish: usize) {
    state[NEW_TIMER] = new_fish;
}

fn run_iterations(input: &Input, n: usize) -> usize {
    let mut fish = group_fish_by_timer(input);
    for _ in 0..n {
        tick(&mut fish);
    }

    fish.iter().sum()
}

fn part1(input: &Input) -> usize {
    run_iterations(input, 80)
}

fn part2(input: &Input) -> usize {
    run_iterations(input, 256)
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
    pub fn examples() {
        let input = Input::new(TEST_INPUT);
        assert_eq!(part1(&input), 5934);
        assert_eq!(part2(&input), 26984457539);
    }

    #[test]
    pub fn answers() {
        let input = Input::new(INPUT);
        assert_eq!(part1(&input), 385391);
        assert_eq!(part2(&input), 1728611055389);
    }
}
