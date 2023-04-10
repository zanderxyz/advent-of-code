const std = @import("std");
const print = std.log.info;
const expect = std.testing.expect;

const INPUT_FILE = @embedFile("inputs/day11.txt");

const Answer = usize;
const Input = Grid;

const Space = enum {
    floor,
    empty,
    full,
};

const Position = struct {
    x: isize,
    y: isize,

    fn new(x: isize, y: isize) Position {
        return Position{
            .x = x,
            .y = y,
        };
    }
};

const Spaces = std.AutoHashMap(Position, Space);

const Grid = struct {
    allocator: std.mem.Allocator,
    height: usize,
    width: usize,
    current: Spaces,
    next: Spaces,

    fn init(allocator: std.mem.Allocator) !Grid {
        return Grid{
            .allocator = allocator,
            .height = 0,
            .width = 0,
            .current = Spaces.init(allocator),
            .next = Spaces.init(allocator),
        };
    }

    fn deinit(self: *Grid) void {
        self.current.deinit();
        self.next.deinit();
    }

    fn addRow(self: *Grid, line: []const u8) void {
        if (self.width == 0) {
            self.width = line.len;
        }
        if (self.width != line.len) {
            unreachable;
        }

        for (line) |char, i| {
            const seat: Space = switch (char) {
                '.' => .floor,
                'L' => .empty,
                else => unreachable,
            };

            const position = Position{
                .x = @intCast(isize, i),
                .y = @intCast(isize, self.height),
            };

            self.current.put(position, seat) catch unreachable;
            self.next.put(position, seat) catch unreachable;
        }

        self.height += 1;
    }

    fn copyNewToOld(self: *Grid) void {
        var iterator = self.next.iterator();
        while (iterator.next()) |entry| {
            const position = entry.key_ptr.*;
            const seat = entry.value_ptr.*;

            self.current.put(position, seat) catch unreachable;
        }
    }

    fn countFull(self: *Grid) usize {
        var count: usize = 0;
        var iterator = self.current.iterator();
        while (iterator.next()) |entry| {
            const seat = entry.value_ptr.*;

            if (seat == .full) {
                count += 1;
            }
        }
        return count;
    }

    fn getOrNull(self: Grid, position: Position) ?Space {
        return self.current.get(position);
    }

    fn get(self: Grid, position: Position) Space {
        return self.getOrNull(position) orelse .floor;
    }

    fn shouldUpdateAdjacent(self: Grid, position: Position) bool {
        return switch (self.get(position)) {
            .floor => false,
            .empty => rule: {
                // If all adjacent spaces are not taken, fill the empty seat
                if (self.countAdjacentFull(position) == 0) {
                    break :rule true;
                } else {
                    break :rule false;
                }
            },
            .full => rule: {
                // If >=4 adjacent spaces are full, empty the seat
                if (self.countAdjacentFull(position) >= 4) {
                    break :rule true;
                } else {
                    break :rule false;
                }
            },
        };
    }

    fn countAdjacentFull(self: Grid, position: Position) usize {
        var count: usize = 0;
        const x = position.x;
        const y = position.y;

        inline for (.{ -1, 0, 1 }) |dx| {
            inline for (.{ -1, 0, 1 }) |dy| {
                if (dx != 0 or dy != 0) {
                    const x2 = if (x == -1) x - 1 else x + dx;
                    const y2 = if (y == -1) y - 1 else y + dy;
                    const position2 = Position.new(x2, y2);
                    if (self.get(position2) == .full) {
                        count += 1;
                    }
                }
            }
        }

        return count;
    }

    fn shouldUpdateVisible(self: Grid, position: Position) bool {
        return switch (self.get(position)) {
            .floor => false,
            .empty => rule: {
                // If all visible spaces are not taken, fill the empty seat
                if (self.countVisibleFull(position) == 0) {
                    break :rule true;
                } else {
                    break :rule false;
                }
            },
            .full => rule: {
                // If >=5 visible spaces are full, empty the seat
                if (self.countVisibleFull(position) >= 5) {
                    break :rule true;
                } else {
                    break :rule false;
                }
            },
        };
    }

    fn countVisibleFull(self: Grid, position: Position) usize {
        var count: usize = 0;
        const x = position.x;
        const y = position.y;

        inline for (.{ -1, 0, 1 }) |dx| {
            inline for (.{ -1, 0, 1 }) |dy| {
                if (dx != 0 or dy != 0) {
                    var x2 = if (dx == -1) x - 1 else x + dx;
                    var y2 = if (dy == -1) y - 1 else y + dy;
                    var position2 = Position.new(x2, y2);
                    while (self.getOrNull(position2)) |seat| {
                        x2 = if (dx == -1) x2 - 1 else x2 + dx;
                        y2 = if (dy == -1) y2 - 1 else y2 + dy;
                        if (seat == .full) count += 1;
                        if (seat != .floor) break;
                        position2 = Position.new(x2, y2);
                    }
                }
            }
        }

        return count;
    }
};

pub fn main() !void {
    var alloc = std.heap.GeneralPurposeAllocator(.{}){};

    var input = try parseInput(alloc.allocator(), INPUT_FILE);
    defer input.deinit();
    print("Part 1: {}\n", .{part1(&input)});

    var input2 = try parseInput(alloc.allocator(), INPUT_FILE);
    defer input2.deinit();
    print("Part 2: {}\n", .{part2(&input2)});
}

fn parseInput(allocator: std.mem.Allocator, input: []const u8) !Input {
    var result = try Grid.init(allocator);
    errdefer result.deinit();

    var instructions = std.mem.tokenize(u8, input, "\n");
    while (instructions.next()) |line| {
        result.addRow(line);
    }

    return result;
}

fn part1(grid: *Input) Answer {
    var changed: bool = true;
    while (changed) {
        changed = false;

        var iterator = grid.current.iterator();
        while (iterator.next()) |entry| {
            const position = entry.key_ptr.*;
            if (grid.shouldUpdateAdjacent(position)) {
                changed = true;
                switch (entry.value_ptr.*) {
                    .floor => {},
                    .empty => {
                        grid.next.put(position, .full) catch unreachable;
                    },
                    .full => {
                        grid.next.put(position, .empty) catch unreachable;
                    },
                }
            }
        }

        grid.copyNewToOld();
    }

    return grid.countFull();
}

fn part2(grid: *Input) Answer {
    var changed: bool = true;
    while (changed) {
        changed = false;

        var iterator = grid.current.iterator();
        while (iterator.next()) |entry| {
            const position = entry.key_ptr.*;
            if (grid.shouldUpdateVisible(position)) {
                changed = true;
                switch (entry.value_ptr.*) {
                    .floor => {},
                    .empty => {
                        grid.next.put(position, .full) catch unreachable;
                    },
                    .full => {
                        grid.next.put(position, .empty) catch unreachable;
                    },
                }
            }
        }

        grid.copyNewToOld();
    }

    return grid.countFull();
}

test "example" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/test_day11.txt");
    var input = try parseInput(alloc, test_input);
    defer input.deinit();
    try expect(part1(&input) == 37);

    var input2 = try parseInput(alloc, test_input);
    defer input2.deinit();
    try expect(part2(&input2) == 26);
}

test "answers" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/day11.txt");
    var input = try parseInput(alloc, test_input);
    defer input.deinit();
    try expect(part1(&input) == 2361);

    var input2 = try parseInput(alloc, test_input);
    defer input2.deinit();
    try expect(part2(&input2) == 2119);
}
