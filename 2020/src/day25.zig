const std = @import("std");
const print = std.log.info;
const expect = std.testing.expect;

const INPUT_FILE = @embedFile("inputs/day25.txt");

const Answer = usize;

const Input = struct {
    door_public_key: usize,
    card_public_key: usize,
};

pub fn main() !void {
    var input = try parseInput(INPUT_FILE);

    print("Part 1: {}\n", .{part1(input)});
}

fn parseInput(input: []const u8) !Input {
    var lines = std.mem.tokenize(u8, input, "\n");

    const card_public_key = try std.fmt.parseInt(usize, lines.next().?, 10);
    const door_public_key = try std.fmt.parseInt(usize, lines.next().?, 10);

    return Input{
        .door_public_key = door_public_key,
        .card_public_key = card_public_key,
    };
}

fn transform(subject: usize, loop: usize) usize {
    var target: usize = 1;
    var i: usize = 0;
    while (i < loop) : (i += 1) {
        target = transformOnce(subject, target);
    }
    return target;
}

fn transformOnce(subject: usize, start: usize) usize {
    var target: usize = start;
    target *= subject;
    target %= 20201227;
    return target;
}

fn crack(subject: usize, public: usize) usize {
    var i: usize = 1;
    var guess = transformOnce(subject, 1);
    while (guess != public) {
        guess = transformOnce(subject, guess);
        i += 1;
    }
    return i;
}

const SUBJECT = 7;

test "transform" {
    try expect(transform(SUBJECT, 8) == 5764801);
}

test "crack" {
    try expect(crack(SUBJECT, 5764801) == 8);
}

fn part1(input: Input) Answer {
    const door_loop = crack(SUBJECT, input.door_public_key);
    const card_loop = crack(SUBJECT, input.card_public_key);

    const door_encryption = transform(input.door_public_key, card_loop);
    const card_encryption = transform(input.card_public_key, door_loop);

    std.debug.assert(door_encryption == card_encryption);

    return door_encryption;
}

test "examples" {
    const test_input = @embedFile("inputs/test_day25.txt");
    var input = try parseInput(test_input);

    try expect(part1(input) == 14897079);
}

test "answers" {
    const test_input = @embedFile("inputs/day25.txt");
    var input = try parseInput(test_input);

    try expect(part1(input) == 9177528);
}
