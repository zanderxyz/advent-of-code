const std = @import("std");
const print = std.log.info;
const expect = std.testing.expect;

const INPUT_FILE = @embedFile("inputs/day14.txt");

const Answer = usize;
const Address = u36;
const Input = struct {
    allocator: std.mem.Allocator,
    instructions: []Instruction,

    fn deinit(self: *Input) void {
        defer self.allocator.free(self.instructions);
    }
};

const Contents = std.AutoHashMap(Answer, Address);

const Memory = struct {
    contents: Contents,

    const Self = @This();

    fn init(allocator: std.mem.Allocator) Self {
        return Self{
            .contents = Contents.init(allocator),
        };
    }

    fn deinit(self: *Self) void {
        defer self.contents.deinit();
    }

    fn sum(self: Self) Answer {
        var s: Answer = 0;
        var iterator = self.contents.iterator();
        while (iterator.next()) |entry| {
            s += entry.value_ptr.*;
        }
        return s;
    }

    fn apply(self: *Self, ix: Instruction) void {
        self.contents.put(ix.register, ix.applyToValue()) catch unreachable;
    }

    fn applyAddress(self: *Self, ix: Instruction) void {
        const address = ix.applyToRegister();
        self.applyFloating(address, ix.value, ix.mask.blanks);
    }

    fn applyFloating(self: *Self, address: Address, value: Address, mask: u36) void {
        if (mask == 0) {
            // When the mask is zero, set the register value
            self.contents.put(address, value) catch unreachable;
        } else {
            // For every 1 in mask, we want to recursively call this function twice
            // Fetch the smallest bit
            const least_significant = @as(Address, 1) << @ctz(mask);

            // Reduce the mask each time, so eventually we end up with 2^n applications
            const new_mask = mask & ~least_significant;

            // Branch twice -> once with 0, once with 1
            self.applyFloating(address, value, new_mask);
            self.applyFloating(address | least_significant, value, new_mask);
        }
    }
};

const Instruction = struct {
    register: Address,
    value: Address,
    mask: Mask,

    fn applyToValue(self: Instruction) Address {
        // Apply the mask
        // => AND with the blanks, so we copy across everything that was blank in the mask
        // => OR with the ones, so we copy across everything that was 1 in the mask
        return (self.value & self.mask.blanks) | self.mask.ones;
    }

    fn applyToRegister(self: Instruction) Address {
        // Apply the mask
        // => AND with the zeroes, so we copy across everything that was 0 in the mask
        // => OR with the ones, so we copy across everything that was 1 in the mask
        return (self.register & self.mask.zeroes) | self.mask.ones;
    }
};

const Mask = struct {
    blanks: Address,
    ones: Address,
    zeroes: Address,

    fn from(string: []const u8) Mask {
        var zero_mask: Address = 0;
        var value_mask: Address = 0;
        var blank_mask: Address = 0;

        for (string) |char, j| {
            const val: Address = @as(Address, 1) << @intCast(u6, 35 - j);

            switch (char) {
                'X' => blank_mask |= val,
                '0' => zero_mask |= val,
                '1' => value_mask |= val,
                else => unreachable,
            }
        }

        return Mask{
            .blanks = blank_mask,
            .zeroes = zero_mask,
            .ones = value_mask,
        };
    }
};

pub fn main() !void {
    var alloc = std.heap.GeneralPurposeAllocator(.{}){};

    var input = try parseInput(alloc.allocator(), INPUT_FILE);
    defer input.deinit();

    print("Part 1: {}\n", .{part1(&input)});
    print("Part 2: {}\n", .{part2(&input)});
}

fn parseInput(allocator: std.mem.Allocator, input: []const u8) !Input {
    var instructions = std.ArrayList(Instruction).init(allocator);
    errdefer instructions.deinit();

    var mask: Mask = undefined;

    var lines = std.mem.tokenize(u8, input, "\n");
    while (lines.next()) |line| {
        if (std.mem.eql(u8, line[0..4], "mask")) {
            mask = Mask.from(line[7..]);
        } else {
            const without_prefix = line[4..];
            var split = std.mem.split(u8, without_prefix, "] = ");
            const register = try std.fmt.parseInt(Address, split.next().?, 10);
            const value = try std.fmt.parseInt(Address, split.next().?, 10);

            const ix = Instruction{
                .register = register,
                .value = value,
                .mask = mask,
            };

            try instructions.append(ix);
        }
    }

    return Input{
        .allocator = allocator,
        .instructions = instructions.toOwnedSlice(),
    };
}

fn part1(input: *Input) Answer {
    var memory = Memory.init(input.allocator);
    defer memory.deinit();

    for (input.instructions) |ix| {
        memory.apply(ix);
    }
    return memory.sum();
}

fn part2(input: *Input) Answer {
    var memory = Memory.init(input.allocator);
    defer memory.deinit();

    for (input.instructions) |ix| {
        memory.applyAddress(ix);
    }
    return memory.sum();
}

test "example 1" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/test_day14.txt");
    var input = try parseInput(alloc, test_input);
    defer input.deinit();

    try expect(part1(&input) == 165);
}

test "example 2" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/test_day14_2.txt");
    var input = try parseInput(alloc, test_input);
    defer input.deinit();

    try expect(part2(&input) == 208);
}

test "answers" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/day14.txt");
    var input = try parseInput(alloc, test_input);
    defer input.deinit();

    try expect(part1(&input) == 11179633149677);
    try expect(part2(&input) == 4822600194774);
}
