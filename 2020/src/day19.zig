const std = @import("std");
const print = std.log.info;
const expect = std.testing.expect;

const INPUT_FILE = @embedFile("inputs/day19.txt");

const Answer = usize;

const Message = []const u8;

const Rules = std.AutoHashMap(usize, Rule);

const Rule = struct {
    number: usize,
    data: RuleData,

    fn possibleLength(self: Rule, rules: Rules, length: usize) bool {
        return switch (self.data) {
            .literal => length == 1,
            .list => |list| one_of: {
                for (list) |group| {
                    var s: usize = 0;
                    for (group) |sub_rule_id| {
                        s += rules.get(sub_rule_id).?.getLength(rules);
                    }
                    if (s == length) {
                        break :one_of true;
                    }
                }
                break :one_of false;
            },
        };
    }

    fn getLength(self: Rule, rules: Rules) usize {
        return switch (self.data) {
            .literal => 1,
            .list => |list| sum: {
                var s: usize = 0;
                for (list[0]) |sub_rule_id| {
                    s += rules.get(sub_rule_id).?.getLength(rules);
                }
                break :sum s;
            },
        };
    }
};

const RuleData = union(enum) {
    literal: u8,
    list: [][]usize,
};

const Input = struct {
    allocator: std.mem.Allocator,
    rules: Rules,
    messages: []Message,

    fn init(allocator: std.mem.Allocator, rules: Rules, messages: []Message) Input {
        return Input{
            .allocator = allocator,
            .rules = rules,
            .messages = messages,
        };
    }

    fn deinit(self: *Input) void {
        var iterator = self.rules.iterator();
        while (iterator.next()) |entry| {
            var rule = entry.value_ptr.*;
            switch (rule.data) {
                .literal => {},
                .list => |list| {
                    for (list) |group| {
                        self.allocator.free(group);
                    }
                    self.allocator.free(list);
                },
            }
        }
        self.rules.deinit();
        self.allocator.free(self.messages);
    }

    fn validate(self: Input, message: Message, rule: Rule) bool {
        if (!rule.possibleLength(self.rules, message.len)) return false;
        return switch (rule.data) {
            .literal => |char| message.len == 1 and message[0] == char,
            .list => |list| self.validateList(message, list),
        };
    }

    fn validateList(self: Input, message: Message, list: [][]usize) bool {
        // A pair is valid if ANY of the sub groups are valid
        for (list) |group| {
            if (self.validateSequence(message, group)) {
                return true;
            }
        }
        return false;
    }

    fn validateSequence(self: Input, message: Message, seq: []usize) bool {
        // A sequence is valid if ALL the sub sequences are valid
        var index: usize = 0;
        var index_end: usize = 0;
        for (seq) |sub_rule_id| {
            index = index_end;
            const sub_rule = self.rules.get(sub_rule_id).?;
            // Slice up the message and move along by the length of the rule we are checking
            index_end += sub_rule.getLength(self.rules);
            // If we have gone too far, rturn
            if (index_end > message.len) return false;
            const sub_message = message[index..index_end];
            if (!self.validate(sub_message, sub_rule)) {
                // If we fail at any point, it is not valid
                return false;
            }
        }
        // Success if we validate the whole message
        return index_end == message.len;
    }
};

pub fn main() !void {
    var alloc = std.heap.GeneralPurposeAllocator(.{}){};

    var input = try parseInput(alloc.allocator(), INPUT_FILE);
    defer input.deinit();

    print("Part 1: {}\n", .{part1(input)});
    print("Part 2: {}\n", .{part2(input, 8)});
}

fn parseInput(allocator: std.mem.Allocator, input: []const u8) !Input {
    var rules = Rules.init(allocator);
    errdefer rules.deinit();

    var messages = std.ArrayList(Message).init(allocator);
    errdefer messages.deinit();

    const ParserState = enum {
        rules,
        messages,
    };

    var state: ParserState = .rules;

    var lines = std.mem.tokenize(u8, input, "\n");
    while (lines.next()) |line| {
        switch (state) {
            .rules => {
                var split = std.mem.split(u8, line, ": ");
                var rule_name = split.next().?;
                const rule_number = std.fmt.parseInt(usize, rule_name, 10) catch {
                    state = .messages;
                    try messages.append(line);
                    continue;
                };

                const rule_content = split.next().?;

                var rule: Rule = undefined;
                if (rule_content[0] == '"') {
                    rule = Rule{
                        .number = rule_number,
                        .data = RuleData{ .literal = rule_content[1] },
                    };
                } else {
                    var list = std.ArrayList([]usize).init(allocator);
                    errdefer list.deinit();

                    var groups = std.mem.split(u8, rule_content, " | ");
                    while (groups.next()) |group_string| {
                        var group = std.ArrayList(usize).init(allocator);
                        errdefer group.deinit();

                        var group_content = std.mem.split(u8, group_string, " ");

                        while (group_content.next()) |string| {
                            const num = try std.fmt.parseInt(usize, string, 10);
                            try group.append(num);
                        }

                        try list.append(group.toOwnedSlice());
                    }

                    rule = Rule{
                        .number = rule_number,
                        .data = RuleData{ .list = list.toOwnedSlice() },
                    };
                }

                try rules.put(rule.number, rule);
            },
            .messages => {
                try messages.append(line);
            },
        }
    }

    return Input.init(allocator, rules, messages.toOwnedSlice());
}

fn part1(input: Input) Answer {
    var sum: Answer = 0;
    const rule = input.rules.get(0).?;
    for (input.messages) |message| {
        if (input.validate(message, rule)) {
            sum += 1;
        }
    }

    return sum;
}

fn part2(input: Input, comptime magic: usize) Answer {
    var sum: Answer = 0;
    const rule42 = input.rules.get(42).?;
    const rule31 = input.rules.get(31).?;

    for (input.messages) |message| {
        if (message.len % magic != 0) continue;
        // Divide message into blocks of 8 (5 for examples)
        // This is a magic number figured out from the input
        // First X blocks must match 42, then remaining Y match 31
        // Rule 8 = (42+)
        // Rule 11 = (42+) (31+) with equal of each
        // Rule 0 = 8 11 => 42a 42b 31b where (a+b)=X b=Y, a+b+b=max
        // Where X > Y and Y >= 1.
        var i: usize = 0;
        var max: usize = message.len / magic;
        var X: usize = 0;
        while (i < max - 1) {
            const sub_str = message[i * magic .. i * magic + magic];
            if (input.validate(sub_str, rule42)) {
                X += 1;
                i += 1;
            } else {
                break;
            }
        }
        if (X > 1 and X < max) {
            var Y: usize = 0;

            while (i < max) : (i += 1) {
                const sub_str = message[i * magic .. i * magic + magic];
                if (input.validate(sub_str, rule31)) {
                    Y += 1;
                } else {
                    break;
                }
            }

            if (X + Y == max and X > Y) {
                sum += 1;
            }
        }
    }
    return sum;
}

test "examples" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/test_day19.txt");
    var input = try parseInput(alloc, test_input);
    defer input.deinit();

    try expect(part1(input) == 2);
}

test "examples 2" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/test_day19_2.txt");
    var input = try parseInput(alloc, test_input);
    defer input.deinit();

    try expect(part1(input) == 3);
    try expect(part2(input, 5) == 12);
}

test "answers" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/day19.txt");
    var input = try parseInput(alloc, test_input);
    defer input.deinit();

    try expect(part1(input) == 187);
    try expect(part2(input, 8) == 392);
}
