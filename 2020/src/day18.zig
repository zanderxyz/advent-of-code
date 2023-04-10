const std = @import("std");
const print = std.log.info;
const expect = std.testing.expect;

const INPUT_FILE = @embedFile("inputs/day18.txt");

const Answer = usize;

const Expression = []Token;

const MAX_STACK = 16;

const Token = union(enum) {
    number: usize,
    op: Operator,
    close: void,
};

const Operator = enum {
    open,
    add,
    mul,
};

const Input = struct {
    allocator: std.mem.Allocator,
    expressions: []Expression,

    fn init(allocator: std.mem.Allocator, expressions: []Expression) Input {
        return Input{
            .allocator = allocator,
            .expressions = expressions,
        };
    }

    fn deinit(self: Input) void {
        for (self.expressions) |expr| {
            self.allocator.free(expr);
        }
        self.allocator.free(self.expressions);
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
    var expressions = std.ArrayList([]Token).init(allocator);
    errdefer expressions.deinit();

    var lines = std.mem.tokenize(u8, input, "\n");
    while (lines.next()) |line| {
        var expr = std.ArrayList(Token).init(allocator);
        errdefer expr.deinit();

        for (line) |char, i| {
            const token: Token = switch (char) {
                ' ' => continue,
                '(' => Token{ .op = Operator.open },
                ')' => Token.close,
                '+' => Token{ .op = Operator.add },
                '*' => Token{ .op = Operator.mul },
                else => number: {
                    const number = try std.fmt.parseInt(usize, line[i .. i + 1], 10);

                    break :number Token{ .number = number };
                },
            };

            try expr.append(token);
        }

        try expressions.append(expr.toOwnedSlice());
    }

    return Input.init(allocator, expressions.toOwnedSlice());
}

// Generic stack with a comptime defined maximum size, so we can use an array
fn Stack(comptime MAX: usize, comptime T: type) type {
    return struct {
        items: [MAX]T = undefined,
        len: usize = 0,

        const Self = @This();

        fn push(self: *Self, item: T) void {
            self.items[self.len] = item;
            self.len += 1;
        }

        fn pop(self: *Self) T {
            self.len -= 1;
            return self.items[self.len];
        }

        // This should return a nullable but implementing it like that raises a compiler error -> Zig compiler bug?
        fn peek(self: Self) T {
            return self.items[self.len - 1];
        }
    };
}

const Priority = enum {
    none,
    add,
};

fn Calculator(comptime MAX: usize, comptime p: Priority) type {
    const Numbers = Stack(MAX, Answer);
    const Operators = Stack(MAX, Operator);

    return struct {
        numbers: Numbers = Numbers{},
        operators: Operators = Operators{},
        priority: Priority = p,

        const Self = @This();

        fn addNumber(self: *Self, n: Answer) void {
            self.numbers.push(n);
        }

        fn addOperator(self: *Self, op: Operator) void {
            self.operators.push(op);
        }

        fn reduce(self: *Self) void {
            switch (self.priority) {
                .none => self.reduceWhileNotEqual(Operator.open),
                .add => self.reduceWhileEqual(Operator.add),
            }
        }

        fn closeParens(self: *Self) void {
            self.reduceWhileNotEqual(Operator.open);
            // Next operator is the open bracket, discard it
            _ = self.operators.pop();
            self.reduce();
        }

        fn reduceFinal(self: *Self) void {
            while (self.operators.len > 0) {
                if (!self.canRun()) {
                    // When reducing for the final time, this should always succeed if the expression is valid
                    unreachable;
                }
                self.runOnce();
            }
        }

        fn reduceWhileEqual(self: *Self, op: Operator) void {
            while (self.nextOperatorEqual(op) and self.canRun()) {
                self.runOnce();
            }
        }

        fn reduceWhileNotEqual(self: *Self, op: Operator) void {
            while (self.nextOperatorNotEqual(op) and self.canRun()) {
                self.runOnce();
            }
        }

        fn nextOperatorEqual(self: Self, op: Operator) bool {
            return self.operators.len > 0 and self.operators.peek() == op;
        }

        fn nextOperatorNotEqual(self: Self, op: Operator) bool {
            return self.operators.len > 0 and self.operators.peek() != op;
        }

        fn canRun(self: Self) bool {
            return self.numbers.len >= 2 and self.operators.len >= 1;
        }

        fn runOnce(self: *Self) void {
            const first = self.numbers.pop();
            const second = self.numbers.pop();

            const answer = switch (self.operators.pop()) {
                Operator.add => first + second,
                Operator.mul => first * second,
                else => unreachable,
            };

            self.addNumber(answer);
        }
    };
}

fn eval(comptime priority: Priority, expr: Expression) Answer {
    var c = Calculator(MAX_STACK, priority){};

    for (expr) |token| {
        switch (token) {
            Token.number => |n| {
                c.addNumber(n);
                c.reduce();
            },
            Token.op => |op| {
                c.addOperator(op);
            },
            Token.close => {
                c.closeParens();
            },
        }
    }

    c.reduceFinal();

    return c.numbers.pop();
}

fn part1(input: Input) Answer {
    var sum: Answer = 0;
    for (input.expressions) |expr| {
        sum += eval(.none, expr);
    }

    return sum;
}

fn part2(input: Input) Answer {
    var sum: Answer = 0;
    for (input.expressions) |expr| {
        sum += eval(.add, expr);
    }

    return sum;
}

test "examples" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/test_day18.txt");
    var input = try parseInput(alloc, test_input);
    defer input.deinit();

    try expect(part1(input) == 26457);
    try expect(part2(input) == 694173);
}

test "answers" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/day18.txt");
    var input = try parseInput(alloc, test_input);
    defer input.deinit();

    try expect(part1(input) == 1408133923393);
    try expect(part2(input) == 314455761823725);
}
