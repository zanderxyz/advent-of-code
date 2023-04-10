const std = @import("std");
const print = std.log.info;
const expect = std.testing.expect;

const INPUT_FILE = @embedFile("inputs/day24.txt");

const Answer = usize;

const Step = enum {
    e,
    se,
    sw,
    w,
    nw,
    ne,

    fn fromString(string: []const u8) ?Step {
        return std.meta.stringToEnum(Step, string);
    }

    fn offset(step: Step) Position {
        return switch (step) {
            .w => [_]isize{ -1, 0 },
            .e => [_]isize{ 1, 0 },
            .sw => [_]isize{ -1, 1 },
            .se => [_]isize{ 0, 1 },
            .nw => [_]isize{ 0, -1 },
            .ne => [_]isize{ 1, -1 },
        };
    }

    fn move(step: Step, position: *Position) void {
        const movement = step.offset();
        position[0] += movement[0];
        position[1] += movement[1];
    }
};

const Path = []Step;

const Input = struct {
    allocator: std.mem.Allocator,
    paths: []Path,

    fn init(allocator: std.mem.Allocator, paths: []Path) Input {
        return Input{
            .allocator = allocator,
            .paths = paths,
        };
    }

    fn deinit(self: Input) void {
        for (self.paths) |ix| {
            self.allocator.free(ix);
        }
        self.allocator.free(self.paths);
    }
};

const Position = [2]isize;

const HexGrid = struct {
    allocator: std.mem.Allocator,
    cells: Cells,

    const Self = @This();
    const Cells = std.AutoHashMap(Position, void);

    const all_directions: [6]Step = [_]Step{ Step.e, Step.w, Step.se, Step.sw, Step.ne, Step.nw };

    fn init(allocator: std.mem.Allocator) Self {
        var cells = Cells.init(allocator);
        return Self{
            .allocator = allocator,
            .cells = cells,
        };
    }

    fn deinit(self: *Self) void {
        self.cells.deinit();
    }

    fn addPath(self: *Self, path: Path) void {
        var current: Position = [_]isize{ 0, 0 };
        for (path) |step| {
            step.move(&current);
        }
        self.flip(current);
    }

    fn flip(self: *Self, position: Position) void {
        if (self.cells.contains(position)) {
            _ = self.cells.remove(position);
        } else {
            self.cells.put(position, {}) catch unreachable;
        }
    }

    fn countNeighbours(self: Self, position: Position) usize {
        var c: usize = 0;
        for (all_directions) |dir| {
            var neighbour = position;
            dir.move(&neighbour);

            if (self.cells.contains(neighbour)) {
                c += 1;
            }
        }
        return c;
    }

    fn updateCell(next: *Cells, position: Position, active: bool, neighbours: usize) !void {
        if (active) {
            if (neighbours == 0 or neighbours > 2) {
                // Moves to inactive
            } else {
                // Stays active
                try next.put(position, {});
            }
        } else {
            if (neighbours == 2) {
                // Moves to active
                try next.put(position, {});
            } else {
                // Stays inactive
            }
        }
    }

    fn getToVisit(self: *Self) !Cells {
        var to_visit = Cells.init(self.allocator);
        errdefer to_visit.deinit();

        var iterator = self.cells.iterator();
        while (iterator.next()) |entry| {
            const position = entry.key_ptr.*;
            // Add every current active cell
            try to_visit.put(position, {});

            // And add every neighbour
            for (all_directions) |dir| {
                var neighbour = position;
                dir.move(&neighbour);

                try to_visit.put(neighbour, {});
            }
        }

        return to_visit;
    }

    fn tick(self: *Self) !void {
        // Calculate the set of all cells we need to check this round
        var to_visit = try self.getToVisit();
        defer to_visit.deinit();

        // Visit them all and apply the update logic
        var next = Cells.init(self.allocator);
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

    fn tickN(self: *Self, n: usize) !void {
        var i: usize = 0;
        while (i < n) : (i += 1) {
            try self.tick();
        }
    }

    fn count(self: Self) usize {
        return self.cells.count();
    }
};

test "hex grid" {
    var alloc = std.testing.allocator;
    var grid = HexGrid.init(alloc);
    defer grid.deinit();

    const position: Position = [_]isize{ 0, 0 };

    try expect(grid.count() == 0);
    try expect(grid.countNeighbours(position) == 0);

    // Flip one cell to the east
    var path: [1]Step = [_]Step{.e};
    grid.addPath(&path);
    try expect(grid.count() == 1);
    try expect(grid.countNeighbours(position) == 1);

    // Flip the origin
    var path2: [2]Step = [_]Step{ .w, .e };
    grid.addPath(&path2);
    try expect(grid.count() == 2);
    try expect(grid.countNeighbours(position) == 1);

    // Flip the origin back
    var path3: [3]Step = [_]Step{ .ne, .w, .se };
    grid.addPath(&path3);
    try expect(grid.count() == 1);
    try expect(grid.countNeighbours(position) == 1);

    var path4: [1]Step = [_]Step{.w};
    grid.addPath(&path4);
    try expect(grid.count() == 2);
    try expect(grid.countNeighbours(position) == 2);

    var path5: [1]Step = [_]Step{.sw};
    grid.addPath(&path5);
    try expect(grid.count() == 3);
    try expect(grid.countNeighbours(position) == 3);
}

pub fn main() !void {
    var alloc = std.heap.GeneralPurposeAllocator(.{}){};

    var input = try parseInput(alloc.allocator(), INPUT_FILE);
    defer input.deinit();

    var grid = HexGrid.init(input.allocator);
    defer grid.deinit();

    print("Part 1: {}\n", .{part1(&grid, &input)});
    print("Part 2: {}\n", .{part2(&grid)});
}

fn parseInput(allocator: std.mem.Allocator, input: []const u8) !Input {
    var paths = std.ArrayList(Path).init(allocator);
    errdefer paths.deinit();

    var lines = std.mem.tokenize(u8, input, "\n");
    while (lines.next()) |line| {
        var steps = std.ArrayList(Step).init(allocator);
        errdefer steps.deinit();

        var i: usize = 0;
        while (i < line.len) {
            if (Step.fromString(line[i .. i + 1])) |step| {
                try steps.append(step);
                i += 1;
            } else if (Step.fromString(line[i .. i + 2])) |step| {
                try steps.append(step);
                i += 2;
            } else {
                unreachable;
            }
        }

        try paths.append(steps.toOwnedSlice());
    }

    return Input.init(allocator, paths.toOwnedSlice());
}

fn part1(grid: *HexGrid, input: *Input) Answer {
    for (input.paths) |path| {
        grid.addPath(path);
    }
    return grid.count();
}

fn part2(grid: *HexGrid) Answer {
    grid.tickN(100) catch unreachable;
    return grid.count();
}

test "examples" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/test_day24.txt");
    var input = try parseInput(alloc, test_input);
    defer input.deinit();

    var grid = HexGrid.init(input.allocator);
    defer grid.deinit();

    try expect(part1(&grid, &input) == 10);
    try expect(part2(&grid) == 2208);
}

test "answers" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/day24.txt");
    var input = try parseInput(alloc, test_input);
    defer input.deinit();

    var grid = HexGrid.init(input.allocator);
    defer grid.deinit();

    try expect(part1(&grid, &input) == 485);
    try expect(part2(&grid) == 3933);
}
