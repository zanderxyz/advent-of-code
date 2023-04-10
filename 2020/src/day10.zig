const std = @import("std");
const print = std.log.info;
const expect = std.testing.expect;

const INPUT_FILE = @embedFile("inputs/day10.txt");

const Answer = usize;
const Input = struct {
    allocator: std.mem.Allocator,
    items: []Answer,

    fn deinit(self: Input) void {
        self.allocator.free(self.items);
    }
};

pub fn main() !void {
    var alloc = std.heap.GeneralPurposeAllocator(.{}){};

    const input = try parseInput(alloc.allocator(), INPUT_FILE);
    defer input.deinit();

    print("Part 1: {}\n", .{part1(input)});
    print("Part 2: {}\n", .{part2(input)});
}

fn parseInput(allocator: std.mem.Allocator, input: []const u8) !Input {
    var result = std.ArrayList(Answer).init(allocator);
    errdefer result.deinit();

    var instructions = std.mem.tokenize(u8, input, "\n");
    while (instructions.next()) |line| {
        const number = try std.fmt.parseInt(Answer, line, 10);

        try result.append(number);
    }

    // Sort the items in place, ascending order
    std.sort.sort(Answer, result.items, {}, comptime std.sort.asc(Answer));

    return Input{
        .allocator = allocator,
        .items = result.toOwnedSlice(),
    };
}

fn part1(input: Input) Answer {
    var differences1: Answer = 0;
    var differences3: Answer = 1;
    var joltage: Answer = 0;
    for (input.items) |current| {
        const acc = current - joltage;
        switch (acc) {
            1 => {
                differences1 += 1;
            },
            2 => {},
            3 => {
                differences3 += 1;
            },
            else => unreachable,
        }
        joltage = current;
    }
    return differences1 * differences3;
}

fn part2(input: Input) Answer {
    var length = input.items.len;

    var paths = std.ArrayList(Answer).init(input.allocator);
    paths.ensureTotalCapacity(length) catch unreachable;
    defer paths.deinit();

    var k: usize = 0;
    while (k < length) : (k += 1) {
        paths.append(0) catch unreachable;
    }

    // Initially we can reach up to the first 3 items, if they have joltage <= 3
    if (input.items[0] <= 3) paths.items[0] = 1;
    if (input.items[1] <= 3) paths.items[1] = 1;
    if (input.items[2] <= 3) paths.items[2] = 1;

    for (input.items) |adaptor, i| {
        var j: usize = 1;
        while (i + j < length) : (j += 1) {
            if (j > 3) break;
            if (input.items[i + j] - adaptor <= 3) {
                // We can reach this new item from our current one, add our current paths count to it
                paths.items[i + j] += paths.items[i];
            }
        }
    }

    // Number of paths to the final item
    return paths.items[length - 1];
}

test "example" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/test_day10.txt");
    const input = try parseInput(alloc, test_input);
    defer input.deinit();

    try expect(part1(input) == 35);
    try expect(part2(input) == 8);
}

test "example 2" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/test_day10_2.txt");
    const input = try parseInput(alloc, test_input);
    defer input.deinit();

    try expect(part1(input) == 220);
    try expect(part2(input) == 19208);
}

test "answers" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/day10.txt");
    const input = try parseInput(alloc, test_input);
    defer input.deinit();

    try expect(part1(input) == 2046);
    try expect(part2(input) == 1157018619904);
}
