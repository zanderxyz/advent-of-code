use std::collections::HashMap;

use serde_scan::scan;

use crate::helpers::increment::Increment;

const INPUT: &str = include_str!("../../inputs/day21.txt");

#[derive(Clone)]
struct Input {
    pub p1: usize,
    pub p2: usize,
}

impl Input {
    pub fn new(input: &str) -> Input {
        let mut iter = input.lines();
        let p1_str = iter.next().unwrap();
        let p2_str = iter.next().unwrap();
        let p1 = scan!("Player 1 starting position: {}" <- p1_str).unwrap();
        let p2 = scan!("Player 2 starting position: {}" <- p2_str).unwrap();
        Input { p1, p2 }
    }
}

trait Die {
    fn roll(&mut self) -> usize;
    fn roll3(&mut self) -> usize;
}

struct DeterministicDie {
    next: usize,
    rolls: usize,
}

impl DeterministicDie {
    pub fn new() -> Self {
        Self { next: 1, rolls: 0 }
    }
}

impl Die for DeterministicDie {
    fn roll(&mut self) -> usize {
        let next = self.next;
        self.rolls += 1;
        self.next += 1;
        if self.next > 100 {
            self.next -= 100;
        }
        next
    }

    fn roll3(&mut self) -> usize {
        let first = self.roll();
        let second = self.roll();
        let third = self.roll();
        first + second + third
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Player {
    position: usize,
    score: usize,
}

impl Player {
    pub fn new(position: usize) -> Self {
        Self { position, score: 0 }
    }

    pub fn go(&mut self, roll: usize) {
        self.position += roll;
        while self.position > 10 {
            self.position -= 10;
        }
        self.score += self.position;
    }
}

fn part1(input: &Input) -> usize {
    let mut p1 = Player::new(input.p1);
    let mut p2 = Player::new(input.p2);
    let mut die = DeterministicDie::new();

    loop {
        let p1_roll = die.roll3();
        p1.go(p1_roll);
        if p1.score >= 1000 {
            break;
        }

        let p2_roll = die.roll3();
        p2.go(p2_roll);
        if p2.score >= 1000 {
            break;
        }
    }

    die.rolls * p1.score.min(p2.score)
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
// Player 1, Player 2, "Is Player 1 going next?"
struct GameState(Player, Player, bool);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum GameUpdate {
    Live(GameState, usize),
    Finished(usize),
}

impl GameState {
    fn progress(&self) -> [GameUpdate; 7] {
        // These are all the possible combinations of three rolls, along with how many universes they occur in
        [
            self.apply(3, 1),
            self.apply(4, 3),
            self.apply(5, 6),
            self.apply(6, 7),
            self.apply(7, 6),
            self.apply(8, 3),
            self.apply(9, 1),
        ]
    }

    fn apply(&self, roll: usize, count: usize) -> GameUpdate {
        // Update the game to reflect the roll
        let game = self.clone();
        let mut player1 = game.0;
        let mut player2 = game.1;

        // The third field tells us which player is going next
        let player = if game.2 { &mut player1 } else { &mut player2 };
        player.go(roll);

        if player.score >= 21 {
            GameUpdate::Finished(count)
        } else {
            GameUpdate::Live(GameState(player1, player2, !game.2), count)
        }
    }
}

fn part2(input: &Input) -> usize {
    // Keep track of the number of ongoing games in each state
    let mut live_games = HashMap::new();
    let p1 = Player::new(input.p1);
    let p2 = Player::new(input.p2);
    live_games.increment(GameState(p1, p2, true), 1);

    // And the number of wins for each player
    let mut w1: usize = 0;
    let mut w2: usize = 0;

    while !live_games.is_empty() {
        // For each ongoing game state, we need to progress it `count` times
        for (game_state, count) in live_games.clone().iter() {
            live_games.decrement_delete(game_state.clone(), *count);

            // This game state results in 7 possible new game states, each with an additional count to reflect how likely it is
            for state in game_state.progress() {
                match state {
                    GameUpdate::Finished(new_count) => {
                        // If the game has ended, update the win counts
                        if game_state.2 {
                            w1 += count * new_count;
                        } else {
                            w2 += count * new_count;
                        }
                    }
                    GameUpdate::Live(game, new_count) => {
                        // Add this game back into the map of game states, with an increased count
                        live_games.increment(game, count * new_count);
                    }
                }
            }
        }
    }

    w1.max(w2)
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
    pub fn examples() {
        let input = Input::new(TEST_INPUT);
        assert_eq!(part1(&input), 739785);
        assert_eq!(part2(&input), 444356092776315);
    }

    #[test]
    pub fn answers() {
        let input = Input::new(INPUT);
        assert_eq!(part1(&input), 906093);
        assert_eq!(part2(&input), 274291038026362);
    }
}
