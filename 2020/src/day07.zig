const std = @import("std");
const print = std.log.info;
const expect = std.testing.expect;

const INPUT_FILE = @embedFile("inputs/day07.txt");
const OUR_BAG_COLOR = "shiny gold";

const Answer = usize;
const Adjacency = std.StringHashMap(Contents);
const Input = struct {
    allocator: std.mem.Allocator,
    map: Adjacency,

    fn deinit(self: *Input) void {
        var iterator = self.map.iterator();
        while (iterator.next()) |entry| {
            self.allocator.free(entry.value_ptr.*);
        }
        self.map.deinit();
    }

    fn getContents(self: Input, color: Color) []BagCount {
        return self.map.get(color).?;
    }

    fn contains(self: Input, color: Color, comptime target: Color) bool {
        for (self.getContents(color)) |bag| {
            if (bag.colorEquals(target) or self.contains(bag.color, target)) {
                return true;
            }
        }
        return false;
    }

    fn countContents(self: Input, color: Color) Answer {
        var count: Answer = 0;
        for (self.getContents(color)) |bag| {
            count += bag.count * (self.countContents(bag.color) + 1);
        }
        return count;
    }
};

const Contents = []BagCount;

const Color = []const u8;

const Rule = struct {
    color: Color,
    contains: Contents,
};

const BagCount = struct {
    color: Color,
    count: usize,

    fn colorEquals(self: BagCount, color: Color) bool {
        return std.mem.eql(u8, self.color, color);
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
    var result = Adjacency.init(allocator);
    errdefer result.deinit();

    var lines = std.mem.tokenize(u8, input, "\n");
    while (lines.next()) |line| {
        const rule = try decodeRule(allocator, line);
        try result.put(rule.color, rule.contains);
    }

    return Input{
        .allocator = allocator,
        .map = result,
    };
}

fn decodeRule(allocator: std.mem.Allocator, line: []const u8) !Rule {
    var contents = std.ArrayList(BagCount).init(allocator);
    errdefer contents.deinit();

    // muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
    var split_line = std.mem.split(u8, line, " bags contain ");
    const color = split_line.next().?; // muted yellow
    const remainder = split_line.next().?; // 2 shiny gold bags, 9 faded blue bags

    if (std.mem.eql(u8, remainder, "no other bags.")) {
        return Rule{
            .color = color,
            .contains = contents.toOwnedSlice(),
        };
    }

    var bag_colors = std.mem.split(u8, remainder, ", "); // 2 shiny gold bags
    while (bag_colors.next()) |color_and_count| {
        var split = std.mem.split(u8, color_and_count, " ");
        const count_string = split.next().?; // 2
        const count = try std.fmt.parseUnsigned(u8, count_string, 10);

        const bag_color = color_and_count[split.index.?..]; // shiny gold bags
        var color_split = std.mem.split(u8, bag_color, " bag");
        const child_color = color_split.next().?;

        const bag_count = BagCount{
            .color = child_color,
            .count = count,
        };
        try contents.append(bag_count);
    }

    return Rule{
        .color = color,
        .contains = contents.toOwnedSlice(),
    };
}

fn part1(input: Input) Answer {
    var count: Answer = 0;
    var iterator = input.map.iterator();
    // Iterate over every color. Key == color, Value == array of bags it contains
    while (iterator.next()) |rule| {
        if (input.contains(rule.key_ptr.*, OUR_BAG_COLOR)) {
            count += 1;
        }
    }
    return count;
}

fn part2(input: Input) Answer {
    return input.countContents(OUR_BAG_COLOR);
}

test "example" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/test_day07.txt");
    var input = try parseInput(alloc, test_input);
    defer input.deinit();

    try expect(part1(input) == 4);
    try expect(part2(input) == 32);
}

test "answers" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/day07.txt");
    var input = try parseInput(alloc, test_input);
    defer input.deinit();

    try expect(part1(input) == 226);
    try expect(part2(input) == 9569);
}
