const std = @import("std");
const print = std.log.info;
const expect = std.testing.expect;

const INPUT_FILE = @embedFile("inputs/day21.txt");

const Answer = usize;

const Ingredient = []const u8;
const Allergen = []const u8;

const Recipe = struct {
    ingredients: []Ingredient,
    allergens: []Allergen,
};

const Input = struct {
    allocator: std.mem.Allocator,
    recipes: []Recipe,

    fn init(allocator: std.mem.Allocator, recipes: []Recipe) Input {
        return Input{
            .allocator = allocator,
            .recipes = recipes,
        };
    }

    fn deinit(self: *Input) void {
        for (self.recipes) |recipe| {
            self.allocator.free(recipe.ingredients);
            self.allocator.free(recipe.allergens);
        }
        self.allocator.free(self.recipes);
    }
};

const Possibles = std.StringHashMap(Allergens); // Key = Ingredient
const IngredientCount = std.StringHashMap(usize); // Key = Ingredient
const Allergens = std.StringHashMap(void); // Key = Allergen
const Solution = std.StringHashMap(Allergen); // Key = Ingredient

const Logic = struct {
    allocator: std.mem.Allocator,
    possibles: Possibles,
    ingredients: IngredientCount,
    solution: Solution,

    const Self = @This();

    fn init(allocator: std.mem.Allocator, input: Input) !Self {
        var ingredients = IngredientCount.init(allocator);
        errdefer ingredients.deinit();

        var allergens = Allergens.init(allocator);
        defer allergens.deinit();

        var possibles = Possibles.init(allocator);
        errdefer possibles.deinit();

        for (input.recipes) |recipe| {
            for (recipe.allergens) |allergen| {
                try allergens.put(allergen, {});
            }

            for (recipe.ingredients) |ingredient| {
                const current = ingredients.get(ingredient) orelse 0;
                try ingredients.put(ingredient, current + 1);
            }
        }

        // Initially assume any allergen is possible for any ingredient
        var ingredient_iterator = ingredients.iterator();
        while (ingredient_iterator.next()) |entry| {
            const ingredient = entry.key_ptr.*;
            try possibles.put(ingredient, try allergens.clone());
        }

        // We can infer that for any allergen, any ingredient missing from the list is definitely not a match
        for (input.recipes) |recipe| {
            for (recipe.allergens) |allergen| {
                var ingredient_iter = ingredients.iterator();
                next: while (ingredient_iter.next()) |entry| {
                    const ingredient = entry.key_ptr.*;

                    // Check if the ingredient is in the recipe
                    for (recipe.ingredients) |recipe_ingr| {
                        if (std.mem.eql(u8, recipe_ingr, ingredient)) {
                            continue :next;
                        }
                    }

                    // If ingredient is not in the recipe, remove this allergen from the possibles
                    var poss_allergens = possibles.get(ingredient).?;
                    _ = poss_allergens.remove(allergen);
                    try possibles.put(ingredient, poss_allergens);
                }
            }
        }

        // Now according to part B we have enough information to solve it, so we can build a mapping from ingredients to allergens
        var solution = Solution.init(allocator);
        errdefer solution.deinit();

        var visited = Allergens.init(allocator);
        defer visited.deinit();

        const allergen_count = allergens.count();
        // Stop when our solution has every possible allergen
        while (solution.count() < allergen_count) {
            var ingredient_iterator2 = ingredients.iterator();
            while (ingredient_iterator2.next()) |entry| {
                const ingredient = entry.key_ptr.*;

                // Fetch possible allergens for this ingredient
                var poss_allergens = possibles.get(ingredient).?;
                if (poss_allergens.count() > 0) {
                    var count: usize = 0;
                    var found: Allergen = undefined;
                    var poss_iter = poss_allergens.iterator();
                    while (poss_iter.next()) |allergen_entry| {
                        const allergen = allergen_entry.key_ptr.*;
                        // Skip anything we have already solved
                        if (visited.get(allergen) == null) {
                            count += 1;
                            found = allergen;
                        }
                    }

                    // Now if only one is possible, it is solved
                    if (count == 1) {
                        try solution.put(ingredient, found);
                        try visited.put(found, {});
                    }
                }
            }
        }

        return Self{
            .allocator = allocator,
            .ingredients = ingredients,
            .possibles = possibles,
            .solution = solution,
        };
    }

    fn deinit(self: *Self) void {
        self.ingredients.deinit();
        var iterator = self.possibles.iterator();
        while (iterator.next()) |entry| {
            entry.value_ptr.*.deinit();
        }
        self.possibles.deinit();
        self.solution.deinit();
    }

    fn countSafe(self: Self) usize {
        var count: usize = 0;
        var iterator = self.ingredients.iterator();
        while (iterator.next()) |entry| {
            const ingredient = entry.key_ptr.*;
            const possible_allergens = self.possibles.get(ingredient).?.count();
            if (possible_allergens == 0) {
                count += entry.value_ptr.*;
            }
        }
        return count;
    }
};

pub fn main() !void {
    var alloc = std.heap.GeneralPurposeAllocator(.{}){};

    var input = try parseInput(alloc.allocator(), INPUT_FILE);
    defer input.deinit();

    print("Part 1: {}\n", .{part1(input)});
    print("Part 2: {s}\n", .{part2(input)});
}

fn parseInput(allocator: std.mem.Allocator, input: []const u8) !Input {
    var recipes = std.ArrayList(Recipe).init(allocator);
    errdefer recipes.deinit();

    var lines = std.mem.tokenize(u8, input, "\n");
    while (lines.next()) |line| {
        var ingredients = std.ArrayList(Ingredient).init(allocator);
        errdefer ingredients.deinit();

        var allergens = std.ArrayList(Allergen).init(allocator);
        errdefer allergens.deinit();

        var split = std.mem.split(u8, line[0 .. line.len - 1], " (contains ");
        const ingredient_str = split.next().?;
        const allergen_str = split.next().?;

        var ingredient_iter = std.mem.split(u8, ingredient_str, " ");
        while (ingredient_iter.next()) |ingredient| {
            try ingredients.append(ingredient);
        }

        var allergen_iter = std.mem.split(u8, allergen_str, ", ");
        while (allergen_iter.next()) |allergen| {
            try allergens.append(allergen);
        }

        const recipe = Recipe{
            .ingredients = ingredients.toOwnedSlice(),
            .allergens = allergens.toOwnedSlice(),
        };

        try recipes.append(recipe);
    }

    return Input.init(allocator, recipes.toOwnedSlice());
}

fn part1(input: Input) Answer {
    var logic = Logic.init(input.allocator, input) catch unreachable;
    defer logic.deinit();

    return logic.countSafe();
}

fn part2(input: Input) []const u8 {
    var logic = Logic.init(input.allocator, input) catch unreachable;
    defer logic.deinit();

    // We have a solution mapping ingredients to allergens
    // Turn it into an array so we can sort it
    var answers = std.ArrayList(Pair).init(input.allocator);
    defer answers.deinit();

    var solution_iter = logic.solution.iterator();
    while (solution_iter.next()) |entry| {
        const pair = Pair{
            .left = entry.key_ptr.*, // ingredient
            .right = entry.value_ptr.*, // allergen
        };
        answers.append(pair) catch unreachable;
    }

    // Sort by allergens
    std.sort.sort(Pair, answers.items, {}, comptime lexographicByAllergen(u8));

    var ingredients = std.ArrayList(Ingredient).init(input.allocator);

    // Create a slice of ingredients, in order
    for (answers.items) |pair| {
        ingredients.append(pair.left) catch unreachable;
    }
    const ingredients_slice = ingredients.toOwnedSlice();
    defer input.allocator.free(ingredients_slice);

    // Join to the answer
    var answer = std.mem.join(input.allocator, ",", ingredients_slice) catch unreachable;
    return answer;
}

const Pair = struct {
    left: []const u8,
    right: []const u8,
};

fn lexographicByAllergen(comptime T: type) fn (void, Pair, Pair) bool {
    return struct {
        fn impl(_: void, a: Pair, b: Pair) bool {
            return std.mem.order(T, a.right, b.right).compare(.lt);
        }
    }.impl;
}

test "examples" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/test_day21.txt");
    var input = try parseInput(alloc, test_input);
    defer input.deinit();

    try expect(part1(input) == 5);

    const answer2 = part2(input);
    defer alloc.free(answer2);
    try expect(std.mem.eql(u8, answer2, "mxmxvkd,sqjhc,fvjkl"));
}

test "answers" {
    var alloc = std.testing.allocator;
    const test_input = @embedFile("inputs/day21.txt");
    var input = try parseInput(alloc, test_input);
    defer input.deinit();

    try expect(part1(input) == 2075);

    const answer2 = part2(input);
    defer alloc.free(answer2);
    try expect(std.mem.eql(u8, answer2, "zfcqk,mdtvbb,ggdbl,frpvd,mgczn,zsfzq,kdqls,kktsjbh"));
}
