const std = @import("std");
const print = std.log.info;
const expect = std.testing.expect;

const INPUT_FILE = @embedFile("inputs/day23.txt");

const Cup = usize;
const Answer = usize;

const Input = struct {
    allocator: std.mem.Allocator,
    cups: []Cup,

    fn init(allocator: std.mem.Allocator, cups: []Cup) Input {
        return Input{
            .allocator = allocator,
            .cups = cups,
        };
    }

    fn deinit(self: Input) void {
        self.allocator.free(self.cups);
    }
};

fn Game(comptime MAX: usize) type {
    return struct {
        current: Cup,
        next: [MAX + 1]Cup,

        const Self = @This();

        fn init(cups: []Cup) !Self {
            var next: [MAX + 1]Cup = undefined;

            // Set up the cups provided
            var current = cups[0];
            for (cups[1..]) |cup| {
                next[current] = cup;
                current = cup;
            }

            // The rest get increasing numbers
            var i: usize = cups.len + 1;
            while (i <= MAX) : (i += 1) {
                next[current] = i;
                current = i;
            }

            // Link the final cup back to the first one
            next[current] = cups[0];

            return Self{
                .current = cups[0],
                .next = next,
            };
        }

        fn goBack(self: Self, from: usize) usize {
            if (self.current == 0) {
                return MAX;
            } else {
                if (from == 1) {
                    return MAX;
                } else {
                    return from - 1;
                }
            }
        }

        fn run(self: *Self) void {
            // Pick up the next three cups
            const a = self.next[self.current];
            const b = self.next[a];
            const c = self.next[b];
            self.next[self.current] = self.next[c];

            // Find the destination
            var destination = self.goBack(self.current);
            while (destination == a or destination == b or destination == c) {
                destination = self.goBack(destination);
            }

            // Put the cups back down right after the destination cup
            // Note: a already points to b, b already points to c
            // So we just have to update the following, in this order:
            // 1. c point to the cup after the destination
            // 2. The destination point to a
            self.next[c] = self.next[destination];
            self.next[destination] = a;

            // Rotate the current cup clockwise
            self.current = self.next[self.current];
        }

        fn runN(self: *Self, n: usize) void {
            var i: usize = 0;
            while (i < n) : (i += 1) {
                self.run();
            }
        }

        fn score(self: Self) usize {
            var mag: usize = 10000000;
            var output: usize = 0;
            var next: usize = self.next[1];
            while (next != 1) {
                output += mag * next;
                mag /= 10;
                next = self.next[next];
            }
            return output;
        }

        fn finalScore(self: Self) usize {
            var first = self.next[1];
            var second = self.next[first];
            return first * second;
        }
    };
}

pub fn main() !void {
    var alloc = std.heap.GeneralPurposeAllocator(.{}){};

    var input = try parseInput(alloc.allocator(), INPUT_FILE);
    defer input.deinit();

    print("Part 1: {}\n", .{part1(input)});
    print("Part 2: {}\n", .{part2(input)});
}

fn parseInput(allocator: std.mem.Allocator, input: []const u8) !Input {
    var cups = std.ArrayList(Cup).init(allocator);
    errdefer cups.deinit();

    var lines = std.mem.tokenize(u8, input, "\n");
    const line = lines.next().?;
    for (line) |char| {
        const str: [1]u8 = [_]u8{char};
        const number = try std.fmt.parseInt(u8, &str, 10);

        try cups.append(number);
    }

    return Input.init(allocator, cups.toOwnedSlice());
}

fn part1(input: Input) Answer {
    var game = Game(9).init(input.cups) catch unreachable;
    game.runN(100);
    return game.score();
}

fn part2(input: Input) Answer {
    // Requires a large stack size, may need to `ulimit -s hard` first
    var game = Game(1_000_000).init(input.cups) catch unreachable;
    game.runN(10_000_000);
    return game.finalScore();
}

test "examples" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/test_day23.txt");
    var input = try parseInput(alloc, test_input);
    defer input.deinit();

    try expect(part1(input) == 67384529);
    try expect(part2(input) == 149245887792);
}

test "answers" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/day23.txt");
    var input = try parseInput(alloc, test_input);
    defer input.deinit();

    try expect(part1(input) == 69425837);
    try expect(part2(input) == 218882971435);
}
