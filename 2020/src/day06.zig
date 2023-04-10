const std = @import("std");
const print = std.log.info;
const expect = std.testing.expect;

const INPUT_FILE = @embedFile("inputs/day06.txt");

const Answer = usize;
const Set = std.AutoHashMap(u8, void);
const SetCount = std.AutoHashMap(u8, usize);
const Input = struct {
    allocator: std.mem.Allocator,
    items: [][][]const u8,

    fn deinit(self: *@This()) void {
        for (self.items) |group| {
            self.allocator.free(group);
        }
        self.allocator.free(self.items);
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
    var result = std.ArrayList([][]const u8).init(allocator);
    errdefer result.deinit();

    var groups = std.mem.split(u8, input, "\n\n");
    while (groups.next()) |group| {
        var groupList = std.ArrayList([]const u8).init(allocator);
        errdefer groupList.deinit();

        var people = std.mem.tokenize(u8, group, "\n");
        while (people.next()) |person| {
            try groupList.append(person);
        }
        try result.append(groupList.toOwnedSlice());
    }

    return Input{
        .allocator = allocator,
        .items = result.toOwnedSlice(),
    };
}

fn part1(input: Input) Answer {
    var count: Answer = 0;
    for (input.items) |group| {
        var set = Set.init(input.allocator);
        defer set.deinit();

        for (group) |person| {
            for (person) |c| {
                set.put(c, {}) catch unreachable;
            }
        }

        // The answers at least one person gave
        count += set.count();
    }
    return count;
}

fn part2(input: Input) Answer {
    var count: Answer = 0;
    for (input.items) |group| {
        var set = SetCount.init(input.allocator);
        defer set.deinit();

        var group_len: usize = 0;
        for (group) |person| {
            for (person) |c| {
                const current = set.get(c) orelse 0;
                set.put(c, current + 1) catch unreachable;
            }
            group_len += 1;
        }

        var iterator = set.iterator();
        while (iterator.next()) |entry| {
            // The answers everyone gave
            if (entry.value_ptr.* == group_len) {
                count += 1;
            }
        }
    }

    return count;
}

test "example" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/test_day06.txt");
    var input = try parseInput(alloc, test_input);
    defer input.deinit();

    try expect(part1(input) == 11);
    try expect(part2(input) == 6);
}

test "answers" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/day06.txt");
    var input = try parseInput(alloc, test_input);
    defer input.deinit();

    try expect(part1(input) == 7283);
    try expect(part2(input) == 3520);
}
