const std = @import("std");
const print = std.log.info;
const expect = std.testing.expect;

const INPUT_FILE = @embedFile("inputs/day05.txt");

const Answer = usize;
const Occupied = std.AutoHashMap(Answer, void);
const Input = struct {
    allocator: std.mem.Allocator,
    seats: []Seat,

    fn deinit(self: *Input) void {
        defer self.allocator.free(self.seats);
    }
};

const Seat = struct {
    row: Answer,
    col: Answer,
    id: Answer,
};

pub fn main() !void {
    var alloc = std.heap.GeneralPurposeAllocator(.{}){};

    var input = try parseInput(alloc.allocator(), INPUT_FILE);
    defer input.deinit();

    print("Part 1: {}\n", .{part1(input)});
    print("Part 2: {}\n", .{part2(input)});
}

fn parseInput(allocator: std.mem.Allocator, input: []const u8) !Input {
    var result = std.ArrayList(Seat).init(allocator);
    errdefer result.deinit();

    var lines = std.mem.tokenize(u8, input, "\n");
    while (lines.next()) |line| {
        const seat = decodeSeat(line);
        try result.append(seat);
    }

    return Input{
        .allocator = allocator,
        .seats = result.toOwnedSlice(),
    };
}

fn part1(input: Input) Answer {
    var max: Answer = 0;
    for (input.seats) |seat| {
        if (seat.id > max) {
            max = seat.id;
        }
    }
    return max;
}

fn part2(input: Input) Answer {
    // Fill an array with the seats we do and don't have
    var occupied = Occupied.init(input.allocator);
    defer occupied.deinit();

    for (input.seats) |seat| {
        occupied.put(seat.id, {}) catch unreachable;
    }

    var i: Answer = 1;
    while (i < 128 * 8 - 1) {
        // We are looking for a missing seat that is filled on each side
        if (occupied.get(i) == null and occupied.get(i - 1) != null and occupied.get(i + 1) != null) {
            return i;
        }
        i += 1;
    }

    unreachable;
}

fn decodeSeat(encoded: []const u8) Seat {
    var row_min: Answer = 0;
    var row_max: Answer = 127;
    var col_min: Answer = 0;
    var col_max: Answer = 7;
    for (encoded) |char| {
        switch (char) {
            'F' => {
                row_max = (row_max + row_min + 1) / 2 - 1;
            },
            'B' => {
                row_min = row_min + (row_max - row_min + 1) / 2;
            },
            'L' => {
                col_max = (col_max + col_min + 1) / 2 - 1;
            },
            'R' => {
                col_min = col_min + (col_max - col_min + 1) / 2;
            },
            else => unreachable,
        }
    }
    if (row_min == row_max and col_min == col_max) {
        return Seat{
            .row = row_min,
            .col = col_min,
            .id = 8 * row_min + col_min,
        };
    }
    unreachable;
}

test "get seat id" {
    try expect(decodeSeat("FBFBBFFRLR").id == 357);
    try expect(decodeSeat("BFFFBBFRRR").id == 567);
    try expect(decodeSeat("FFFBBBFRRR").id == 119);
    try expect(decodeSeat("BBFFBBFRLL").id == 820);
}

test "answers" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/day05.txt");
    var input = try parseInput(alloc, test_input);
    defer input.deinit();

    try expect(part1(input) == 885);
    try expect(part2(input) == 623);
}
