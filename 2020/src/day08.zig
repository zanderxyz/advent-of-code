const std = @import("std");
const print = std.log.info;
const expect = std.testing.expect;

const INPUT_FILE = @embedFile("inputs/day08.txt");

const Answer = isize;
const Input = struct {
    allocator: std.mem.Allocator,
    items: []Instruction,

    fn deinit(self: *Input) void {
        self.allocator.free(self.items);
    }

    fn run(input: Input) Output {
        var acc: Answer = 0;
        var pointer: Answer = 0;
        var visited = Set.init(input.allocator);
        defer visited.deinit();

        while (true) {
            if (visited.contains(pointer)) {
                return Output{
                    .not_terminated = acc,
                };
            }
            if (pointer >= input.items.len) {
                return Output{
                    .terminated = acc,
                };
            }

            visited.put(pointer, {}) catch unreachable;

            const instruction = input.items[@intCast(usize, pointer)];
            switch (instruction.operation) {
                Operation.nop => {
                    pointer += 1;
                },
                Operation.acc => {
                    pointer += 1;
                    acc += instruction.argument;
                },
                Operation.jmp => {
                    pointer += instruction.argument;
                },
            }
        }

        unreachable;
    }
};

const Instruction = struct {
    operation: Operation,
    argument: Answer,
};

const Operation = enum { nop, acc, jmp };

const Set = std.AutoHashMap(isize, void);

const Output = union(enum) {
    terminated: Answer,
    not_terminated: Answer,
};

pub fn main() !void {
    var alloc = std.heap.GeneralPurposeAllocator(.{}){};

    var input = try parseInput(alloc.allocator(), INPUT_FILE);
    defer input.deinit();

    print("Part 1: {}\n", .{part1(input)});
    print("Part 2: {}\n", .{part2(input)});
}

fn parseInput(allocator: std.mem.Allocator, input: []const u8) !Input {
    var result = std.ArrayList(Instruction).init(allocator);
    errdefer result.deinit();

    var instructions = std.mem.tokenize(u8, input, "\n");
    while (instructions.next()) |line| {
        var parts = std.mem.split(u8, line, " ");
        var operationString = parts.next().?;
        var argumentString = parts.next().?;

        const operation = std.meta.stringToEnum(Operation, operationString).?;
        const argument = try std.fmt.parseInt(isize, argumentString, 10);

        const instruction = Instruction{
            .operation = operation,
            .argument = argument,
        };

        try result.append(instruction);
    }

    return Input{
        .allocator = allocator,
        .items = result.toOwnedSlice(),
    };
}

fn modifyOperation(instruction: *Instruction) ?Instruction {
    const new_operation = switch (instruction.operation) {
        Operation.acc => return null,
        Operation.nop => Operation.jmp,
        Operation.jmp => Operation.nop,
    };

    return Instruction{
        .operation = new_operation,
        .argument = instruction.argument,
    };
}

fn part1(input: Input) Answer {
    switch (input.run()) {
        Output.terminated => unreachable,
        Output.not_terminated => |value| return value,
    }

    unreachable;
}

fn part2(input: Input) Answer {
    for (input.items) |*instruction| {
        const new_instruction = modifyOperation(instruction) orelse continue;
        instruction.* = new_instruction;

        switch (input.run()) {
            Output.terminated => |value| return value,
            Output.not_terminated => {},
        }

        instruction.* = modifyOperation(instruction).?;
    }

    unreachable;
}

test "example" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/test_day08.txt");
    var input = try parseInput(alloc, test_input);
    defer input.deinit();

    try expect(part1(input) == 5);
    try expect(part2(input) == 8);
}

test "answers" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/day08.txt");
    var input = try parseInput(alloc, test_input);
    defer input.deinit();

    try expect(part1(input) == 2080);
    try expect(part2(input) == 2477);
}
