use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    fmt,
};

const INPUT: &str = include_str!("../../inputs/day23.txt");
const INPUT2: &str = include_str!("../../inputs/day23_2.txt");

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Amphipod {
    A,
    B,
    C,
    D,
}

impl Amphipod {
    fn cost(&self) -> usize {
        match &self {
            Amphipod::A => 1,
            Amphipod::B => 10,
            Amphipod::C => 100,
            Amphipod::D => 1000,
        }
    }

    fn room_index(&self) -> usize {
        match &self {
            Amphipod::A => 0,
            Amphipod::B => 1,
            Amphipod::C => 2,
            Amphipod::D => 3,
        }
    }

    fn name(&self) -> char {
        match &self {
            Amphipod::A => 'A',
            Amphipod::B => 'B',
            Amphipod::C => 'C',
            Amphipod::D => 'D',
        }
    }
}

impl fmt::Display for Amphipod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())?;

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Positions<const N: usize> {
    // 11 cells of hallway, followed by the 8 cells of the hallways, ordered ABCDABCD in the target
    cells: [Option<Amphipod>; N],
}

const HALLWAY_INDICES: [usize; 7] = [0, 1, 3, 5, 7, 9, 10];
const ROOMS: [usize; 4] = [0, 1, 2, 3];

fn exit_point(room_index: usize) -> usize {
    2 + 2 * room_index
}

fn indices_for_room<const N: usize>(room_index: usize) -> Vec<usize> {
    if N == 19 {
        [11 + room_index, 15 + room_index].into()
    } else if N == 27 {
        [
            11 + room_index,
            15 + room_index,
            19 + room_index,
            23 + room_index,
        ]
        .into()
    } else {
        unreachable!()
    }
}

fn index_to_room(index: usize) -> usize {
    if index < 15 {
        index - 11
    } else if index < 19 {
        index - 15
    } else if index < 23 {
        index - 19
    } else if index < 27 {
        index - 23
    } else {
        panic!("Impossible index {}", index);
    }
}

fn distance<const N: usize>(x: usize, y: usize) -> usize {
    if x > y {
        distance::<N>(y, x)
    } else if y < 11 {
        // Both are in the hallway
        y - x
    } else {
        // Movement between hallway and room
        // x is hallway, y is room
        if x < 11 {
            let room_index = index_to_room(y);
            let exit = exit_point(room_index);
            let distance_to_exit = if y >= 23 {
                // y is at the end of a room
                4
            } else if y >= 19 {
                3
            } else if y >= 15 {
                2
            } else {
                1
            };
            distance_to_exit + distance::<N>(x, exit)
        } else {
            // Both in the rooms?
            todo!()
        }
    }
}

impl<const N: usize> Positions<N> {
    fn new(amphipods: &[Amphipod]) -> Self {
        let mut cells = [None; N];
        for (i, &amphipod) in amphipods.iter().enumerate() {
            cells[11 + i] = Some(amphipod);
        }
        Self { cells }
    }

    fn possible_moves(&self) -> Vec<(usize, usize, usize)> {
        let mut moves = Vec::new();

        for from in HALLWAY_INDICES {
            // Skip empty cells
            if let Some(amphipod) = self.at_index(from) {
                if let Some(to) = self.can_enter_room_from(from, amphipod) {
                    // A room can be entered from here
                    moves.push((from, to, distance::<N>(from, to)));
                }
            }
        }

        for room_index in ROOMS {
            if let Some(from) = self.can_leave_room(room_index) {
                // println!("Can leave room {} and move from {}", room_index, from);
                // Something can leave this room

                for to in self.get_possible_hallway_positions_from(room_index) {
                    // println!(" Can move to {}", to);
                    moves.push((from, to, distance::<N>(from, to)))
                }
            }
        }

        moves
    }

    fn can_enter_room_from(&self, index: usize, amphipod: Amphipod) -> Option<usize> {
        // We want to go into this room
        let target_room = amphipod.room_index();

        // This is the entry point at the top of the room, needs to be unblocked for us to move
        let target_exit = exit_point(target_room);

        if !self.not_blocked(index, target_exit) {
            // Can't get to the target exit
            return None;
        }

        if !self.is_empty_at(target_exit) {
            // The exit for this room is blocked
            return None;
        }

        // Check what is in the room right now
        let room_cells = indices_for_room::<N>(target_room);
        let room_occupants: Vec<Option<Amphipod>> = room_cells
            .iter()
            .map(|&index| self.at_index(index))
            .collect();
        if N == 19 {
            if room_occupants[0].is_none() && room_occupants[1].is_none() {
                // Both are empty, we can move to the end
                return Some(room_cells[1]);
            }
            if room_occupants[0].is_none() && room_occupants[1].unwrap().room_index() == target_room
            {
                // Furthest spot is filled, we can move to the near one and complete this room
                return Some(room_cells[0]);
            }
        } else if N == 27 {
            if room_occupants[0].is_none()
                && room_occupants[1].is_none()
                && room_occupants[2].is_none()
                && room_occupants[3].is_none()
            {
                // All are empty, we can move to the end
                return Some(room_cells[3]);
            }

            if room_occupants[0].is_none()
                && room_occupants[1].is_none()
                && room_occupants[2].is_none()
            {
                // First three are empty
                if room_occupants[3].unwrap().room_index() == target_room {
                    // Three are empty and final is complete, we can move to the third
                    return Some(room_cells[2]);
                } else {
                    return None;
                }
            }

            if room_occupants[0].is_none() && room_occupants[1].is_none() {
                if room_occupants[2].unwrap().room_index() == target_room
                    && room_occupants[3].unwrap().room_index() == target_room
                {
                    // Two are empty and next two are complete, we can move to the third
                    return Some(room_cells[1]);
                } else {
                    return None;
                }
            }

            if room_occupants[0].is_none()
                && room_occupants[1].unwrap().room_index() == target_room
                && room_occupants[2].unwrap().room_index() == target_room
                && room_occupants[3].unwrap().room_index() == target_room
            {
                // Furthest spot is filled, we can move to the near one and complete this room
                return Some(room_cells[0]);
            }
        } else {
            unreachable!();
        }

        None
    }

    fn can_leave_room(&self, room_index: usize) -> Option<usize> {
        indices_for_room::<N>(room_index)
            .into_iter()
            .find(|&i| self.at_index(i).is_some())
    }

    fn get_possible_hallway_positions_from(&self, room_index: usize) -> Vec<usize> {
        HALLWAY_INDICES
            .into_iter()
            // Only empty positions
            .filter(|&i| self.is_empty_at(i))
            // Not blocked from the room exit
            .filter(|&i| self.not_blocked(exit_point(room_index), i))
            .collect()
    }

    fn not_blocked(&self, from: usize, to: usize) -> bool {
        if from > to {
            self.not_blocked(to, from)
        } else {
            // All cells in between these need to be empty
            (from + 1..to).all(|i| self.is_empty_at(i))
        }
    }

    fn is_empty_at(&self, index: usize) -> bool {
        self.at_index(index).is_none()
    }

    fn at_index(&self, i: usize) -> Option<Amphipod> {
        self.cells[i]
    }

    fn update_move(&mut self, from: usize, to: usize) {
        assert!(self.cells[from].is_some());
        assert!(self.cells[to].is_none());

        // Move whatever is at the `from` position to the `to` position
        self.cells[to] = self.cells[from].take();
    }
}

impl<const N: usize> fmt::Display for Positions<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "#############")?;
        write!(f, "#")?;
        for hallway in 0..11 {
            if let Some(amphipod) = self.at_index(hallway) {
                write!(f, "{}", amphipod)?;
            } else {
                write!(f, " ")?;
            }
        }
        writeln!(f, "#")?;
        write!(f, "###")?;
        for index in 11..15 {
            if let Some(amphipod) = self.at_index(index) {
                write!(f, "{}", amphipod)?;
            } else {
                write!(f, " ")?;
            }
            write!(f, "#")?;
        }
        writeln!(f, "##")?;
        write!(f, "###")?;
        for index in 15..19 {
            if let Some(amphipod) = self.at_index(index) {
                write!(f, "{}", amphipod)?;
            } else {
                write!(f, " ")?;
            }
            write!(f, "#")?;
        }
        writeln!(f, "##")?;
        if N == 27 {
            write!(f, "###")?;
            for index in 19..23 {
                if let Some(amphipod) = self.at_index(index) {
                    write!(f, "{}", amphipod)?;
                } else {
                    write!(f, " ")?;
                }
                write!(f, "#")?;
            }
            writeln!(f, "##")?;
            write!(f, "###")?;
            for index in 23..27 {
                if let Some(amphipod) = self.at_index(index) {
                    write!(f, "{}", amphipod)?;
                } else {
                    write!(f, " ")?;
                }
                write!(f, "#")?;
            }
            writeln!(f, "##")?;
        }
        writeln!(f, "#############")?;
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State<const N: usize> {
    positions: Positions<N>,
    energy: usize,
}

impl<const N: usize> State<N> {
    pub fn new(positions: Positions<N>, energy: usize) -> State<N> {
        State { positions, energy }
    }
}

impl<const N: usize> Ord for State<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.energy.cmp(&self.energy)
    }
}

impl<const N: usize> PartialOrd for State<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const N: usize> fmt::Display for State<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.positions)?;
        writeln!(f, "Energy {}", self.energy)?;

        Ok(())
    }
}

#[derive(Clone)]
struct Input<const N: usize> {
    pub positions: Positions<N>,
}

impl<const N: usize> Input<N> {
    pub fn new(input: &str) -> Input<N> {
        let amphipods = {
            let mut iter = input.lines();
            // Skip the first two rows of input
            iter.next();
            iter.next();

            // Find the starting positions of the amphipods
            let mut amphipods = Vec::new();
            for line in iter {
                if line.chars().nth(3) != Some('#') {
                    amphipods.extend(get_amphipods(line));
                }
            }

            amphipods
        };

        let positions = Positions::new(&amphipods);
        Input { positions }
    }
}

fn get_amphipods(row: &str) -> Vec<Amphipod> {
    row.chars()
        .filter_map(|c| match c {
            'A' => Some(Amphipod::A),
            'B' => Some(Amphipod::B),
            'C' => Some(Amphipod::C),
            'D' => Some(Amphipod::D),
            _ => None,
        })
        .collect()
}

fn dijkstra<const N: usize>(start: Positions<N>, target: Positions<N>) -> usize {
    let mut best_distance_to: HashMap<Positions<N>, usize> = HashMap::new();
    let mut queue: BinaryHeap<State<N>> = BinaryHeap::new();

    best_distance_to.insert(start.clone(), 0);
    queue.push(State::new(start, 0));

    while let Some(state) = queue.pop() {
        if state.positions == target {
            return state.energy;
        }

        for (from, to, distance) in state.positions.possible_moves() {
            let extra_cost = distance * state.positions.at_index(from).unwrap().cost();
            let new_cost = state.energy + extra_cost;

            // Work out the new position and
            let mut new_positions = state.positions.clone();
            new_positions.update_move(from, to);

            // Previous best cost to get to this position
            let best_cost_so_far = *best_distance_to
                .entry(new_positions.clone())
                .or_insert(usize::MAX);

            if new_cost < best_cost_so_far {
                // This is better than we'd done so far - update the record, and add this to the queue
                best_distance_to.insert(new_positions.clone(), new_cost);
                queue.push(State::new(new_positions, new_cost));
            }
        }
    }

    panic!("Failed to find a solution")
}

fn part1(input: &Input<19>) -> usize {
    let target = Positions::new(&[
        Amphipod::A,
        Amphipod::B,
        Amphipod::C,
        Amphipod::D,
        Amphipod::A,
        Amphipod::B,
        Amphipod::C,
        Amphipod::D,
    ]);

    dijkstra(input.positions.clone(), target)
}

fn part2(input: &Input<27>) -> usize {
    let target = Positions::new(&[
        Amphipod::A,
        Amphipod::B,
        Amphipod::C,
        Amphipod::D,
        Amphipod::A,
        Amphipod::B,
        Amphipod::C,
        Amphipod::D,
        Amphipod::A,
        Amphipod::B,
        Amphipod::C,
        Amphipod::D,
        Amphipod::A,
        Amphipod::B,
        Amphipod::C,
        Amphipod::D,
    ]);

    dijkstra(input.positions.clone(), target)
}

pub fn main() {
    let input = Input::new(INPUT);
    let input2 = Input::new(INPUT2);
    let answer1 = part1(&input);
    println!("Part 1: {}", answer1);
    let answer2 = part2(&input2);
    println!("Part 2: {}", answer2);
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../inputs/test_day23.txt");
    const TEST_INPUT_2: &str = include_str!("../../inputs/test_day23_2.txt");

    #[test]
    pub fn examples() {
        let input = Input::new(TEST_INPUT);
        assert_eq!(part1(&input), 12521);

        let input2 = Input::new(TEST_INPUT_2);
        assert_eq!(part2(&input2), 44169);
    }

    #[test]
    pub fn answers() {
        let input = Input::new(INPUT);
        assert_eq!(part1(&input), 15385);

        let input = Input::new(INPUT2);
        assert_eq!(part2(&input), 49803);
    }
}
