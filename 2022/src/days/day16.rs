use std::{collections::HashMap, convert::Infallible, str::FromStr};

use itertools::Itertools;
use serde_scan::scan;

const INPUT: &str = include_str!("../../inputs/day16.txt");

#[derive(Clone, Debug)]
struct Input {
    valves: Vec<InputCave>,
}

impl Input {
    fn new(input: &str) -> Input {
        let valves = input
            .lines()
            .map(|line| line.parse::<InputCave>().unwrap())
            .collect();

        Input { valves }
    }
}

type ValveName = String;

#[derive(Clone, Debug)]
struct InputCave {
    name: ValveName,
    flow_rate: usize,
    tunnels: Vec<ValveName>,
}

impl FromStr for InputCave {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Modified input slightly to avoid the plural/singular difference (tunnels lead vs tunnel leads)
        let (name, flow_rate, tunnels): (String, usize, String) =
            scan!("Valve {} has flow rate={}; tunnels lead to valves {}" <- s).unwrap();
        let tunnels = tunnels.split(", ").map(|s| s.to_owned()).collect();
        Ok(Self {
            name,
            flow_rate,
            tunnels,
        })
    }

    type Err = Infallible;
}

type ValveId = usize;

#[derive(Clone, Debug)]
struct CaveSystem<const N: usize> {
    start: usize,
    // time_to_travel[a][b] is the time to travel from a to b
    time_to_travel: [[usize; N]; N],
    // flow_rates[a] is the flow rate per minute from valve a
    flow_rates: [usize; N],
}

// Floyd-Warshall algorithm to find all pairwise shortest distances
fn pairwise_shortest_distances<const N: usize>(
    edges: impl Iterator<Item = (usize, usize)>,
) -> [[usize; N]; N] {
    let mut time_to_travel = [[usize::MAX; N]; N];

    for (from, to) in edges {
        time_to_travel[from][to] = 1;
        time_to_travel[to][from] = 1;
    }

    (0..N).for_each(|i| {
        time_to_travel[i][i] = 0;
    });

    for k in 0..N {
        for i in 0..N {
            for j in 0..N {
                if time_to_travel[i][k] == usize::MAX || time_to_travel[k][j] == usize::MAX {
                    continue;
                }
                if time_to_travel[i][j] > time_to_travel[i][k] + time_to_travel[k][j] {
                    time_to_travel[i][j] = time_to_travel[i][k] + time_to_travel[k][j]
                }
            }
        }
    }

    time_to_travel
}

impl<const N: usize> CaveSystem<N> {
    fn build(caves: Vec<InputCave>) -> Self {
        let flow_rates = caves
            .iter()
            .map(|c| c.flow_rate)
            .collect_vec()
            .try_into()
            .unwrap();

        // We need a pairwise collection of the time to get between each pair of caves
        // Get a map so we can convert from name to id
        let name_to_id: HashMap<ValveName, usize> = caves
            .iter()
            .enumerate()
            .map(|(index, c)| (c.name.clone(), index))
            .collect();

        // Get all the edges in the graph
        let edges = caves.iter().enumerate().flat_map(|(index, cave)| {
            cave.tunnels
                .iter()
                .map(|name| (index, *name_to_id.get(name).unwrap()))
                .collect_vec()
        });
        let time_to_travel = pairwise_shortest_distances(edges);

        Self {
            start: name_to_id["AA"],
            time_to_travel,
            flow_rates,
        }
    }

    fn optimise<const M: usize>(self) -> CaveSystem<M> {
        // Optimise a CaveSystem by ordering by flow rate and removing those with zero weight

        // Maps from new_id to old_id + flow_rate
        let flow_rates_map: Vec<(usize, usize)> = self
            .flow_rates
            .into_iter()
            .enumerate()
            .filter(|&(i, f)| i == self.start || f > 0)
            .sorted_by_key(|&(i, flow_rate)| {
                // Ensure that the starting point moves to the first index
                if i == self.start {
                    isize::MIN
                } else {
                    -(flow_rate as isize)
                }
            })
            .collect();

        // Order the flow rates
        let flow_rates = flow_rates_map
            .iter()
            .map(|(_, f)| *f)
            .collect_vec()
            .try_into()
            .unwrap();

        // Build a new time to travel matrix
        let mut time_to_travel = [[usize::MAX; M]; M];
        for i in 0..M {
            for j in 0..M {
                if i == j {
                    time_to_travel[i][j] = 0;
                } else {
                    time_to_travel[i][j] =
                        self.time_to_travel[flow_rates_map[i].0][flow_rates_map[j].0]
                }
            }
        }

        CaveSystem {
            start: 0,
            time_to_travel,
            flow_rates,
        }
    }

    fn flow(&self, location: usize) -> usize {
        if location < self.flow_rates.len() {
            self.flow_rates[location]
        } else {
            0
        }
    }

    fn times_to_travel_from(&self, location: usize) -> &[usize] {
        if location < self.time_to_travel.len() {
            &self.time_to_travel[location]
        } else {
            &[]
        }
    }

    fn possible_destinations<'a>(
        &'a self,
        person: &'a Person,
        visited: &'a [bool],
    ) -> impl Iterator<Item = (usize, usize)> + 'a {
        self.times_to_travel_from(person.location)
            .iter()
            .enumerate()
            .filter(|&(next, cost)| person.time_left > *cost && !visited[next])
            .map(|(next, cost)| (next, *cost))
    }
}

#[derive(Clone, Debug)]
struct CaveExplorer<const N: usize> {
    max_pressure_released: usize,
    visited: Vec<bool>,
}

impl<const N: usize> CaveExplorer<N> {
    fn new() -> Self {
        Self {
            max_pressure_released: 0,
            visited: vec![false; N + 1],
        }
    }

    fn search(&mut self, system: &CaveSystem<N>, total_time: usize) -> usize {
        let total_flow = system.flow_rates.iter().sum();
        let state = State::new(0, total_time, total_flow);
        self.search_from(system, state);
        self.max_pressure_released
    }

    fn search_from(&mut self, system: &CaveSystem<N>, mut state: State<N>) {
        let location = state.person.location;
        let flow = system.flow(location);
        self.visited[location] = true;
        state.open_valve(flow);
        if state.pressure_released > self.max_pressure_released {
            self.max_pressure_released = state.pressure_released;
        }

        let visited = self.visited.clone();
        let possible_destinations = system.possible_destinations(&state.person, &visited);

        for (next, cost) in possible_destinations {
            let mut state = state.clone();
            state.person.move_to(next, cost);
            if state.max_possible_score() > self.max_pressure_released {
                self.search_from(system, state);
            }
        }

        self.visited[location] = false;
    }

    fn search_pair(&mut self, system: &CaveSystem<N>, total_time: usize) -> usize {
        let total_flow = system.flow_rates.iter().sum();
        let state = StatePair::new(0, total_time, total_flow);
        self.search_pair_from(system, state);
        self.max_pressure_released
    }

    fn search_pair_from(&mut self, system: &CaveSystem<N>, mut state: StatePair<N>) {
        let location_person = state.person.location;
        let location_elephant = state.elephant.location;
        self.visited[location_person] = true;
        self.visited[location_elephant] = true;

        let flow_person = system.flow(location_person);
        let flow_elephant = system.flow(location_elephant);

        state.open_valve(flow_person, state.person.time_left);
        state.open_valve(flow_elephant, state.elephant.time_left);

        if state.pressure_released > self.max_pressure_released {
            self.max_pressure_released = state.pressure_released;
        }

        let visited = self.visited.clone();
        let possible_destinations_person = system.possible_destinations(&state.person, &visited);
        let possible_destinations_elephant =
            system.possible_destinations(&state.elephant, &visited);

        // Allow the elephant to move to a hidden cave, effectively stopping it's movement
        let mut possible_destinations_elephant = possible_destinations_elephant.collect_vec();
        possible_destinations_elephant.push((N, 0));

        for (next, cost) in possible_destinations_person {
            let mut state = state.clone();
            state.person.move_to(next, cost);

            for (next2, cost2) in possible_destinations_elephant
                .iter()
                .filter(|&(next2, _)| *next2 != next)
            {
                let mut state = state.clone();
                state.elephant.move_to(*next2, *cost2);

                if state.max_possible_score() > self.max_pressure_released {
                    self.search_pair_from(system, state);
                }
            }
        }

        self.visited[location_person] = false;
        self.visited[location_elephant] = false;
    }
}

#[derive(Clone, Debug, Copy)]
struct Person {
    time_left: usize,
    location: ValveId,
}

impl Person {
    fn move_to(&mut self, valve_id: ValveId, time_taken: usize) {
        if self.time_left == time_taken {
            self.time_left = 0;
        } else {
            self.time_left -= time_taken + 1;
        }
        self.location = valve_id;
    }
}

#[derive(Clone, Debug)]
struct State<const N: usize> {
    pressure_released: usize,
    remaining_flow: usize,
    person: Person,
}

impl<const N: usize> State<N> {
    fn new(location: usize, time_left: usize, remaining_flow: usize) -> Self {
        Self {
            remaining_flow,
            pressure_released: 0,
            person: Person {
                time_left,
                location,
            },
        }
    }

    fn open_valve(&mut self, flow_rate: usize) {
        self.pressure_released += self.person.time_left * flow_rate;
        self.remaining_flow -= flow_rate;
    }

    fn max_possible_score(&self) -> usize {
        self.pressure_released + self.person.time_left * self.remaining_flow
    }
}

#[derive(Clone, Debug)]
struct StatePair<const N: usize> {
    pressure_released: usize,
    remaining_flow: usize,
    person: Person,
    elephant: Person,
}

impl<const N: usize> StatePair<N> {
    fn new(location: usize, time_left: usize, remaining_flow: usize) -> Self {
        Self {
            remaining_flow,
            pressure_released: 0,
            person: Person {
                time_left,
                location,
            },
            elephant: Person {
                time_left,
                location,
            },
        }
    }

    fn open_valve(&mut self, flow_rate: usize, time_left: usize) {
        self.pressure_released += time_left * flow_rate;

        self.remaining_flow -= flow_rate;
    }

    fn max_possible_score(&self) -> usize {
        self.pressure_released
            + self.person.time_left.max(self.elephant.time_left) * self.remaining_flow
    }
}

fn part1<const N: usize, const M: usize>(input: &Input) -> usize {
    let system: CaveSystem<M> = CaveSystem::<N>::build(input.valves.clone()).optimise();

    CaveExplorer::new().search(&system, 30)
}

fn part2<const N: usize, const M: usize>(input: &Input) -> usize {
    let system: CaveSystem<M> = CaveSystem::<N>::build(input.valves.clone()).optimise();

    CaveExplorer::new().search_pair(&system, 26)
}

pub fn main() {
    let input = Input::new(INPUT);
    let answer1 = part1::<57, 16>(&input);
    println!("Part 1: {}", answer1);
    let answer2 = part2::<57, 16>(&input);
    println!("Part 2: {}", answer2);
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/test_day16.txt");

    #[test]
    fn examples() {
        let input = Input::new(TEST_INPUT);
        assert_eq!(part1::<10, 7>(&input), 1651);
        assert_eq!(part2::<10, 7>(&input), 1707);
    }

    #[test]
    fn answers() {
        let input = Input::new(INPUT);
        assert_eq!(part1::<57, 16>(&input), 2359);
        assert_eq!(part2::<57, 16>(&input), 2999);
    }
}
