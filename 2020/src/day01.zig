const std = @import("std");
const print = std.log.info;
const expect = std.testing.expect;

const INPUT_FILE = @embedFile("inputs/day01.txt");
const TARGET_SUM = 2020;

const Answer = usize;
const Input = []Answer;

pub fn main() !void {
    var alloc = std.heap.GeneralPurposeAllocator(.{}){};

    const input = try parseInput(alloc.allocator(), INPUT_FILE);
    defer alloc.allocator().free(input);

    print("Part 1: {}\n", .{part1(input)});
    print("Part 2: {}\n", .{part2(input)});
}

fn parseInput(allocator: std.mem.Allocator, input: []const u8) !Input {
    var result = std.ArrayList(Answer).init(allocator);
    errdefer result.deinit();

    var lines = std.mem.tokenize(u8, input, "\n");
    while (lines.next()) |line| {
        const value = try std.fmt.parseInt(Answer, line, 10);
        try result.append(value);
    }

    // Sort the items in place, ascending order
    std.sort.sort(Answer, result.items, {}, comptime std.sort.asc(Answer));

    return result.toOwnedSlice();
}

fn part1(input: Input) Answer {
    var i: usize = 0;
    var j: usize = input.len - 1;
    while (i < j) {
        const left = input[i];
        const right = input[j];
        const sum = left + right;

        if (sum == TARGET_SUM) {
            return left * right;
        } else if (sum > TARGET_SUM) {
            j -= 1;
        } else if (sum < TARGET_SUM) {
            i += 1;
        }
    }
    unreachable;
}

fn part2(input: Input) Answer {
    for (input) |current, k| {
        var i: usize = k;
        var j: usize = input.len - 1;
        while (i < j) {
            const left = input[i];
            const right = input[j];
            const sum = left + right + current;

            if (sum == TARGET_SUM) {
                return left * right * current;
            } else if (sum > TARGET_SUM) {
                j -= 1;
            } else if (sum < TARGET_SUM) {
                i += 1;
            }
        }
    }
    unreachable;
}

test "example" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/test_day01.txt");
    const input = try parseInput(alloc, test_input);
    defer alloc.free(input);

    try expect(part1(input) == 514579);
    try expect(part2(input) == 241861950);
}

test "answers" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/day01.txt");
    const input = try parseInput(alloc, test_input);
    defer alloc.free(input);

    try expect(part1(input) == 987339);
    try expect(part2(input) == 259521570);
}
