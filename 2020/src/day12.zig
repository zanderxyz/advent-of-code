const std = @import("std");
const print = std.log.info;
const expect = std.testing.expect;

const INPUT_FILE = @embedFile("inputs/day12.txt");

const Answer = isize;
const Input = []Instruction;

const Instruction = struct {
    action: Action,
    distance: isize,
};

const Action = enum {
    north,
    south,
    east,
    west,
    left,
    right,
    forward,
};

const Position = struct {
    x: isize,
    y: isize,

    fn apply(self: *Position, direction: Direction, distance: isize) void {
        switch (direction) {
            .north => {
                self.y += distance;
            },
            .south => {
                self.y -= distance;
            },
            .west => {
                self.x -= distance;
            },
            .east => {
                self.x += distance;
            },
        }
    }

    fn rotateRight(self: *Position) void {
        const temp = self.x;
        self.x = self.y;
        self.y = -temp;
    }

    fn dist(self: Position) isize {
        const x = std.math.absInt(self.x) catch unreachable;
        const y = std.math.absInt(self.y) catch unreachable;
        return x + y;
    }
};

const Direction = enum {
    north,
    south,
    west,
    east,
};

const Ship = struct {
    position: Position,
    waypoint: Position,
    direction: Direction,

    // 0 == due north
    fn new() Ship {
        return Ship{
            .position = Position{
                .x = 0,
                .y = 0,
            },
            .waypoint = Position{
                .x = 10,
                .y = 1,
            },
            .direction = .east,
        };
    }

    fn move(self: *Ship, direction: Direction, distance: isize) void {
        self.position.apply(direction, distance);
        //self.waypoint.apply(direction, distance);
    }

    fn moveToWaypoint(self: *Ship, moves: isize) void {
        var i: usize = 0;
        while (i < moves) : (i += 1) {
            self.move(.east, self.waypoint.x);
            self.move(.north, self.waypoint.y);
        }
    }

    fn moveWaypoint(self: *Ship, direction: Direction, distance: isize) void {
        self.waypoint.apply(direction, distance);
    }

    fn apply(self: *Ship, ix: Instruction) void {
        switch (ix.action) {
            .north => {
                self.move(.north, ix.distance);
            },
            .south => {
                self.move(.south, ix.distance);
            },
            .west => {
                self.move(.west, ix.distance);
            },
            .east => {
                self.move(.east, ix.distance);
            },
            .left => {
                self.rotate(-ix.distance);
            },
            .right => {
                self.rotate(ix.distance);
            },
            .forward => {
                self.move(self.direction, ix.distance);
            },
        }
    }

    fn applyWaypoint(self: *Ship, ix: Instruction) void {
        switch (ix.action) {
            .north => {
                self.moveWaypoint(.north, ix.distance);
            },
            .south => {
                self.moveWaypoint(.south, ix.distance);
            },
            .west => {
                self.moveWaypoint(.west, ix.distance);
            },
            .east => {
                self.moveWaypoint(.east, ix.distance);
            },
            .left => {
                self.rotateWaypoint(-ix.distance);
            },
            .right => {
                self.rotateWaypoint(ix.distance);
            },
            .forward => {
                self.moveToWaypoint(ix.distance);
            },
        }
    }

    fn rotate(self: *Ship, degrees: isize) void {
        var rotations: isize = undefined;
        if (degrees < 0) {
            rotations = 4 - @divFloor(-degrees, 90);
        } else {
            rotations = @divFloor(degrees, 90);
        }
        var i: usize = 0;
        while (i < rotations) : (i += 1) {
            self.rotateRight();
        }
    }

    fn rotateRight(self: *Ship) void {
        switch (self.direction) {
            .north => {
                self.direction = .east;
            },
            .east => {
                self.direction = .south;
            },
            .south => {
                self.direction = .west;
            },
            .west => {
                self.direction = .north;
            },
        }
    }

    fn rotateWaypoint(self: *Ship, degrees: isize) void {
        var rotations: isize = undefined;
        if (degrees < 0) {
            rotations = 4 - @divFloor(-degrees, 90);
        } else {
            rotations = @divFloor(degrees, 90);
        }
        var i: usize = 0;
        while (i < rotations) : (i += 1) {
            self.rotateWaypointRight();
        }
    }

    fn rotateWaypointRight(self: *Ship) void {
        self.waypoint.rotateRight();
    }

    fn dist(self: Ship) isize {
        return self.position.dist();
    }
};

pub fn main() !void {
    var alloc = std.heap.GeneralPurposeAllocator(.{}){};

    const input = try parseInput(alloc.allocator(), INPUT_FILE);
    defer alloc.allocator().free(input);

    print("Part 1: {}\n", .{part1(input)});
    print("Part 2: {}\n", .{part2(input)});
}

fn parseInput(allocator: std.mem.Allocator, input: []const u8) !Input {
    var result = std.ArrayList(Instruction).init(allocator);
    errdefer result.deinit();

    var instructions = std.mem.tokenize(u8, input, "\n");
    while (instructions.next()) |line| {
        const action: Action = switch (line[0]) {
            'N' => .north,
            'S' => .south,
            'E' => .east,
            'W' => .west,
            'L' => .left,
            'R' => .right,
            'F' => .forward,
            else => unreachable,
        };
        const distance = try std.fmt.parseInt(isize, line[1..], 10);

        const instruction = Instruction{
            .action = action,
            .distance = distance,
        };

        try result.append(instruction);
    }

    return result.toOwnedSlice();
}

fn part1(input: Input) Answer {
    var ship = Ship.new();

    for (input) |instruction| {
        ship.apply(instruction);
    }

    return ship.dist();
}

fn part2(input: Input) Answer {
    var ship = Ship.new();

    for (input) |instruction| {
        ship.applyWaypoint(instruction);
    }

    return ship.dist();
}

test "example" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/test_day12.txt");
    const input = try parseInput(alloc, test_input);
    defer alloc.free(input);

    try expect(part1(input) == 25);
    try expect(part2(input) == 286);
}

test "answers" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/day12.txt");
    const input = try parseInput(alloc, test_input);
    defer alloc.free(input);

    try expect(part1(input) == 1457);
    try expect(part2(input) == 106860);
}
