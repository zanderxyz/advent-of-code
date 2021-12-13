use std::collections::HashMap;

use serde_scan::scan;

const INPUT: &str = include_str!("../../inputs/day12.txt");

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Cave<'a> {
    Start,
    End,
    Small(&'a str),
    Large(&'a str),
}

impl<'a> Cave<'a> {
    pub fn new(s: &'a str) -> Self {
        match s {
            "start" => Self::Start,
            "end" => Self::End,
            _ => {
                let upper = s.to_uppercase();
                if upper == s {
                    Self::Large(s)
                } else {
                    Self::Small(s)
                }
            }
        }
    }

    pub fn is_small(&self) -> bool {
        matches!(self, Cave::Small(_))
    }
}

#[derive(Clone)]
struct Input<'a> {
    pub adjacency: Adjacency<'a>,
}

impl Input<'_> {
    pub fn new(input: &str) -> Input {
        let adjacency = input
            .lines()
            .map(|line| {
                let (from_str, to_str) = scan!("{}-{}" <- line).unwrap();
                let from = Cave::new(from_str);
                let to = Cave::new(to_str);
                (from, to)
            })
            .fold(HashMap::new(), |mut adjacency: Adjacency, (from, to)| {
                if to != Cave::Start && from != Cave::End {
                    adjacency.entry(from.clone()).or_default().push(to.clone());
                }
                if from != Cave::Start && to != Cave::End {
                    adjacency.entry(to).or_default().push(from);
                }
                adjacency
            });
        Input { adjacency }
    }
}

type Adjacency<'a> = HashMap<Cave<'a>, Vec<Cave<'a>>>;

struct Graph<'a> {
    adjacency: &'a Adjacency<'a>,
    visits: HashMap<Cave<'a>, usize>,
}

impl Graph<'_> {
    pub fn new<'a>(adjacency: &'a Adjacency) -> Graph<'a> {
        Graph {
            adjacency,
            visits: HashMap::with_capacity(adjacency.len()),
        }
    }

    pub fn count_routes(&mut self, allow_revisit_one_small: bool) -> usize {
        self.count_dfs(Cave::Start, 0, allow_revisit_one_small)
    }

    fn count_visits(&self, cave: &Cave) -> usize {
        *self.visits.get(cave).unwrap_or(&0)
    }

    fn already_visited(&self, cave: &Cave) -> bool {
        self.count_visits(cave) > 0
    }

    fn count_dfs(&mut self, start: Cave, count: usize, allow_revisit_one_small: bool) -> usize {
        // If this is the end, then this path has completed and we can increment the current count
        if start == Cave::End {
            return count + 1;
        }

        let mut next_count: usize = count;

        // Check all neighbouring caves
        for cave in self.adjacency.get(&start).unwrap() {
            // We can visit any non-small cave, or any non-visited small cave
            let is_small = cave.is_small();
            let already_visited = self.already_visited(cave);
            // If `allow_revisit_one_small` is set, we can visit any cave
            let can_visit = !is_small || !already_visited || allow_revisit_one_small;
            if can_visit {
                // If this is the first time we've made a second visit to this cave, then we no longer allow revisits
                let this_is_second_visit = is_small && already_visited;
                let allow_revisit = allow_revisit_one_small && !this_is_second_visit;

                // Increment the visit count
                *self.visits.entry(cave.clone()).or_insert(0) += 1;

                next_count = self.count_dfs(cave.clone(), next_count, allow_revisit);

                // Decrement the visit count
                *self.visits.entry(cave.clone()).or_insert(0) -= 1;
            }
        }

        next_count
    }
}

fn count_routes_start(adjacency: &Adjacency, allow_revisit_one_small: bool) -> usize {
    let mut graph = Graph::new(adjacency);
    graph.count_routes(allow_revisit_one_small)
}

fn part1(input: &Input) -> usize {
    count_routes_start(&input.adjacency, false)
}

fn part2(input: &Input) -> usize {
    count_routes_start(&input.adjacency, true)
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
    const TEST_INPUT: &str = include_str!("../../inputs/test_day12.txt");

    #[test]
    pub fn examples() {
        let input = Input::new(TEST_INPUT);
        assert_eq!(part1(&input), 10);
        assert_eq!(part2(&input), 36);
    }

    #[test]
    pub fn answers() {
        let input = Input::new(INPUT);
        assert_eq!(part1(&input), 3563);
        assert_eq!(part2(&input), 105453);
    }
}
