const std = @import("std");
const log = std.log;

pub fn solve(allocator: std.mem.Allocator) !void {
    const content = @embedFile("./data/day03.txt");

    const part1 = try findTotalJoltage(allocator, content, false);
    const part2 = try findTotalJoltage(allocator, content, true);

    std.debug.print("Part 1 -> {d}\n", .{part1});
    std.debug.print("Part 2 -> {d}", .{part2});
}

fn findTotalJoltage(allocator: std.mem.Allocator, content: []const u8, part2: bool) !u64 {
    var it = std.mem.tokenizeScalar(u8, content, '\n');

    var sum: u64 = 0;
    while (it.next()) |line| {
        if (!part2) {
            const num = findTwoLargestWithIndex(line);
            sum += num;
        } else {
            // part 2
            const num = try findLargestDigits(allocator, line);
            sum += num;
        }
    }

    return sum;
}

fn findTwoLargestWithIndex(digits: []const u8) u64 {
    var current: u64 = 0;

    for (digits, 0..) |d1, idx1| {
        const num1 = d1 - '0';
        for (digits[1..], 1..) |d2, idx2| {
            const num2 = d2 - '0';
            if (idx2 > idx1) {
                const tmp = concat(num1, num2);
                if (tmp > current) {
                    current = tmp;
                }
            }
        }
    }

    return current;
}

fn concat(n1: u8, n2: u8) u64 {
    var buffer: [2]u8 = undefined;
    buffer[0] = n1 + '0';
    buffer[1] = n2 + '0';
    const slice: []const u8 = &buffer;
    return std.fmt.parseInt(u64, slice, 10) catch |err| switch (err) {
        error.Overflow => {
            return 0;
        },
        error.InvalidCharacter => {
            return 0;
        },
    };
}

fn findLargestDigits(allocator: std.mem.Allocator, input: []const u8) !u64 {
    var joltage_sum: u64 = 0;
    const JOLTAGE_LENGTH = 12;
    var lines = std.mem.splitScalar(u8, input, '\n');
    while (lines.next()) |bank| {
        var joltage_str = try allocator.alloc(u8, JOLTAGE_LENGTH);
        defer allocator.free(joltage_str);

        var prev_index: u64 = 0;
        for (0..JOLTAGE_LENGTH) |i| {
            const value = std.mem.indexOfMax(u8, bank[prev_index .. bank.len - (JOLTAGE_LENGTH - i - 1)]);
            joltage_str[i] = bank[value + prev_index];
            prev_index += value + 1;
        }
        const joltage = try std.fmt.parseInt(u64, joltage_str, 10);
        joltage_sum += joltage;
    }
    return joltage_sum;
}

test "test line 1" {
    const n = findTwoLargestWithIndex("987654321111111");
    try std.testing.expectEqual(98, n);
}

test "test line 2" {
    const n = findTwoLargestWithIndex("811111111111119");
    try std.testing.expectEqual(89, n);
}

test "test line 3" {
    const n = findTwoLargestWithIndex("234234234234278");
    try std.testing.expectEqual(78, n);
}

test "test line 4" {
    const n = findTwoLargestWithIndex("818181911112111");
    try std.testing.expectEqual(92, n);
}

test "test line 1 part 2" {
    const num = try findLargestDigits(std.testing.allocator, "987654321111111");
    try std.testing.expectEqual(987654321111, num);
}

test "test part 1" {
    const sample =
        \\987654321111111
        \\811111111111119
        \\234234234234278
        \\818181911112111
    ;
    const totalJoule = try findTotalJoltage(std.testing.allocator, sample, false);
    try std.testing.expectEqual(357, totalJoule);
}

test "test part 2" {
    const sample =
        \\987654321111111
        \\811111111111119
        \\234234234234278
        \\818181911112111
    ;
    const totalJoule = findTotalJoltage(std.testing.allocator, sample, true);
    try std.testing.expectEqual(3121910778619, totalJoule);
}
