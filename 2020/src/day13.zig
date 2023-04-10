const std = @import("std");
const print = std.log.info;
const expect = std.testing.expect;

const INPUT_FILE = @embedFile("inputs/day13.txt");

const Answer = usize;
const Input = struct {
    allocator: std.mem.Allocator,
    time: Answer,
    buses: []Bus,

    fn deinit(self: Input) void {
        self.allocator.free(self.buses);
    }
};

const Bus = struct {
    id: Answer,
    mod: Answer,
};

pub fn main() !void {
    var alloc = std.heap.GeneralPurposeAllocator(.{}){};

    const input = try parseInput(alloc.allocator(), INPUT_FILE);
    defer input.deinit();

    print("Part 1: {}\n", .{part1(input)});
    print("Part 2: {}\n", .{part2(input)});
}

fn parseInput(allocator: std.mem.Allocator, input: []const u8) !Input {
    var result = std.ArrayList(Bus).init(allocator);
    errdefer result.deinit();

    var lines = std.mem.tokenize(u8, input, "\n");
    const time = try std.fmt.parseInt(usize, lines.next().?, 10);
    var buses = std.mem.tokenize(u8, lines.next().?, ",");
    var i: usize = 0;
    while (buses.next()) |bus_id_string| {
        const id: ?Answer = std.fmt.parseInt(Answer, bus_id_string, 10) catch {
            // Skip any non-integer bus numbers
            i += 1;
            continue;
        };

        const bus_id = id.?;
        const mod = bus_id - (i % bus_id);
        const bus = Bus{
            .id = bus_id,
            .mod = mod,
        };
        try result.append(bus);

        i += 1;
    }

    return Input{
        .allocator = allocator,
        .time = time,
        .buses = result.toOwnedSlice(),
    };
}

fn part1(input: Input) Answer {
    const BestBus = struct {
        bus_id: Answer,
        time: Answer,
    };

    var min = BestBus{
        .bus_id = 0,
        .time = input.time,
    };

    for (input.buses) |bus| {
        const time = bus.id - (input.time % bus.id);
        if (time < min.time) {
            min.time = time;
            min.bus_id = bus.id;
        }
    }
    return min.bus_id * min.time;
}

fn part2(input: Input) Answer {
    // Step forward until all buses fit, increasing the jump each time we find a match
    // LCM = product of the bus ids, as they are all prime
    var time: usize = 0;
    var jump: usize = input.buses[0].id;
    var found: usize = 1;
    while (found < input.buses.len) {
        time += jump;

        const bus = input.buses[found];
        if (time % bus.id == bus.mod) {
            jump *= bus.id;
            found += 1;
        }
    }
    return time;
}

test "example" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/test_day13.txt");
    const input = try parseInput(alloc, test_input);
    defer input.deinit();

    try expect(part1(input) == 295);
    try expect(part2(input) == 1068781);
}

test "answers" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/day13.txt");
    const input = try parseInput(alloc, test_input);
    defer input.deinit();

    try expect(part1(input) == 3035);
    try expect(part2(input) == 725169163285238);
}
