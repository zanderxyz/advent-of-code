use std::cmp::Ordering;

use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day13.txt");

#[derive(Clone, Debug)]
struct Input {
    pairs: Vec<(Packet, Packet)>,
}

impl Input {
    fn new(input: &str) -> Input {
        parser::parse_input(input)
    }
}

// Separate module to avoid polluting the namespace with nom functions
mod parser {
    use super::*;
    use nom::{
        bytes::complete::tag,
        character::complete::{line_ending, u8},
        combinator::{all_consuming, map},
        multi::{many0, separated_list0},
        sequence::{delimited, separated_pair, terminated},
        IResult, Parser,
    };

    pub(super) fn parse_input(i: &str) -> Input {
        Input {
            pairs: parse_pairs(i).unwrap().1,
        }
    }

    fn parse_pairs(i: &str) -> IResult<&str, Vec<(Packet, Packet)>> {
        all_consuming(many0(
            terminated(parse_packet_pair, line_ending).or(parse_packet_pair),
        ))(i)
    }

    fn parse_packet_pair(i: &str) -> IResult<&str, (Packet, Packet)> {
        terminated(
            separated_pair(parse_packet, line_ending, parse_packet),
            line_ending,
        )(i)
    }

    fn parse_packet(i: &str) -> IResult<&str, Packet> {
        map(parse_packet_items, Packet::new)(i)
    }

    fn parse_packet_number(i: &str) -> IResult<&str, PacketItem> {
        map(u8, PacketItem::Number)(i)
    }

    fn parse_packet_list(i: &str) -> IResult<&str, PacketItem> {
        map(parse_packet_items, PacketItem::List)(i)
    }

    fn parse_packet_items(i: &str) -> IResult<&str, Vec<PacketItem>> {
        delimited(
            tag("["),
            separated_list0(tag(","), parse_packet_number.or(parse_packet_list)),
            tag("]"),
        )(i)
    }
}

// This pair of structs defines a n-ary tree
#[derive(Clone, Debug)]
struct Packet {
    data: Vec<PacketItem>,
}

impl Packet {
    fn new(data: Vec<PacketItem>) -> Self {
        Self { data }
    }
}

#[derive(Clone, Debug)]
enum PacketItem {
    Number(u8),
    List(Vec<PacketItem>),
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.data.cmp(&other.data) == Ordering::Equal
    }
}

impl Eq for Packet {}

impl PartialEq for PacketItem {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for PacketItem {}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.data.partial_cmp(&other.data)
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for PacketItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (PacketItem::Number(x), PacketItem::Number(y)) => Some(x.cmp(y)),
            (PacketItem::Number(_), PacketItem::List(y)) => vec![self.clone()].partial_cmp(y),
            (PacketItem::List(x), PacketItem::Number(_)) => x.partial_cmp(&vec![other.clone()]),
            // The complex sorting rules for lists are actually the default
            (PacketItem::List(x), PacketItem::List(y)) => x.partial_cmp(y),
        }
    }
}

impl Ord for PacketItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn part1(input: &Input) -> usize {
    input
        .pairs
        .iter()
        .enumerate()
        .filter(|(_, (left, right))| left < right)
        .map(|(index, _)| index + 1)
        .sum()
}

fn part2(input: &Input) -> usize {
    let mut packets = input.pairs.iter().flat_map(|(l, r)| [l, r]).collect_vec();
    let divider_one = Packet {
        data: vec![PacketItem::List(vec![PacketItem::Number(2)])],
    };
    let divider_two = Packet {
        data: vec![PacketItem::List(vec![PacketItem::Number(6)])],
    };
    packets.push(&divider_one);
    packets.push(&divider_two);
    packets.sort();

    packets
        .into_iter()
        .enumerate()
        .filter(|(_, p)| **p == divider_one || **p == divider_two)
        // The indices should start at 1
        .map(|(i, _)| i + 1)
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
    const TEST_INPUT: &str = include_str!("../../inputs/test_day13.txt");

    #[test]
    fn examples() {
        let input = Input::new(TEST_INPUT);
        assert_eq!(part1(&input), 13);
        assert_eq!(part2(&input), 140);
    }

    #[test]
    fn answers() {
        let input = Input::new(INPUT);
        assert_eq!(part1(&input), 5252);
        assert_eq!(part2(&input), 20592);
    }
}
