const std = @import("std");
const print = std.log.info;
const expect = std.testing.expect;

const INPUT_FILE = @embedFile("inputs/day04.txt");
const VALID_EYE_COLORS = [_][]const u8{ "amb", "blu", "brn", "gry", "grn", "hzl", "oth" };

const Answer = usize;
const Input = std.ArrayList(Passport);

const Height = struct {
    unit: []const u8,
    amount: usize,
};

const Passport = struct {
    birth_year: ?usize,
    issue_year: ?usize,
    expiration_year: ?usize,
    height: ?Height,
    hair_color: ?[]const u8,
    eye_color: ?[]const u8,
    passport_id: ?[]const u8,
    country_id: ?[]const u8,
};

pub fn main() !void {
    var alloc = std.heap.GeneralPurposeAllocator(.{}){};

    const input = try parseInput(alloc.allocator(), INPUT_FILE);
    defer input.deinit();

    print("Part 1: {}\n", .{part1(input)});
    print("Part 2: {}\n", .{part2(input)});
}

fn parseInput(allocator: std.mem.Allocator, input: []const u8) !Input {
    var result = Input.init(allocator);
    errdefer result.deinit();

    var lines = std.mem.split(u8, input, "\n\n");
    while (lines.next()) |line| {
        const value = try parseLine(line);
        try result.append(value);
    }

    return result;
}

// Example:
// ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
// byr:1937 iyr:2017 cid:147 hgt:183cm
fn parseLine(line: []const u8) !Passport {
    var birth_year: ?usize = null;
    var issue_year: ?usize = null;
    var expiration_year: ?usize = null;
    var height: ?Height = null;
    var hair_color: ?[]const u8 = null;
    var eye_color: ?[]const u8 = null;
    var passport_id: ?[]const u8 = null;
    var country_id: ?[]const u8 = null;

    var parts = std.mem.tokenize(u8, line, " \n");

    while (parts.next()) |part| {
        var field_and_value = std.mem.split(u8, part, ":");
        const field = field_and_value.next().?;
        const value = field_and_value.next().?;

        if (std.mem.eql(u8, field, "byr")) {
            birth_year = std.fmt.parseInt(usize, value, 10) catch unreachable;
        } else if (std.mem.eql(u8, field, "iyr")) {
            issue_year = std.fmt.parseInt(usize, value, 10) catch unreachable;
        } else if (std.mem.eql(u8, field, "eyr")) {
            expiration_year = std.fmt.parseInt(usize, value, 10) catch unreachable;
        } else if (std.mem.eql(u8, field, "hgt")) {
            const unit = value[value.len - 2 ..];
            const amount = std.fmt.parseInt(usize, value[0 .. value.len - 2], 10) catch 0;

            height = Height{
                .unit = unit,
                .amount = amount,
            };
        } else if (std.mem.eql(u8, field, "hcl")) {
            hair_color = value;
        } else if (std.mem.eql(u8, field, "ecl")) {
            eye_color = value;
        } else if (std.mem.eql(u8, field, "pid")) {
            passport_id = value;
        } else if (std.mem.eql(u8, field, "cid")) {
            country_id = value;
        }
    }

    return Passport{
        .birth_year = birth_year,
        .issue_year = issue_year,
        .expiration_year = expiration_year,
        .height = height,
        .hair_color = hair_color,
        .eye_color = eye_color,
        .passport_id = passport_id,
        .country_id = country_id,
    };
}

fn part1(input: Input) Answer {
    var valid: Answer = 0;
    for (input.items) |passport| {
        if (isValidPart1(passport)) {
            valid += 1;
        }
    }
    return valid;
}

fn isValidPart1(p: Passport) bool {
    return p.birth_year != null and p.issue_year != null and p.expiration_year != null and p.height != null and p.hair_color != null and p.eye_color != null and p.passport_id != null;
}

fn isValidPart2(p: Passport) bool {
    return isValidPart1(p) and p.birth_year.? >= 1920 and p.birth_year.? <= 2002 and p.issue_year.? >= 2010 and p.issue_year.? <= 2020 and p.expiration_year.? >= 2020 and p.expiration_year.? <= 2030 and isValidHeight(p.height.?) and isValidHairColor(p.hair_color.?) and isValidEyeColor(p.eye_color.?) and isValidPassportId(p.passport_id.?);
}

fn isValidHeight(height: Height) bool {
    const value = height.amount;
    if (std.mem.eql(u8, height.unit, "cm")) {
        if (value >= 150 and value <= 193) {
            return true;
        }
    } else if (std.mem.eql(u8, height.unit, "in")) {
        if (value >= 59 and value <= 76) {
            return true;
        }
    }

    return false;
}

fn isValidHairColor(color: []const u8) bool {
    return color.len == 7 and color[0] == '#';
}

fn isValidEyeColor(color: []const u8) bool {
    for (VALID_EYE_COLORS) |valid| {
        if (std.mem.eql(u8, valid, color)) {
            return true;
        }
    }
    return false;
}

fn isValidPassportId(id: []const u8) bool {
    return id.len == 9;
}

fn part2(input: Input) Answer {
    var valid: Answer = 0;
    for (input.items) |passport| {
        if (isValidPart2(passport)) {
            valid += 1;
        }
    }
    return valid;
}

test "example" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/test_day04.txt");
    const input = try parseInput(alloc, test_input);
    defer input.deinit();

    try expect(part1(input) == 2);
}

test "answers" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/day04.txt");
    const input = try parseInput(alloc, test_input);
    defer input.deinit();

    try expect(part1(input) == 170);
    try expect(part2(input) == 103);
}
