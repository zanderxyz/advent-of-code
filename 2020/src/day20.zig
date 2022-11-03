// TODO: Rewrite using a single slice rather than a slice of slices

const std = @import("std");
const print = std.log.info;
const expect = std.testing.expect;

const INPUT_FILE = @embedFile("inputs/day20.txt");

const N = 10; // Tile width

const monster = [_][]const u8{
    "                  # ",
    "#    ##    ##    ###",
    " #  #  #  #  #  #   ",
};

const Answer = usize;
const Tile = struct {
    allocator: std.mem.Allocator,
    id: usize,
    data: [][]bool,
    edges: [][]bool,

    fn init(allocator: std.mem.Allocator, id: usize, data: [][]bool) Tile {
        return Tile{
            .allocator = allocator,
            .id = id,
            .data = data,
            .edges = initEdges(allocator, data),
        };
    }

    fn deinit(self: *Tile) void {
        self.deinitData();
        self.deinitEdges();
    }

    fn deinitData(self: *Tile) void {
        // for (self.data) |row| {
        //     self.allocator.free(row);
        // }
        self.allocator.free(self.data);
    }

    fn deinitEdges(self: *Tile) void {
        for (self.edges) |e| {
            self.allocator.free(e);
        }
        self.allocator.free(self.edges);
    }

    fn show(self: Tile) void {
        print("Tile {}\n", .{self.id});
        for (self.data) |row| {
            showRow(row);
        }
        print("\n", .{});
    }

    fn showRow(edge: []bool) void {
        for (edge) |b| {
            const x: []const u8 = if (b) "#" else ".";
            print("{s}", .{x});
        }
        print("\n", .{});
    }

    fn rotateRight(self: *Tile) void {
        rotateRightData(self.allocator, &self.data);
    }

    fn flipVertically(self: *Tile) void {
        flipVerticallyData(self.allocator, &self.data);
    }

    fn initEdges(allocator: std.mem.Allocator, data: [][]bool) [][]bool {
        var all = std.ArrayList([]bool).init(allocator);

        var top = std.ArrayList(bool).init(allocator);
        var bottom = std.ArrayList(bool).init(allocator);
        var left = std.ArrayList(bool).init(allocator);
        var right = std.ArrayList(bool).init(allocator);
        var top2 = std.ArrayList(bool).init(allocator);
        var bottom2 = std.ArrayList(bool).init(allocator);
        var left2 = std.ArrayList(bool).init(allocator);
        var right2 = std.ArrayList(bool).init(allocator);

        for (data[0]) |col| {
            top.append(col) catch unreachable;
            top2.append(col) catch unreachable;
        }
        for (data) |row| {
            left.append(row[0]) catch unreachable;
            left2.append(row[0]) catch unreachable;
            right.append(row[N - 1]) catch unreachable;
            right2.append(row[N - 1]) catch unreachable;
        }
        for (data[N - 1]) |col| {
            bottom.append(col) catch unreachable;
            bottom2.append(col) catch unreachable;
        }

        std.mem.reverse(bool, top2.items);
        std.mem.reverse(bool, bottom2.items);
        std.mem.reverse(bool, left2.items);
        std.mem.reverse(bool, right2.items);

        all.append(top.toOwnedSlice()) catch unreachable;
        all.append(top2.toOwnedSlice()) catch unreachable;
        all.append(bottom.toOwnedSlice()) catch unreachable;
        all.append(bottom2.toOwnedSlice()) catch unreachable;
        all.append(left.toOwnedSlice()) catch unreachable;
        all.append(left2.toOwnedSlice()) catch unreachable;
        all.append(right.toOwnedSlice()) catch unreachable;
        all.append(right2.toOwnedSlice()) catch unreachable;

        return all.toOwnedSlice();
    }

    fn topEdge(self: Tile) []bool {
        var edge = std.ArrayList(bool).init(self.allocator);

        for (self.data[0]) |col| {
            edge.append(col) catch unreachable;
        }

        return edge.toOwnedSlice();
    }

    fn bottomEdge(self: Tile) []bool {
        var edge = std.ArrayList(bool).init(self.allocator);

        for (self.data[N - 1]) |col| {
            edge.append(col) catch unreachable;
        }

        return edge.toOwnedSlice();
    }

    fn leftEdge(self: Tile) []bool {
        var edge = std.ArrayList(bool).init(self.allocator);

        for (self.data) |row| {
            edge.append(row[0]) catch unreachable;
        }

        return edge.toOwnedSlice();
    }

    fn rightEdge(self: Tile) []bool {
        var edge = std.ArrayList(bool).init(self.allocator);

        for (self.data) |row| {
            edge.append(row[N - 1]) catch unreachable;
        }

        return edge.toOwnedSlice();
    }

    fn matchesExactly(left: Tile, right: Tile) ?Offset {
        const leftTop = left.topEdge();
        defer left.allocator.free(leftTop);
        const leftBottom = left.bottomEdge();
        defer left.allocator.free(leftBottom);
        const leftLeft = left.leftEdge();
        defer left.allocator.free(leftLeft);
        const leftRight = left.rightEdge();
        defer left.allocator.free(leftRight);

        const rightTop = right.topEdge();
        defer right.allocator.free(rightTop);
        const rightBottom = right.bottomEdge();
        defer right.allocator.free(rightBottom);
        const rightLeft = right.leftEdge();
        defer right.allocator.free(rightLeft);
        const rightRight = right.rightEdge();
        defer right.allocator.free(rightRight);

        if (std.mem.eql(bool, leftTop, rightBottom)) {
            return Offset{
                .x = 0,
                .y = -1,
            };
        } else if (std.mem.eql(bool, leftBottom, rightTop)) {
            return Offset{
                .x = 0,
                .y = 1,
            };
        } else if (std.mem.eql(bool, leftLeft, rightRight)) {
            showRow(leftLeft);
            showRow(rightRight);
            return Offset{
                .x = -1,
                .y = 0,
            };
        } else if (std.mem.eql(bool, leftRight, rightLeft)) {
            return Offset{
                .x = 1,
                .y = 0,
            };
        }
        return null;
    }

    fn matchesWith(self: Tile, right: Tile) bool {
        for (self.edges) |l| {
            for (right.edges) |r| {
                if (std.mem.eql(bool, l, r)) {
                    return true;
                }
            }
        }
        return false;
    }
};

fn rotateRightData(allocator: std.mem.Allocator, data: *[][]bool) void {
    var new = std.ArrayList([]bool).init(allocator);

    // Map old (r, c) => new (c, n-r-1)
    // New tile (i,j) = old tile (n-j-1, i)
    const max = data.*.len;
    var i: usize = 0;
    while (i < max) : (i += 1) {
        var new_row = std.ArrayList(bool).init(allocator);
        var j: usize = 0;
        while (j < max) : (j += 1) {
            new_row.append(data.*[max - j - 1][i]) catch unreachable;
        }
        new.append(new_row.toOwnedSlice()) catch unreachable;
    }
    for (data.*) |row| {
        allocator.free(row);
    }

    data.* = new.toOwnedSlice();
}

fn flipVerticallyData(allocator: std.mem.Allocator, data: *[][]bool) void {
    var new = std.ArrayList([]bool).init(allocator);

    // Map old (r, c) => new (c, n-r-1)
    // New tile (i,j) = old tile (n-j-1, i)
    const max = data.*.len;
    var i: usize = 0;
    while (i < max) : (i += 1) {
        var new_row = std.ArrayList(bool).init(allocator);
        var j: usize = 0;
        while (j < max) : (j += 1) {
            new_row.append(data.*[max - i - 1][j]) catch unreachable;
        }
        new.append(new_row.toOwnedSlice()) catch unreachable;
    }

    data.* = new.toOwnedSlice();
}

const Input = struct {
    allocator: std.mem.Allocator,
    tiles: []Tile,

    fn init(allocator: std.mem.Allocator, tiles: []Tile) Input {
        return Input{
            .allocator = allocator,
            .tiles = tiles,
        };
    }

    fn deinit(self: *Input) void {
        for (self.tiles) |*tile| {
            tile.deinit();
        }
        self.allocator.free(self.tiles);
    }
};

pub fn main() !void {
    var alloc = std.heap.GeneralPurposeAllocator(.{}){};
    var arena = std.heap.ArenaAllocator.init(alloc.allocator());
    defer arena.deinit();

    var input = try parseInput(arena.allocator(), INPUT_FILE);
    defer input.deinit();

    print("Part 1: {}\n", .{part1(input)});
    print("Part 2: {}\n", .{part2(input, false)});
}

fn parseInput(allocator: std.mem.Allocator, input: []const u8) !Input {
    var tiles = std.ArrayList(Tile).init(allocator);
    errdefer tiles.deinit();

    var groups = std.mem.split(u8, input, "\n\n");
    while (groups.next()) |group| {
        var tile_data = std.ArrayList([]bool).init(allocator);
        errdefer tile_data.deinit();

        var lines = std.mem.tokenize(u8, group, "\n");
        const title_row = lines.next().?;
        const tile_id_str = title_row[5..9];
        const tile_id = try std.fmt.parseInt(usize, tile_id_str, 10);

        while (lines.next()) |line| {
            if (line.len == 0) continue;
            var tile_row = std.ArrayList(bool).init(allocator);
            errdefer tile_row.deinit();

            for (line) |char| {
                const data = switch (char) {
                    '#' => true,
                    else => false,
                };

                try tile_row.append(data);
            }

            try tile_data.append(tile_row.toOwnedSlice());
        }

        const data = tile_data.toOwnedSlice();
        const tile = Tile.init(allocator, tile_id, data);
        try tiles.append(tile);
    }

    return Input.init(allocator, tiles.toOwnedSlice());
}

fn part1(input: Input) Answer {
    var product: Answer = 1;

    for (input.tiles) |tile, i| {
        var matches: usize = 0;
        for (input.tiles) |tile2, j| {
            if (i == j) continue;
            if (tile.matchesWith(tile2)) {
                matches += 1;
            }
        }
        if (matches == 2) {
            product *= tile.id;
        }
    }

    return product;
}

const TileMap = std.AutoHashMap(usize, Tile);
const Links = std.AutoHashMap(usize, []usize);

const Graph = struct {
    allocator: std.mem.Allocator,
    tiles: TileMap,
    links: std.AutoHashMap(usize, []usize),

    const Self = @This();

    fn init(allocator: std.mem.Allocator, input: Input) !Self {
        var tiles = TileMap.init(allocator);
        var links = Links.init(allocator);

        for (input.tiles) |tile, i| {
            try tiles.put(tile.id, tile);
            var tile_links = std.ArrayList(usize).init(input.allocator);
            errdefer tile_links.deinit();

            for (input.tiles) |tile2, j| {
                if (i == j) continue;

                if (tile.matchesWith(tile2)) {
                    try tile_links.append(tile2.id);
                }
            }

            try links.put(tile.id, tile_links.toOwnedSlice());
        }

        return Self{
            .allocator = allocator,
            .tiles = tiles,
            .links = links,
        };
    }

    fn arrange(self: *Self, grid: *Grid, is_test: bool) !void {
        // BFS through the graph
        // As we find a tile for the first time, rotate it to match
        var visited = std.AutoHashMap(usize, void).init(self.allocator);
        defer visited.deinit();

        var queue = std.ArrayList(usize).init(self.allocator);
        defer queue.deinit();

        var positions = std.AutoHashMap(usize, Position).init(self.allocator);
        defer positions.deinit();

        var corner: usize = switch (is_test) {
            true => 1951,
            false => 2749,
        };

        // print("Starting arrange with {}\n", .{corner});

        var corner_tile = self.tiles.get(corner).?;
        // corner_tile.show();

        // Trial and error:
        if (is_test) {
            corner_tile.rotateRight();
        } else {
            corner_tile.rotateRight();
            corner_tile.rotateRight();
        }
        try self.tiles.put(corner, corner_tile);

        try visited.put(corner, {});
        try queue.append(corner);

        var position = Position{
            .x = 0,
            .y = 0,
        };
        try positions.put(corner, position);
        grid.set(position, corner);

        while (queue.items.len > 0) {
            const tile_id = queue.orderedRemove(0);
            const links = self.links.get(tile_id).?;
            position = positions.get(tile_id).?;
            // print("Found tile {} with {} links and {}\n", .{ tile_id, links.len, position });
            for (links) |linked_tile_id| {
                if (visited.get(linked_tile_id) == null) {
                    // print(" -> Attempting match for new {}\n", .{linked_tile_id});
                    const offset = try self.forceMatch(tile_id, linked_tile_id);
                    // print(" -> Found orientation for {}: {}\n", .{ linked_tile_id, offset });
                    const new_position = position.add(offset);
                    try positions.put(linked_tile_id, new_position);
                    grid.set(new_position, linked_tile_id);
                    // print(" -> Found position for {}: {}\n", .{ linked_tile_id, new_position });
                    try queue.append(linked_tile_id);
                    try visited.put(linked_tile_id, {});
                    // print(" -> Added {} to queue\n", .{linked_tile_id});
                } else {
                    // print(" -> Already visited {}\n", .{linked_tile_id});
                }
            }
        }
    }

    fn forceMatch(self: *Self, left_id: usize, right_id: usize) !Offset {
        const left = self.tiles.get(left_id).?;
        // left.show();
        var right = self.tiles.get(right_id).?;
        return self.forceMatchTiles(left, &right);
    }

    fn forceMatchTiles(self: *Self, left: Tile, right: *Tile) !Offset {
        var tile = right.*;
        if (left.matchesExactly(tile)) |p| {
            return p;
        }

        tile.flipVertically();
        if (left.matchesExactly(tile)) |p| {
            try self.tiles.put(tile.id, tile);
            return p;
        }
        tile.flipVertically();

        tile.rotateRight();
        if (left.matchesExactly(tile)) |p| {
            try self.tiles.put(tile.id, tile);
            return p;
        }

        tile.flipVertically();
        if (left.matchesExactly(tile)) |p| {
            try self.tiles.put(tile.id, tile);
            return p;
        }
        tile.flipVertically();

        tile.rotateRight();
        if (left.matchesExactly(tile)) |p| {
            try self.tiles.put(tile.id, tile);
            return p;
        }

        tile.flipVertically();
        if (left.matchesExactly(tile)) |p| {
            try self.tiles.put(tile.id, tile);
            return p;
        }
        tile.flipVertically();

        tile.rotateRight();
        if (left.matchesExactly(tile)) |p| {
            try self.tiles.put(tile.id, tile);
            return p;
        }

        tile.flipVertically();
        if (left.matchesExactly(tile)) |p| {
            try self.tiles.put(tile.id, tile);
            return p;
        }

        @panic("Failed to match tiles");
    }

    fn deinit(self: *Self) void {
        var iterator = self.links.iterator();
        while (iterator.next()) |entry| {
            self.allocator.free(entry.value_ptr.*);
        }
        self.tiles.deinit();
        self.links.deinit();
    }
};

const Position = struct {
    x: usize,
    y: usize,

    fn add(self: Position, o: Offset) Position {
        const new_x = @intCast(isize, self.x) + o.x;
        const new_y = @intCast(isize, self.y) + o.y;
        if (new_x >= 0 and new_y >= 0) {
            return Position{
                .x = @intCast(usize, new_x),
                .y = @intCast(usize, new_y),
            };
        }
        unreachable;
    }
};

const Offset = struct {
    x: isize,
    y: isize,
};

const Grid = struct {
    allocator: std.mem.Allocator,
    grid: [][]?usize,

    const Self = @This();

    fn init(allocator: std.mem.Allocator, input: Input) Self {
        const width = @intCast(u32, std.math.sqrt(input.tiles.len));
        var grid = std.ArrayList([]?usize).init(allocator);
        var i: usize = 0;
        while (i < width) : (i += 1) {
            var row = std.ArrayList(?usize).init(allocator);
            var j: usize = 0;
            while (j < width) : (j += 1) {
                row.append(null) catch unreachable;
            }
            grid.append(row.toOwnedSlice()) catch unreachable;
        }
        return Self{
            .allocator = allocator,
            .grid = grid.toOwnedSlice(),
        };
    }

    fn deinit(self: *Self) void {
        for (self.grid) |row| {
            self.allocator.free(row);
        }
        self.allocator.free(self.grid);
    }

    fn show(self: Self) void {
        print("Grid\n", .{});
        for (self.grid) |row| {
            for (row) |item| {
                print(" {} ", .{item});
            }
            print("\n", .{});
        }
    }

    fn set(self: *Self, p: Position, id: usize) void {
        if (self.grid[p.x][p.y] == null) {
            self.grid[p.x][p.y] = id;
        } else {
            unreachable;
        }
    }
};

const Image = struct {
    allocator: std.mem.Allocator,
    data: [][]bool,

    const Self = @This();

    fn init(allocator: std.mem.Allocator, tiles: TileMap, grid: Grid) Self {
        var width = (N - 2) * grid.grid.len;
        var data = std.ArrayList([]bool).init(allocator);
        var i: usize = 0;
        var tile_id: usize = undefined;
        var tile: Tile = undefined;
        // i,j is a position in the image
        while (i < width) : (i += 1) {
            var row = std.ArrayList(bool).init(allocator);
            var j: usize = 0;
            while (j < width) : (j += 1) {
                const x: usize = j / (N - 2);
                const y: usize = i / (N - 2);
                // Inside a single cell, copy across data
                tile_id = grid.grid[x][y].?;
                // print("i {} j {} x {} y {} Tile id {}\n", .{ i, j, x, y, tile_id });
                tile = tiles.get(tile_id).?;

                row.append(tile.data[i % (N - 2) + 1][j % (N - 2) + 1]) catch unreachable;
            }
            data.append(row.toOwnedSlice()) catch unreachable;
        }
        return Self{
            .allocator = allocator,
            .data = data.toOwnedSlice(),
        };
    }

    fn show(self: Self) void {
        print("Image\n", .{});
        for (self.data) |row| {
            Tile.showRow(row);
        }
        print("\n", .{});
    }

    fn deinit(self: *Self) void {
        for (self.data) |row| {
            self.allocator.free(row);
        }
        self.allocator.free(self.data);
    }

    fn countTrue(self: Self) usize {
        var count: usize = 0;
        var i: usize = 0;
        while (i < self.data.len) : (i += 1) {
            var j: usize = 0;
            while (j < self.data.len) : (j += 1) {
                if (self.data[i][j]) {
                    count += 1;
                }
            }
        }

        return count;
    }

    fn countMonsters(self: Self) usize {
        var y: usize = 0;
        var count: usize = 0;
        while (y < self.data.len - monster.len) : (y += 1) {
            var x: usize = 0;
            next: while (x < self.data.len - monster[0].len) : (x += 1) {
                var sy: usize = 0;
                while (sy < monster.len) : (sy += 1) {
                    var sx: usize = 0;
                    while (sx < monster[0].len) : (sx += 1) {
                        if (monster[sy][sx] == '#' and !self.data[x + sx][y + sy])
                            continue :next;
                    }
                }
                count += 1;
            }
        }
        return count;
    }
};

fn part2(input: Input, is_test: bool) Answer {
    var graph = Graph.init(input.allocator, input) catch unreachable;
    defer graph.deinit();

    // Arrange into a grid
    var grid = Grid.init(input.allocator, input);
    defer grid.deinit();

    graph.arrange(&grid, is_test) catch unreachable;

    // grid.show();

    // Combine to an image
    var image = Image.init(input.allocator, graph.tiles, grid);
    defer image.deinit();

    // image.show();

    var monsters: usize = image.countMonsters();

    // Trial and error:
    if (is_test) {
        flipVerticallyData(image.allocator, &image.data);
    } else {
        flipVerticallyData(image.allocator, &image.data);
    }
    while (monsters == 0) {
        rotateRightData(image.allocator, &image.data);
        monsters = image.countMonsters();
    }

    var cells: usize = image.countTrue();
    // print("Cells: {}\n", .{cells});
    // print("Monsters: {}\n", .{monsters});
    const roughness = cells - 15 * monsters;
    // print("Roughness: {}\n", .{roughness});

    return roughness;
}

test "examples" {
    var alloc = std.testing.allocator;
    var arena = std.heap.ArenaAllocator.init(alloc);
    defer arena.deinit();

    const test_input = @embedFile("inputs/test_day20.txt");
    var input = try parseInput(arena.allocator(), test_input);
    defer input.deinit();

    try expect(part1(input) == 20899048083289);
    try expect(part2(input, true) == 273);
}

test "answers" {
    var alloc = std.testing.allocator;
    var arena = std.heap.ArenaAllocator.init(alloc);
    defer arena.deinit();

    const test_input = @embedFile("inputs/day20.txt");
    var input = try parseInput(arena.allocator(), test_input);
    defer input.deinit();

    try expect(part1(input) == 11788777383197);
    try expect(part2(input, false) == 2242);
}
