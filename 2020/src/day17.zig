const std = @import("std");
const print = std.log.info;
const expect = std.testing.expect;

const INPUT_FILE = @embedFile("inputs/day17.txt");

const Answer = usize;

fn Position(comptime D: usize) type {
    return [D]isize;
}

fn Cells(comptime D: usize) type {
    return std.AutoHashMap(Position(D), void);
}

fn Space(comptime D: usize) type {
    return struct {
        allocator: std.mem.Allocator,
        cells: Cells(D),

        const Self = @This();

        fn init(input: Input) !Self {
            var cells = Cells(D).init(input.allocator);

            for (input.initial) |row, i| {
                for (row) |active, j| {
                    if (active) {
                        // Add the new position
                        var position: Position(D) = [_]isize{0} ** D;
                        position[0] = @intCast(isize, i);
                        position[1] = @intCast(isize, j);

                        try cells.put(position, {});
                    }
                }
            }

            return Self{
                .allocator = input.allocator,
                .cells = cells,
            };
        }

        fn deinit(self: *Self) void {
            defer self.cells.deinit();
        }

        fn updateCell(next: *Cells(D), position: Position(D), active: bool, neighbours: usize) !void {
            if (active) {
                // We use 3/4 instead of 2/3 as we counted the cell itself in neighbours
                if (neighbours == 3 or neighbours == 4) {
                    // Stays active
                    try next.put(position, {});
                } else {
                    // Moves to inactive
                }
            } else {
                if (neighbours == 3) {
                    // Moves to active
                    try next.put(position, {});
                } else {
                    // Stays inactive
                }
            }
        }

        fn countNeighbours(self: Self, position: Position(D)) usize {
            const NeighbourCounter = struct {
                cells: Cells(D),
                neighbours: usize = 0,

                fn apply(s: *@This(), neighbour: Position(D)) void {
                    if (s.cells.contains(neighbour)) {
                        s.neighbours += 1;
                    }
                }
            };

            var counter = NeighbourCounter{
                .cells = self.cells,
            };

            forEachNeighbour(position, &counter);

            return counter.neighbours;
        }

        fn forEachNeighbour(position: Position(D), func: anytype) void {
            // 2d has 3^2== 9 neighbours including itself
            // 3d has 3^3 == 27 neighbours including itself
            comptime var max: usize = std.math.pow(usize, 3, D);
            comptime var i: usize = 0;

            inline while (i < max) : (i += 1) {
                // Create a copy of the central position, and offset it to give a neighbour
                var neighbour: Position(D) = [_]isize{0} ** D;

                comptime var d: usize = 0;
                inline while (d < D) : (d += 1) {
                    // Gives us every offset combination
                    const offset = (i / std.math.pow(usize, 3, d)) % 3;

                    // Subtract one as the range above is 0 to 2 and we want -1 to 1
                    neighbour[d] = position[d] + @intCast(isize, offset) - 1;
                }

                // Do something for each neighbour
                func.apply(neighbour);
            }
        }

        fn getToVisit(self: *Self) !Cells(D) {
            // Calculate the set of all cells we need to check this round
            var to_visit = Cells(D).init(self.allocator);
            errdefer to_visit.deinit();

            var iterator = self.cells.iterator();
            while (iterator.next()) |entry| {
                const position = entry.key_ptr.*;

                // Add every neighbour including the cell itself
                const NeighbourVisit = struct {
                    to_visit: Cells(D),

                    fn apply(s: *@This(), neighbour: Position(D)) void {
                        s.to_visit.put(neighbour, {}) catch unreachable;
                    }
                };

                var visit = NeighbourVisit{
                    .to_visit = to_visit,
                };

                forEachNeighbour(position, &visit);

                to_visit = visit.to_visit;
            }

            return to_visit;
        }

        fn step(self: *Self) !void {
            // Calculate the set of all cells we need to check this round
            var to_visit = try self.getToVisit();
            defer to_visit.deinit();

            // Visit them and apply the update logic
            var next = Cells(D).init(self.allocator);
            var visit = to_visit.iterator();
            while (visit.next()) |entry| {
                const position = entry.key_ptr.*;
                const active = self.cells.contains(position);
                const neighbours = self.countNeighbours(position);
                try updateCell(&next, position, active, neighbours);
            }

            // Finally copy the next state into the current one
            self.cells.deinit();
            self.cells = next;
        }

        fn stepN(self: *Self, n: usize) !void {
            var i: usize = 0;
            while (i < n) : (i += 1) {
                try self.step();
            }
        }

        fn countCells(self: Self) usize {
            return self.cells.count();
        }
    };
}

const Input = struct {
    allocator: std.mem.Allocator,
    initial: [][]bool,

    fn init(allocator: std.mem.Allocator, initial: [][]bool) Input {
        return Input{
            .allocator = allocator,
            .initial = initial,
        };
    }

    fn deinit(self: Input) void {
        for (self.initial) |row| {
            self.allocator.free(row);
        }
        self.allocator.free(self.initial);
    }
};

pub fn main() !void {
    var alloc = std.heap.GeneralPurposeAllocator(.{}){};

    var input = try parseInput(alloc.allocator(), INPUT_FILE);
    defer input.deinit();

    print("Part 1: {!}\n", .{part1(input)});
    print("Part 2: {!}\n", .{part2(input)});
}

fn parseInput(allocator: std.mem.Allocator, input: []const u8) !Input {
    var initial = std.ArrayList([]bool).init(allocator);
    errdefer initial.deinit();

    var lines = std.mem.tokenize(u8, input, "\n");
    while (lines.next()) |line| {
        var row = std.ArrayList(bool).init(allocator);
        errdefer row.deinit();

        for (line) |char| {
            const cell: bool = switch (char) {
                '.' => false,
                '#' => true,
                else => unreachable,
            };

            try row.append(cell);
        }

        try initial.append(row.toOwnedSlice());
    }

    return Input.init(allocator, initial.toOwnedSlice());
}

fn part1(input: Input) !Answer {
    var space = try Space(3).init(input);
    defer space.deinit();

    try space.stepN(6);
    return space.countCells();
}

fn part2(input: Input) !Answer {
    var space = try Space(4).init(input);
    defer space.deinit();

    try space.stepN(6);
    return space.countCells();
}

test "examples" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/test_day17.txt");
    var input = try parseInput(alloc, test_input);
    defer input.deinit();

    try expect((try part1(input)) == 112);
    try expect((try part2(input)) == 848);
}

test "examples" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/day17.txt");
    var input = try parseInput(alloc, test_input);
    defer input.deinit();

    try expect((try part1(input)) == 223);
    try expect((try part2(input)) == 1884);
}
