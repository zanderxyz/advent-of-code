const std = @import("std");
const print = std.log.info;
const expect = std.testing.expect;

const INPUT_FILE = @embedFile("inputs/day16.txt");

const FIELDS = 20;

const Answer = usize;

const Input = struct {
    allocator: std.mem.Allocator,
    rules: []Rule,
    ticket: Ticket,
    nearby: []Ticket,

    fn deinit(self: *Input) void {
        defer self.allocator.free(self.rules);
        defer self.allocator.free(self.nearby);
        defer self.allocator.free(self.ticket.numbers);
        defer {
            for (self.nearby) |ticket| {
                self.allocator.free(ticket.numbers);
            }
        }
    }

    fn validNumber(self: Input, number: usize) bool {
        for (self.rules) |rule| {
            if (rule.validFor(number)) {
                return true;
            }
        }

        return false;
    }

    // Returns 0 if the ticket is valid
    fn validate(self: Input, ticket: *Ticket) usize {
        var error_rate: usize = 0;
        for (ticket.numbers) |number| {
            if (!self.validNumber(number)) {
                error_rate += number;
                ticket.valid = false;
            }
        }

        return error_rate;
    }

    fn validateRuleForColumn(self: Input, rule: *Rule, column: usize) void {
        for (self.nearby) |ticket| {
            if (!ticket.valid) continue;
            if (!rule.validFor(ticket.numbers[column])) {
                return;
            }
        }

        rule.possibleColumns[column] = true;
    }
};

const Ticket = struct {
    numbers: []usize,
    valid: bool = true,
};

const Range = struct {
    min: usize,
    max: usize,

    fn validFor(self: Range, number: usize) bool {
        return number >= self.min and number <= self.max;
    }
};

const Rule = struct {
    name: []const u8,
    is_departure: bool,
    is_final: bool = false,
    left: Range,
    right: Range,
    possibleColumns: [FIELDS]bool = [_]bool{false} ** FIELDS,

    fn validFor(self: Rule, number: usize) bool {
        return self.left.validFor(number) or self.right.validFor(number);
    }

    fn findFinalColumn(self: *Rule) ?usize {
        if (self.is_final) return null;

        var i: usize = 0;
        var final: usize = undefined;
        var count: usize = 0;
        while (i < FIELDS) : (i += 1) {
            const possible = self.possibleColumns[i];
            if (possible) {
                count += 1;
                final = i;
            }
        }
        // If there is only one possible column for this rule, it is final
        if (count == 1) {
            self.is_final = true;
            return final;
        } else {
            return null;
        }
    }
};

pub fn main() !void {
    var alloc = std.heap.GeneralPurposeAllocator(.{}){};

    var input = try parseInput(alloc.allocator(), INPUT_FILE);
    defer input.deinit();

    print("Part 1: {}\n", .{part1(&input)});
    print("Part 2: {}\n", .{part2(input)});
}

fn parseInput(allocator: std.mem.Allocator, input: []const u8) !Input {
    const Stage = enum {
        rules,
        ticket,
        nearby,
    };

    var stage = Stage.rules;

    var rules = std.ArrayList(Rule).init(allocator);
    errdefer rules.deinit();

    var ticket: Ticket = undefined;

    var nearby = std.ArrayList(Ticket).init(allocator);
    errdefer nearby.deinit();

    var lines = std.mem.tokenize(u8, input, "\n");
    while (lines.next()) |line| {
        switch (stage) {
            .rules => {
                if (std.mem.eql(u8, line[0..4], "your")) {
                    stage = .ticket;
                    continue;
                }

                const rule = try parseRule(line);

                try rules.append(rule);
            },
            .ticket => {
                if (std.mem.eql(u8, line[0..6], "nearby")) {
                    stage = .nearby;
                    continue;
                }

                ticket = try parseTicket(allocator, line);
            },
            .nearby => {
                const ticket_nearby = try parseTicket(allocator, line);

                try nearby.append(ticket_nearby);
            },
        }
    }

    return Input{
        .allocator = allocator,
        .rules = rules.toOwnedSlice(),
        .ticket = ticket,
        .nearby = nearby.toOwnedSlice(),
    };
}

fn parseRule(line: []const u8) !Rule {
    var parts = std.mem.split(u8, line, ": ");
    const name = parts.next().?;
    const is_departure = std.mem.eql(u8, name[0..3], "dep");

    const rule_str = parts.next().?;

    var rule_split = std.mem.split(u8, rule_str, " or ");
    const left_str = rule_split.next().?;

    var left_str_split = std.mem.split(u8, left_str, "-");
    const left_str_min = left_str_split.next().?;
    const left_min = try std.fmt.parseInt(usize, left_str_min, 10);
    const left_str_max = left_str_split.next().?;
    const left_max = try std.fmt.parseInt(usize, left_str_max, 10);

    const right_str = rule_split.next().?;
    var right_str_split = std.mem.split(u8, right_str, "-");
    const right_str_min = right_str_split.next().?;
    const right_min = try std.fmt.parseInt(usize, right_str_min, 10);
    const right_str_max = right_str_split.next().?;
    const right_max = try std.fmt.parseInt(usize, right_str_max, 10);

    const rule = Rule{
        .name = name,
        .is_departure = is_departure,
        .left = Range{
            .min = left_min,
            .max = left_max,
        },
        .right = Range{
            .min = right_min,
            .max = right_max,
        },
    };

    return rule;
}

fn parseTicket(allocator: std.mem.Allocator, line: []const u8) !Ticket {
    var ticket = std.ArrayList(usize).init(allocator);
    errdefer ticket.deinit();

    var numbers = std.mem.split(u8, line, ",");
    while (numbers.next()) |string| {
        var number = try std.fmt.parseInt(usize, string, 10);

        try ticket.append(number);
    }

    return Ticket{
        .numbers = ticket.toOwnedSlice(),
    };
}

fn part1(input: *Input) Answer {
    var error_rate: usize = 0;
    for (input.nearby) |*ticket| {
        error_rate += input.validate(ticket);
    }
    return error_rate;
}

fn part2(input: Input) Answer {
    for (input.rules) |*rule| {
        var i: usize = 0;
        while (i < FIELDS) : (i += 1) {
            input.validateRuleForColumn(rule, i);
        }
    }

    var finalCount: usize = 0;
    var columns: [FIELDS]?Rule = [_]?Rule{null} ** FIELDS;
    while (finalCount < FIELDS) {
        for (input.rules) |*rule| {
            if (rule.findFinalColumn()) |final| {
                // Found a final column!
                columns[final] = rule.*;

                // No other columns can be for this rule
                for (input.rules) |*other_rule| {
                    if (rule != other_rule) {
                        other_rule.possibleColumns[final] = false;
                    }
                }

                // Increment the count so we eventually finish the loop
                finalCount += 1;

                break;
            }
        }
    }

    // Now find the fields on our ticket that start with departure
    var product: usize = 1;
    for (columns) |rule, i| {
        if (rule.?.is_departure) {
            const value = input.ticket.numbers[i];
            product *= value;
        }
    }
    return product;
}

test "example 1" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/test_day16.txt");
    var input = try parseInput(alloc, test_input);
    defer input.deinit();

    try expect(part1(&input) == 71);
}

test "example 2" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/test_day16_2.txt");
    var input = try parseInput(alloc, test_input);
    defer input.deinit();

    try expect(part1(&input) == 0);
    // try expect(part2(input) == 1);
}

test "answers" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/day16.txt");
    var input = try parseInput(alloc, test_input);
    defer input.deinit();

    try expect(part1(&input) == 23044);
    try expect(part2(input) == 3765150732757);
}
