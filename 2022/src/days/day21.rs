use std::collections::HashMap;

const INPUT: &str = include_str!("../../inputs/day21.txt");

#[derive(Clone, Debug)]
struct Input {
    monkeys: Vec<Monkey>,
}

type Name = String;

#[derive(Clone, Debug)]
struct Monkey {
    name: Name,
    job: Job,
}

#[derive(Clone, Debug)]
enum Job {
    Number(usize),
    Operation(Action, Name, Name),
}

#[derive(Clone, Debug)]
enum Action {
    Add,
    Sub,
    Mul,
    Div,
}

impl Input {
    fn new(input: &str) -> Input {
        let monkeys = input
            .lines()
            .map(|line| {
                let mut iter = line.split(": ");
                let name = iter.next().unwrap().to_owned();
                let job_str = iter.next().unwrap();
                let job = if let Ok(value) = job_str.parse::<usize>() {
                    Job::Number(value)
                } else {
                    let left = job_str[..4].to_owned();
                    let right = job_str[7..].to_owned();
                    let action = match job_str.chars().nth(5).unwrap() {
                        '+' => Action::Add,
                        '-' => Action::Sub,
                        '*' => Action::Mul,
                        '/' => Action::Div,
                        other => panic!("Unexpected char {}", other),
                    };
                    Job::Operation(action, left, right)
                };
                Monkey { name, job }
            })
            .collect();

        Input { monkeys }
    }
}

#[derive(Debug, Clone)]
struct MonkeyForest {
    monkeys: HashMap<Name, Job>,
    cache: HashMap<Name, usize>,
}

impl MonkeyForest {
    fn new(monkeys: Vec<Monkey>) -> Self {
        Self {
            monkeys: monkeys
                .iter()
                .map(|m| (m.name.clone(), m.job.clone()))
                .collect(),
            cache: HashMap::new(),
        }
    }

    fn set_action(&mut self, name: &str, action: Action) {
        let job = &self.monkeys[name];
        if let Job::Operation(_, left, right) = job {
            self.monkeys.insert(
                name.to_owned(),
                Job::Operation(action, left.clone(), right.clone()),
            );
        } else {
            panic!("Monkey does not have operation, cannot set action")
        }
    }

    fn find_number(&mut self, name: &str) -> usize {
        let monkey = self.monkeys[name].clone();
        match monkey {
            Job::Number(val) => val,
            Job::Operation(ref action, ref left, ref right) => {
                let left = if self.cache.contains_key(left) {
                    self.cache[left]
                } else {
                    self.find_number(left)
                };
                let right = if self.cache.contains_key(right) {
                    self.cache[right]
                } else {
                    self.find_number(right)
                };

                match action {
                    Action::Add => left + right,
                    Action::Sub => left - right,
                    Action::Mul => left * right,
                    Action::Div => left / right,
                }
            }
        }
    }

    fn find_number_derived(&mut self, target: &str) -> usize {
        // humn is meant to be an input, and root = x EQ y
        // But we can rebase, and instead search for humn given root = x - y = 0

        // We need to figure out humn, so we remove it
        self.monkeys.remove(target);

        // We know root must = 0
        self.cache.insert("root".to_owned(), 0);

        // We also know that root = x - y
        self.set_action("root", Action::Sub);

        // This lets us derive:
        //  x = y + root
        //  y = x - root
        // We can do the same derivation for all other formulas
        let derived_formulas: Vec<(Name, Job)> = self
            .monkeys
            .iter()
            .flat_map(|(name, job)| match job {
                Job::Number(_) => vec![(name.clone(), job.clone())],
                Job::Operation(action, left, right) => match action {
                    // N = L + R
                    // L = N - R
                    // R = N - L
                    Action::Add => vec![
                        (
                            name.clone(),
                            Job::Operation(Action::Add, left.clone(), right.clone()),
                        ),
                        (
                            left.clone(),
                            Job::Operation(Action::Sub, name.clone(), right.clone()),
                        ),
                        (
                            right.clone(),
                            Job::Operation(Action::Sub, name.clone(), left.clone()),
                        ),
                    ],
                    // N = L - R
                    // L = N + R
                    // R = L - N
                    Action::Sub => vec![
                        (
                            name.clone(),
                            Job::Operation(Action::Sub, left.clone(), right.clone()),
                        ),
                        (
                            left.clone(),
                            Job::Operation(Action::Add, name.clone(), right.clone()),
                        ),
                        (
                            right.clone(),
                            Job::Operation(Action::Sub, left.clone(), name.clone()),
                        ),
                    ],
                    // N = L * R
                    // L = N / R
                    // R = N / L
                    Action::Mul => vec![
                        (
                            name.clone(),
                            Job::Operation(Action::Mul, left.clone(), right.clone()),
                        ),
                        (
                            left.clone(),
                            Job::Operation(Action::Div, name.clone(), right.clone()),
                        ),
                        (
                            right.clone(),
                            Job::Operation(Action::Div, name.clone(), left.clone()),
                        ),
                    ],
                    // N = L / R
                    // L = N * R
                    // R = L / N
                    Action::Div => vec![
                        (
                            name.clone(),
                            Job::Operation(Action::Div, left.clone(), right.clone()),
                        ),
                        (
                            left.clone(),
                            Job::Operation(Action::Mul, name.clone(), right.clone()),
                        ),
                        (
                            right.clone(),
                            Job::Operation(Action::Div, left.clone(), name.clone()),
                        ),
                    ],
                },
            })
            .collect();

        // Loop through inefficiently
        // We should find at least one result on each cycle, so this is O(n^2)
        while !self.cache.contains_key(target) {
            for (name, job) in derived_formulas.iter() {
                if self.cache.contains_key(name) {
                    continue;
                }
                if let Some(value) = self.find_number_once(job) {
                    self.cache.insert(name.clone(), value);
                }
            }
        }

        self.cache[target]
    }

    fn find_number_once(&self, job: &Job) -> Option<usize> {
        match job {
            Job::Number(val) => Some(*val),
            Job::Operation(ref action, ref left, ref right) => {
                let left = if self.cache.contains_key(left) {
                    self.cache[left]
                } else {
                    return None;
                };
                let right = if self.cache.contains_key(right) {
                    self.cache[right]
                } else {
                    return None;
                };

                let v = match action {
                    Action::Add => left + right,
                    Action::Sub => left - right,
                    Action::Mul => left * right,
                    Action::Div => left / right,
                };

                Some(v)
            }
        }
    }
}

fn part1(input: &Input) -> usize {
    let mut forest = MonkeyForest::new(input.monkeys.clone());
    forest.find_number("root")
}

fn part2(input: &Input) -> usize {
    let mut forest = MonkeyForest::new(input.monkeys.clone());
    forest.find_number_derived("humn")
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
    const TEST_INPUT: &str = include_str!("../../inputs/test_day21.txt");

    #[test]
    fn examples() {
        let input = Input::new(TEST_INPUT);
        assert_eq!(part1(&input), 152);
        assert_eq!(part2(&input), 301);
    }

    #[test]
    fn answers() {
        let input = Input::new(INPUT);
        assert_eq!(part1(&input), 80326079210554);
        assert_eq!(part2(&input), 3617613952378);
    }
}
