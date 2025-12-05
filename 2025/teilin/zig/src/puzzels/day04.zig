const std = @import("std");
const log = std.log;

pub fn solve(allocator: std.mem.Allocator) !void {
    const content = @embedFile("./data/day04.txt");

    const p1 = try part1(allocator, content);
    const p2 = try part2(allocator, content);

    std.debug.print("Part 1 -> {d}\n", .{p1});
    std.debug.print("Part 2 -> {d}", .{p2});
}

fn part1(allocator: std.mem.Allocator, input: []const u8) !u64 {
    var map: std.ArrayListUnmanaged([]bool) = .empty;
    defer {
        for (map.items) |row| {
            allocator.free(row);
        }
        map.deinit(allocator);
    }

    var it = std.mem.tokenizeScalar(u8, input, '\n');
    while (it.next()) |line| {
        var row: std.ArrayListUnmanaged(bool) = .empty;
        errdefer row.deinit(allocator);

        for (line) |c| {
            switch (c) {
                '@' => try row.append(allocator, true),
                '.' => try row.append(allocator, false),
                else => {
                    std.debug.print("Unexpected character '{c}'. Assuming empty space\n", .{c});
                    try row.append(allocator, false);
                },
            }
        }

        try map.append(allocator, try row.toOwnedSlice(allocator));
    }

    var accessible_rolls: u64 = 0;
    for (0..map.items.len) |y| {
        for (0..map.items[0].len) |x| {
            if (!map.items[y][x]) continue;

            if (count_neighbours(map.items, x, y) < 4) {
                accessible_rolls += 1;
            }
        }
    }

    return accessible_rolls;
}

fn part2(allocator: std.mem.Allocator, input: []const u8) !u64 {
    var map: std.ArrayListUnmanaged([]bool) = .empty;
    defer {
        for (map.items) |row| {
            allocator.free(row);
        }
        map.deinit(allocator);
    }

    var it = std.mem.tokenizeScalar(u8, input, '\n');
    while (it.next()) |line| {
        var row: std.ArrayListUnmanaged(bool) = .empty;
        errdefer row.deinit(allocator);

        for (line) |c| {
            switch (c) {
                '@' => try row.append(allocator, true),
                '.' => try row.append(allocator, false),
                else => {
                    std.debug.print("Unexpected character '{c}'. Assuming empty space\n", .{c});
                    try row.append(allocator, false);
                },
            }
        }

        try map.append(allocator, try row.toOwnedSlice(allocator));
    }

    var total_accessible_rolls: u64 = 0;

    while (true) {
        var accessible_rolls: u64 = 0;
        for (0..map.items.len) |y| {
            for (0..map.items[0].len) |x| {
                if (!map.items[y][x]) continue;

                if (count_neighbours(map.items, x, y) < 4) {
                    accessible_rolls += 1;
                    map.items[y][x] = false;
                }
            }
        }

        if (accessible_rolls == 0) break;

        total_accessible_rolls += accessible_rolls;
    }

    return total_accessible_rolls;
}

fn count_neighbours(map: [][]bool, x: usize, y: usize) u8 {
    var neighbours: u8 = 0;

    const min_y = if (y == 0) 0 else y - 1;
    const min_x = if (x == 0) 0 else x - 1;
    const max_y = @min(y + 2, map.len); // Plus two because for is not inclusive for the upper bound
    const max_x = @min(x + 2, map[0].len);

    for (min_y..max_y) |y_idx| {
        for (min_x..max_x) |x_idx| {
            if (map[y_idx][x_idx]) {
                neighbours += 1;
            }
        }
    }

    if (map[y][x]) {
        neighbours -= 1;
    }

    return neighbours;
}

test "test part 1" {
    const sample =
        \\..@@.@@@@.
        \\@@@.@.@.@@
        \\@@@@@.@.@@
        \\@.@@@@..@.
        \\@@.@@@@.@@
        \\.@@@@@@@.@
        \\.@.@.@.@@@
        \\@.@@@.@@@@
        \\.@@@@@@@@.
        \\@.@.@@@.@.
    ;
    const p1 = try part1(std.testing.allocator, sample);
    try std.testing.expectEqual(13, p1);
}

test "test part 2" {
    const sample =
        \\..@@.@@@@.
        \\@@@.@.@.@@
        \\@@@@@.@.@@
        \\@.@@@@..@.
        \\@@.@@@@.@@
        \\.@@@@@@@.@
        \\.@.@.@.@@@
        \\@.@@@.@@@@
        \\.@@@@@@@@.
        \\@.@.@@@.@.
    ;
    const p1 = try part2(std.testing.allocator, sample);
    try std.testing.expectEqual(43, p1);
}
