const std = @import("std");
const print = std.log.info;
const expect = std.testing.expect;

const INPUT_FILE = @embedFile("inputs/day15.txt");

const Answer = usize;
const LastSeen = std.AutoHashMap(Answer, Answer);
const Input = struct {
    allocator: std.mem.Allocator,
    numbers: []Answer,

    fn deinit(self: *Input) void {
        defer self.allocator.free(self.numbers);
    }
};

pub fn main() !void {
    var alloc = std.heap.GeneralPurposeAllocator(.{}){};

    var input = try parseInput(alloc.allocator(), INPUT_FILE);
    defer input.deinit();

    print("Part 1: {}\n", .{part1(input)});
    print("Part 2: {}\n", .{part2(input)});
}

fn parseInput(allocator: std.mem.Allocator, input: []const u8) !Input {
    var result = std.ArrayList(Answer).init(allocator);
    errdefer result.deinit();

    var numbers = std.mem.split(u8, input, ",");
    while (numbers.next()) |num| {
        const number = try std.fmt.parseInt(Answer, num, 10);
        try result.append(number);
    }

    return Input{
        .allocator = allocator,
        .numbers = result.toOwnedSlice(),
    };
}

fn run(comptime max: usize, input: Input) Answer {
    var last_seen = LastSeen.init(input.allocator);
    defer last_seen.deinit();

    var i: usize = 0;
    var number: Answer = undefined;
    var last_seen_on_turn: ?Answer = undefined;

    while (i < max) : (i += 1) {
        if (i < input.numbers.len) {
            number = input.numbers[i];
        } else {
            if (last_seen_on_turn) |last| {
                number = i - last;
            } else {
                number = 0;
            }
        }

        last_seen_on_turn = last_seen.get(number);
        last_seen.put(number, i + 1) catch unreachable;
    }

    return number;
}

fn part1(input: Input) Answer {
    return run(2020, input);
}

fn part2(input: Input) Answer {
    return run(30000000, input);
}

test "examples" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/test_day15.txt");
    var input = try parseInput(alloc, test_input);
    defer input.deinit();

    try expect(part1(input) == 436);
    try expect(part2(input) == 175594);
}

test "answers" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/day15.txt");
    var input = try parseInput(alloc, test_input);
    defer input.deinit();

    try expect(part1(input) == 700);
    try expect(part2(input) == 51358);
}
