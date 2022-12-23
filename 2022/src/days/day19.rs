use std::{convert::Infallible, str::FromStr};

use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day19.txt");

#[derive(Clone, Debug)]
struct Input {
    blueprints: Vec<Blueprint>,
}

impl Input {
    fn new(input: &str) -> Input {
        let blueprints = input
            .lines()
            .map(|line| {
                let line = line.split_once(':').unwrap().1;
                let costs = line
                    .split('.')
                    .filter(|line| !line.is_empty())
                    .map(|line| {
                        let line = line.strip_prefix(" Each ").unwrap();
                        let (_, line) = line.split_once(' ').unwrap();
                        let line = line.strip_prefix("robot costs ").unwrap();
                        let costs: Vec<(Material, usize)> = line
                            .split(" and ")
                            .map(|s| {
                                let (cost, material) = s.split_once(' ').unwrap();
                                (material.parse().unwrap(), cost.parse().ok().unwrap())
                            })
                            .collect();
                        costs
                    })
                    .collect_vec()
                    .try_into()
                    .unwrap();

                Blueprint { costs }
            })
            .collect();
        Input { blueprints }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl Material {
    fn all() -> [Material; 4] {
        [
            Material::Geode,
            Material::Obsidian,
            Material::Clay,
            Material::Ore,
        ]
    }
}

impl FromStr for Material {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "ore" => Material::Ore,
            "clay" => Material::Clay,
            "obsidian" => Material::Obsidian,
            "geode" => Material::Geode,
            _ => unreachable!(),
        })
    }
}

#[derive(Clone, Debug)]
struct Blueprint {
    costs: [Vec<(Material, usize)>; 4],
}

#[derive(Clone, Debug)]
struct State {
    flow: [usize; 4],
    stock: [usize; 4],
    time_left: usize,
}

impl State {
    fn new(time_left: usize) -> Self {
        Self {
            flow: [1, 0, 0, 0],
            stock: [0, 0, 0, 0],
            time_left,
        }
    }

    fn stock(&self, target: &Material) -> usize {
        self.stock[*target as usize]
    }

    fn sufficient_to_build(&self, costs: &[(Material, usize)]) -> bool {
        costs.iter().all(|(item, cost)| self.stock(item) >= *cost)
    }

    fn flow(&self, target: &Material) -> usize {
        self.flow[*target as usize]
    }

    fn build_robot(&mut self, target: &Material, costs: &[(Material, usize)]) {
        self.flow[*target as usize] += 1;
        for (item, cost) in costs {
            self.stock[*item as usize] -= cost;
        }
    }

    fn tick(&mut self) {
        self.time_left -= 1;
        for i in 0..4 {
            self.stock[i] += self.flow[i];
        }
    }

    // The highest possible score we can reach from our current state if we build a geode every turn
    fn max_possible_score(&self) -> usize {
        self.stock(&Material::Geode)
            + self.flow(&Material::Geode) * self.time_left
            + ((self.time_left - 1) * self.time_left) / 2
    }

    fn has_flow_required_for(&self, target: &Material) -> bool {
        // Check if we have the prerequisite flow required to build a particular target
        match target {
            Material::Geode => self.flow(&Material::Obsidian) > 0,
            Material::Obsidian => self.flow(&Material::Clay) > 0,
            _ => true,
        }
    }
}

impl Blueprint {
    fn cost(&self, target: &Material) -> &Vec<(Material, usize)> {
        &self.costs[*target as usize]
    }

    fn most_we_can_spend_per_turn(&self, target: &Material) -> usize {
        // We can only build one robot per turn
        // So we can calculate the maximum we can spend per turn of any material
        *self
            .costs
            .iter()
            .flatten()
            .filter(|(m, _)| m == target)
            .map(|(_, c)| c)
            .max()
            .unwrap_or(&0)
    }

    fn max_score(&self, time_left: usize) -> usize {
        let state = State::new(time_left);
        let mut best_score = 0;

        // Initially we only have the items to try building ore or clay robots
        for target_robot in [Material::Ore, Material::Clay] {
            self.find_best_score(state.clone(), &target_robot, &mut best_score)
        }

        best_score
    }

    fn we_have_sufficient_already(&self, state: &State, target: &Material) -> bool {
        if *target == Material::Geode {
            // We always want to build geodes
            return false;
        }

        // We can only spend a certain amount per turn, so we may already have sufficient material, no point generating more
        let most_we_can_spend_per_turn = self.most_we_can_spend_per_turn(target);
        let most_we_need_from_here = most_we_can_spend_per_turn * (state.time_left - 1);
        let amount_we_generate_already = state.flow(target) * (state.time_left - 2);
        let most_we_need =
            most_we_can_spend_per_turn.max(most_we_need_from_here - amount_we_generate_already);

        state.flow(target) >= most_we_can_spend_per_turn || state.stock(target) >= most_we_need
    }

    fn find_best_score(&self, mut state: State, target: &Material, best_score: &mut usize) {
        while state.time_left > 0 {
            if state.sufficient_to_build(self.cost(target)) {
                // We can build a robot and move forward
                state.tick();
                state.build_robot(target, self.cost(target));

                for target_robot in Material::all() {
                    // End the branch if we can't build any of this material yet
                    if state.has_flow_required_for(&target_robot)
                        // End the branch if it's not possible to beat our best score
                        && state.max_possible_score() > *best_score
                        // End the branch if we already have enough of this particular material
                        && !self.we_have_sufficient_already(&state, &target_robot)
                    {
                        self.find_best_score(state.clone(), &target_robot, best_score)
                    }
                }
                return;
            }
            state.tick();
        }

        // We cannot build a robot, we are at a final state
        let geodes = state.stock(&Material::Geode);
        if geodes > *best_score {
            *best_score = geodes;
        }
    }
}

fn part1(input: &Input) -> usize {
    input
        .blueprints
        .iter()
        .enumerate()
        .map(|(i, b)| b.max_score(24) * (i + 1))
        .sum()
}

fn part2(input: &Input) -> usize {
    input
        .blueprints
        .iter()
        .take(3)
        .map(|b| b.max_score(32))
        .product()
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
    const TEST_INPUT: &str = include_str!("../../inputs/test_day19.txt");

    #[test]
    fn examples() {
        let input = Input::new(TEST_INPUT);
        assert_eq!(part1(&input), 33);
        assert_eq!(part2(&input), 3472);
    }
    #[test]
    fn answers() {
        let input = Input::new(INPUT);
        assert_eq!(part1(&input), 1192);
        assert_eq!(part2(&input), 14725);
    }
}
