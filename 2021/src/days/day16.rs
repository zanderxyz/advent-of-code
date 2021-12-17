use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/day16.txt");

#[derive(Clone)]
struct Input {
    pub bits: Vec<u8>,
}

trait Indexable {
    fn read(&self, start: usize, len: usize) -> usize;
}

impl Indexable for &[u8] {
    fn read(&self, start: usize, len: usize) -> usize {
        bits_to_int(&self[start..start + len])
    }
}

fn bits_to_int(bits: &[u8]) -> usize {
    let len = bits.len();
    bits.iter()
        .enumerate()
        .map(|(i, &b)| (b as usize) << (len - i - 1))
        .sum()
}

impl Input {
    pub fn new(input: &str) -> Input {
        let bits = input.trim().chars().flat_map(char_to_bits).collect();
        Input { bits }
    }
}

fn char_to_bits(c: char) -> [u8; 4] {
    // Read a single char from the hex input and convert to bits
    // We're storing the bits as u8s to make everything else far simpler
    let d: u8 = c.to_digit(16).unwrap().try_into().unwrap();
    [d >> 3, (d >> 2) & 1, (d >> 1) & 1, d & 1]
}

#[derive(Debug)]
enum Packet {
    // length, version, literal
    Literal(usize, usize, usize),
    // length, version, sub-packets, operator
    Operator(usize, usize, Vec<Self>, Op),
}

#[derive(Debug)]
enum Op {
    Sum,
    Product,
    Min,
    Max,
    Greater,
    Less,
    Eq,
}

impl Op {
    fn from(id: usize) -> Self {
        match id {
            0 => Self::Sum,
            1 => Self::Product,
            2 => Self::Min,
            3 => Self::Max,
            5 => Self::Greater,
            6 => Self::Less,
            7 => Self::Eq,
            _ => unreachable!(),
        }
    }

    fn apply(&self, packets: &[Packet]) -> usize {
        match self {
            Op::Sum => packets.iter().map(|p| p.value()).sum(),
            Op::Product => packets.iter().map(|p| p.value()).product(),
            Op::Min => packets.iter().map(|p| p.value()).min().unwrap(),
            Op::Max => packets.iter().map(|p| p.value()).max().unwrap(),
            Op::Greater => {
                if packets[0].value() > packets[1].value() {
                    1
                } else {
                    0
                }
            }
            Op::Less => {
                if packets[0].value() < packets[1].value() {
                    1
                } else {
                    0
                }
            }
            Op::Eq => {
                if packets[0].value() == packets[1].value() {
                    1
                } else {
                    0
                }
            }
        }
    }
}

impl Packet {
    fn len(&self) -> usize {
        match *self {
            Packet::Literal(len, _, _) => len,
            Packet::Operator(len, _, _, _) => len,
        }
    }

    // Version of this single packet
    fn version(&self) -> usize {
        match *self {
            Packet::Literal(_, version, _) => version,
            Packet::Operator(_, version, _, _) => version,
        }
    }

    fn child_versions(&self) -> usize {
        match self {
            Packet::Literal(_, _, _) => 0,
            Packet::Operator(_, _, sub, _) => sub.iter().map(|p| p.total_versions()).sum(),
        }
    }

    // Recursive sum of all versions
    fn total_versions(&self) -> usize {
        self.version() + self.child_versions()
    }

    // Value of this packet for part 2
    fn value(&self) -> usize {
        match self {
            Packet::Literal(_, _, value) => *value,
            Packet::Operator(_, _, subs, op) => op.apply(subs),
        }
    }
}

fn parse_packet(packet: &[u8]) -> Packet {
    let version = packet.read(0, 3);
    let id = packet.read(3, 3);

    match id {
        4 => parse_literal(packet, version),
        _ => parse_operator(packet, version, id),
    }
}

fn parse_literal(packet: &[u8], version: usize) -> Packet {
    let mut i: usize = 0;
    let mut literal: usize = 0;
    let number_of_sections = (0..)
        .map(|i| (i, packet.read(6 + i * 5, 1)))
        .find_or_first(|&(_, v)| v == 0)
        .unwrap()
        .0;

    while i <= number_of_sections {
        let value = packet.read(7 + i * 5, 4);
        literal += value << (4 * (number_of_sections - i));
        i += 1;
    }

    Packet::Literal(6 + (1 + number_of_sections) * 5, version, literal)
}

fn parse_operator(packet: &[u8], version: usize, id: usize) -> Packet {
    match packet.read(6, 1) {
        0 => parse_operator_total_length(packet, version, id),
        1 => parse_operator_num_packets(packet, version, id),
        _ => unreachable!(),
    }
}

fn parse_operator_total_length(packet: &[u8], version: usize, id: usize) -> Packet {
    let total_length_bits = packet.read(7, 15);
    let mut length_read: usize = 0;
    let mut sub_packets = Vec::new();
    while length_read < total_length_bits {
        let packet = parse_packet(&packet[length_read + 15 + 7..]);
        length_read += packet.len();
        sub_packets.push(packet);
    }
    Packet::Operator(7 + 15 + length_read, version, sub_packets, Op::from(id))
}

fn parse_operator_num_packets(packet: &[u8], version: usize, id: usize) -> Packet {
    let number_sub_packets = packet.read(7, 11);
    let mut length_read: usize = 0;
    let mut sub_packets = Vec::new();
    while sub_packets.len() < number_sub_packets {
        let packet = parse_packet(&packet[length_read + 11 + 7..]);
        length_read += packet.len();
        sub_packets.push(packet);
    }
    Packet::Operator(7 + 11 + length_read, version, sub_packets, Op::from(id))
}

fn part1(input: Input) -> Packet {
    parse_packet(&input.bits)
}

fn part2(packet: Packet) -> usize {
    packet.value()
}

pub fn main() {
    let input = Input::new(INPUT);
    let packet = part1(input);
    println!("Part 1: {}", packet.total_versions());
    let answer2 = part2(packet);
    println!("Part 2: {}", answer2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn examples() {
        let input = Input::new("8A004A801A8002F478");
        let packet = part1(input);
        assert_eq!(packet.total_versions(), 16);

        let packet2 = part1(Input::new("9C0141080250320F1802104A08"));
        assert_eq!(packet2.value(), 1);

        let packet3 = part1(Input::new("F600BC2D8F"));
        assert_eq!(packet3.value(), 0);
    }

    #[test]
    pub fn answers() {
        let input = Input::new(INPUT);
        let packet = part1(input);
        assert_eq!(packet.total_versions(), 949);
        assert_eq!(part2(packet), 1114600142730);
    }
}
