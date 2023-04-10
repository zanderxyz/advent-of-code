const std = @import("std");
const print = std.log.info;
const expect = std.testing.expect;

const INPUT_FILE = @embedFile("inputs/day22.txt");

const Answer = usize;

const MAX_STACK = 1000;

const Input = struct {
    allocator: std.mem.Allocator,
    cards1: []Card,
    cards2: []Card,

    fn init(allocator: std.mem.Allocator, cards1: []Card, cards2: []Card) Input {
        return Input{
            .allocator = allocator,
            .cards1 = cards1,
            .cards2 = cards2,
        };
    }

    fn deinit(self: *Input) void {
        self.allocator.free(self.cards1);
        self.allocator.free(self.cards2);
    }
};

const Player = enum {
    one,
    two,
};

fn Game(comptime MAX: usize) type {
    return struct {
        allocator: std.mem.Allocator,
        my_deck: Deck(MAX),
        your_deck: Deck(MAX),

        const Mode = enum {
            standard,
            recursive,
        };

        const Self = @This();

        fn new(input: Input) Self {
            return Self{
                .allocator = input.allocator,
                .my_deck = Deck(MAX).new(input.cards1),
                .your_deck = Deck(MAX).new(input.cards2),
            };
        }

        fn newWithDecks(allocator: std.mem.Allocator, my_deck: Deck(MAX), your_deck: Deck(MAX)) Self {
            return Self{
                .allocator = allocator,
                .my_deck = my_deck,
                .your_deck = your_deck,
            };
        }

        fn state(self: Self) []u8 {
            const deck1 = self.my_deck.state(self.allocator);
            defer self.allocator.free(deck1);

            const deck2 = self.your_deck.state(self.allocator);
            defer self.allocator.free(deck2);

            var states: [2][]u8 = [_][]u8{ deck1, deck2 };
            return std.mem.join(self.allocator, "   ", &states) catch unreachable;
        }

        fn play(self: *Self, mode: Mode, game_num: usize) Player {
            // Keep a record of game states
            var visited = std.StringHashMap(void).init(self.allocator);
            defer {
                var iterator = visited.iterator();
                while (iterator.next()) |entry| {
                    self.allocator.free(entry.key_ptr.*);
                }
                visited.deinit();
            }

            var game_winner: Player = undefined;

            while (true) {
                // If either player's deck is empty, they lose
                if (self.my_deck.depth() == 0) {
                    game_winner = .two;
                    break;
                }
                if (self.your_deck.depth() == 0) {
                    game_winner = .one;
                    break;
                }

                if (mode == .recursive) {
                    // If this state has been visited before, player 1 wins
                    const current = self.state();
                    if (visited.contains(current)) {
                        defer self.allocator.free(current);

                        game_winner = .one;
                        break;
                    }
                    visited.put(current, {}) catch unreachable;
                }

                const card1 = self.my_deck.draw().?;
                const card2 = self.your_deck.draw().?;

                var winner: Player = undefined;
                if (mode == .recursive and card1 <= self.my_deck.depth() and card2 <= self.your_deck.depth()) {
                    var my_deck_copy = self.my_deck.copy(card1);
                    var your_deck_copy = self.your_deck.copy(card2);

                    if (my_deck_copy.max() > your_deck_copy.max()) {
                        winner = .one;
                    } else {
                        var game = Game(MAX).newWithDecks(self.allocator, my_deck_copy, your_deck_copy);
                        winner = game.play(mode, game_num + 1);
                    }
                } else {
                    winner = if (card1 > card2) .one else .two;
                }

                self.settleRound(winner, card1, card2);
            }

            return game_winner;
        }

        fn settleRound(self: *Self, winner: Player, card1: Card, card2: Card) void {
            if (winner == .one) {
                // Player 1 wins
                self.my_deck.push(card1);
                self.my_deck.push(card2);
            } else {
                // Player 2 wins
                self.your_deck.push(card2);
                self.your_deck.push(card1);
            }
        }

        fn score(self: Self) usize {
            const deck = switch (self.my_deck.depth()) {
                0 => self.your_deck,
                else => self.my_deck,
            };

            return deck.score();
        }
    };
}

pub fn main() !void {
    var alloc = std.heap.GeneralPurposeAllocator(.{}){};

    var input = try parseInput(alloc.allocator(), INPUT_FILE);
    defer input.deinit();

    print("Part 1: {}\n", .{part1(input)});
    print("Part 2: {}\n", .{part2(input)});
}

fn parseInput(allocator: std.mem.Allocator, input: []const u8) !Input {
    var cards1 = std.ArrayList(Card).init(allocator);
    errdefer cards1.deinit();

    var cards2 = std.ArrayList(Card).init(allocator);
    errdefer cards2.deinit();

    var player = Player.one;

    var lines = std.mem.tokenize(u8, input, "\n");
    _ = lines.next(); // Skip the first player number

    while (lines.next()) |line| {
        const number = std.fmt.parseInt(Card, line, 10) catch {
            player = .two;

            continue;
        };

        switch (player) {
            .one => try cards1.append(number),
            .two => try cards2.append(number),
        }
    }

    return Input.init(allocator, cards1.toOwnedSlice(), cards2.toOwnedSlice());
}

const Card = u8;

fn Deck(comptime MAX: usize) type {
    return struct {
        cards: [MAX]Card = undefined,
        top: usize = 0,
        bottom: usize = 0,

        const Self = @This();

        fn new(cards: []const Card) Self {
            var deck = Self{};
            deck.add(cards);
            return deck;
        }

        fn add(self: *Self, cards: []const Card) void {
            for (cards) |card| {
                self.push(card);
            }
        }

        fn copy(self: Self, up_to: Card) Self {
            var cards: []const Card = self.cards[self.top .. self.top + up_to];
            return Self.new(cards);
        }

        fn max(self: Self) Card {
            return std.mem.max(Card, &self.cards);
        }

        fn state(self: Self, allocator: std.mem.Allocator) []u8 {
            const cards = self.cards[self.top..self.bottom];
            const string: [1][]const u8 = [_][]const u8{cards};
            return std.mem.join(allocator, " ", &string) catch unreachable;
        }

        fn depth(self: Self) usize {
            return self.bottom - self.top;
        }

        fn draw(self: *Self) ?Card {
            if (self.depth() == 0) {
                return null;
            } else {
                const card = self.cards[self.top];
                self.top += 1;
                return card;
            }
        }

        fn push(self: *Self, item: Card) void {
            self.cards[self.bottom] = item;
            self.bottom += 1;
        }

        fn show(self: Self, player: usize) void {
            print("Deck {}: ({} {} {}) ", .{ player, self.top, self.bottom, self.depth() });
            var i: usize = 0;
            const end = self.depth();
            while (i < end) : (i += 1) {
                print("{} ", .{self.cards[self.top + i]});
            }
            print("\n", .{});
        }

        fn score(self: Self) usize {
            var s: usize = 0;
            var k: usize = 1;
            while (k <= self.depth()) {
                const card = self.cards[self.bottom - k];
                s += k * card;

                k += 1;
            }

            return s;
        }
    };
}

test "deck" {
    var deck = Deck(MAX_STACK){};
    var cards: [3]Card = [_]Card{ 1, 2, 3 };
    deck.add(&cards);

    try expect(deck.depth() == 3);
    try expect(deck.top == 0);
    try expect(deck.bottom == 3);
    try expect(deck.cards[0] == 1);
    try expect(deck.cards[1] == 2);
    try expect(deck.cards[2] == 3);

    try expect(deck.draw().? == 1);
    try expect(deck.depth() == 2);
    try expect(deck.top == 1);
    try expect(deck.bottom == 3);

    try expect(deck.draw().? == 2);
    try expect(deck.depth() == 1);
    try expect(deck.top == 2);
    try expect(deck.bottom == 3);

    try expect(deck.draw().? == 3);
    try expect(deck.depth() == 0);
    try expect(deck.top == 3);
    try expect(deck.bottom == 3);

    try expect(deck.draw() == null);
    try expect(deck.depth() == 0);

    deck.push(4);
    try expect(deck.depth() == 1);
    try expect(deck.top == 3);
    try expect(deck.bottom == 4);
    try expect(deck.cards[3] == 4);

    try expect(deck.draw().? == 4);
    try expect(deck.top == 4);
    try expect(deck.bottom == 4);

    deck.push(5);
    deck.push(6);
    try expect(deck.depth() == 2);
    try expect(deck.top == 4);
    try expect(deck.bottom == 6);
    try expect(deck.cards[4] == 5);
    try expect(deck.cards[5] == 6);

    try expect(deck.score() == 16);
}

fn part1(input: Input) Answer {
    var game = Game(MAX_STACK).new(input);
    _ = game.play(.standard, 1);
    return game.score();
}

fn part2(input: Input) Answer {
    var game = Game(MAX_STACK * 10).new(input);
    _ = game.play(.recursive, 1);
    return game.score();
}

test "examples" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/test_day22.txt");
    var input = try parseInput(alloc, test_input);
    defer input.deinit();

    try expect(part1(input) == 306);
    try expect(part2(input) == 291);
}

test "example infinite" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/test_day22_2.txt");
    var input = try parseInput(alloc, test_input);
    defer input.deinit();

    var game = Game(MAX_STACK).new(input);
    try expect(game.play(.recursive, 1) == .one);
}

test "answers" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/day22.txt");
    var input = try parseInput(alloc, test_input);
    defer input.deinit();

    try expect(part1(input) == 33010);
    try expect(part2(input) == 32769);
}
