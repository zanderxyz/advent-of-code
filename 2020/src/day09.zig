const std = @import("std");
const print = std.log.info;
const expect = std.testing.expect;

const INPUT_FILE = @embedFile("inputs/day09.txt");

const Answer = usize;
const Input = []Answer;

pub fn main() !void {
    var alloc = std.heap.GeneralPurposeAllocator(.{}){};

    const input = try parseInput(alloc.allocator(), INPUT_FILE);
    defer alloc.allocator().free(input);

    const p1answer = part1(input, 25);
    print("Part 1: {}\n", .{p1answer});
    print("Part 2: {}\n", .{part2(input, p1answer)});
}

fn parseInput(allocator: std.mem.Allocator, input: []const u8) !Input {
    var result = std.ArrayList(Answer).init(allocator);
    errdefer result.deinit();

    var instructions = std.mem.tokenize(u8, input, "\n");
    while (instructions.next()) |line| {
        const number = try std.fmt.parseInt(Answer, line, 10);

        try result.append(number);
    }

    return result.toOwnedSlice();
}

fn part1(input: Input, preambleSize: usize) Answer {
    var i: usize = 0;
    while (i < input.len) : (i += 1) {
        const slice = input[i .. i + preambleSize];
        const next = input[i + preambleSize];
        if (!isValid(slice, next)) {
            return next;
        }
    }
    unreachable;
}

fn part2(input: Input, target: Answer) Answer {
    var i: usize = 0;
    while (i < input.len) {
        var j: usize = 2;
        while (j < input.len) {
            const slice = input[i..j];
            const new_sum = sum(slice);
            if (new_sum == target) {
                const min = std.mem.min(Answer, slice);
                const max = std.mem.max(Answer, slice);
                return min + max;
            } else if (new_sum > target) {
                i += 1;
                j = i + 2;
            } else {
                j += 1;
            }
        }
    }
    unreachable;
}

fn sum(input: []const Answer) Answer {
    var result: Answer = 0;
    for (input) |item| {
        result += item;
    }
    return result;
}

fn isValid(numbers: []const usize, number: usize) bool {
    var i: usize = 0;
    while (i < numbers.len) : (i += 1) {
        var j: usize = 1;
        while (j < numbers.len) : (j += 1) {
            if (i == j) continue;

            if (numbers[i] + numbers[j] == number) {
                return true;
            }
        }
    }

    return false;
}

test "example" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/test_day09.txt");
    const input = try parseInput(alloc, test_input);
    defer alloc.free(input);

    const first = part1(input, 5);
    try expect(first == 127);
    try expect(part2(input, first) == 62);
}

test "answers" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/day09.txt");
    const input = try parseInput(alloc, test_input);
    defer alloc.free(input);

    const first = part1(input, 25);
    try expect(first == 1124361034);
    try expect(part2(input, first) == 129444555);
}

test "is valid" {
    const list: [5]usize = .{ 1, 2, 3, 4, 5 };
    try expect(isValid(list[0..5], 6) == true);
    try expect(isValid(list[0..5], 10) == false);
}
