const std = @import("std");
const log = std.log;

const Range = struct {
    min: u64,
    max: u64,

    pub fn contains(self: *const Range, num: u64) bool {
        return num >= self.min and num <= self.max;
    }

    pub fn overlaps(self: *const Range, other: *const Range) bool {
        return self.min <= other.max and self.max >= other.min;
    }

    pub fn extend(self: *Range, other: *const Range) void {
        self.min = @min(self.min, other.min);
        self.max = @max(self.max, other.max);
    }
};

pub fn solve(allocator: std.mem.Allocator) !void {
    const content = @embedFile("./data/day05.txt");

    const p1 = try part1(allocator, content);
    const p2 = try part2(allocator, content);

    std.debug.print("Part 1 -> {d}\n", .{p1});
    std.debug.print("Part 2 -> {d}", .{p2});
}

fn part1(allocator: std.mem.Allocator, input: []const u8) !u64 {
    var list = try std.ArrayList(Range).initCapacity(allocator, 100);
    defer list.deinit(allocator);

    var it = std.mem.tokenizeScalar(u8, input, '\n');

    var countSpolied: u64 = 0;

    while (it.next()) |line| {
        if (std.mem.indexOfScalar(u8, line, '-')) |sep_idx| {
            const sMin = line[0..sep_idx];
            const sMax = line[sep_idx + 1 .. line.len];
            const iMin = try std.fmt.parseInt(u64, sMin, 10);
            const iMax = try std.fmt.parseInt(u64, sMax, 10);
            try list.append(allocator, .{ .min = iMin, .max = iMax });
        } else {
            const id = try std.fmt.parseInt(u64, line, 10);

            for (list.items) |range| {
                if (range.contains(id)) {
                    countSpolied += 1;
                    break;
                }
            }
        }
    }
    return countSpolied;
}

fn part2(allocator: std.mem.Allocator, input: []const u8) !u64 {
    var list = try std.ArrayList(Range).initCapacity(allocator, 100);
    defer list.deinit(allocator);

    var it = std.mem.tokenizeScalar(u8, input, '\n');

    while (it.next()) |line| {
        if (std.mem.indexOfScalar(u8, line, '-')) |sep_idx| {
            const sMin = line[0..sep_idx];
            const sMax = line[sep_idx + 1 .. line.len];
            const iMin = try std.fmt.parseInt(u64, sMin, 10);
            const iMax = try std.fmt.parseInt(u64, sMax, 10);
            try list.append(allocator, .{ .min = iMin, .max = iMax });
        }
    }

    const len = list.items.len;
    for (1..len + 1) |i| {
        const idx = len - i;
        const range = &list.items[idx];

        for (0..idx) |j| {
            if (list.items[j].overlaps(range)) {
                list.items[j].extend(range);
                _ = list.orderedRemove(idx);
                break;
            }
        }
    }

    var totalFresh: u64 = 0;
    for (list.items) |range| {
        totalFresh += (range.max - range.min + 1);
    }

    return totalFresh;
}

test "part 1 test" {
    const sample =
        \\3-5
        \\10-14
        \\16-20
        \\12-18
        \\
        \\1
        \\5
        \\8
        \\11
        \\17
        \\32
    ;
    const count = try part1(std.testing.allocator, sample);
    try std.testing.expectEqual(3, count);
}

test "part 2 test" {
    const sample =
        \\3-5
        \\10-14
        \\16-20
        \\12-18
        \\
        \\1
        \\5
        \\8
        \\11
        \\17
        \\32
    ;
    const count = try part2(std.testing.allocator, sample);
    try std.testing.expectEqual(14, count);
}
