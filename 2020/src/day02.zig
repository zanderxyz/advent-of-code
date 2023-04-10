const std = @import("std");
const print = std.log.info;
const expect = std.testing.expect;

const INPUT_FILE = @embedFile("inputs/day02.txt");

const Answer = i32;
const Input = []Password;

const Password = struct {
    password: []const u8,
    rule: PasswordRule,
};

const PasswordRule = struct {
    min: usize,
    max: usize,
    letter: u8,
};

pub fn main() !void {
    var alloc = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer alloc.deinit();

    const input = try parseInput(alloc.allocator(), INPUT_FILE);
    defer alloc.allocator().free(input);

    print("Part 1: {}\n", .{part1(input)});
    print("Part 2: {}\n", .{part2(input)});
}

fn parseInput(allocator: std.mem.Allocator, input: []const u8) !Input {
    var result = std.ArrayList(Password).init(allocator);
    errdefer result.deinit();

    var lines = std.mem.tokenize(u8, input, "\n");
    while (lines.next()) |line| {
        const value = try parseLine(line);
        try result.append(value);
    }

    return result.toOwnedSlice();
}

// Example:
// 2-9 c: ccccccccc
fn parseLine(line: []const u8) !Password {
    var parts = std.mem.tokenize(u8, line, ": ");
    const definition = parts.next().?;
    const letter = parts.next().?[0];
    const password = parts.next().?;

    var min_and_max = std.mem.tokenize(u8, definition, "-");
    const min = try std.fmt.parseInt(usize, min_and_max.next().?, 10);
    const max = try std.fmt.parseInt(usize, min_and_max.next().?, 10);

    const rule = PasswordRule{
        .min = min,
        .max = max,
        .letter = letter,
    };

    return Password{
        .password = password,
        .rule = rule,
    };
}

fn part1(input: Input) Answer {
    var valid: Answer = 0;
    for (input) |password| {
        var count: i32 = 0;
        for (password.password) |char| {
            if (char == password.rule.letter) {
                count += 1;
            }
        }
        if (password.rule.min <= count and count <= password.rule.max) {
            valid += 1;
        }
    }
    return valid;
}

fn part2(input: Input) Answer {
    var valid: Answer = 0;
    for (input) |password| {
        var count: usize = 0;
        if (password.password[password.rule.min - 1] == password.rule.letter) {
            count += 1;
        }
        if (password.password[password.rule.max - 1] == password.rule.letter) {
            count += 1;
        }
        if (count == 1) {
            valid += 1;
        }
    }
    return valid;
}

test "example" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/test_day02.txt");
    const input = try parseInput(alloc, test_input);
    defer alloc.free(input);

    try expect(part1(input) == 2);
    try expect(part2(input) == 1);
}

test "answers" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/day02.txt");
    const input = try parseInput(alloc, test_input);
    defer alloc.free(input);

    try expect(part1(input) == 519);
    try expect(part2(input) == 708);
}
