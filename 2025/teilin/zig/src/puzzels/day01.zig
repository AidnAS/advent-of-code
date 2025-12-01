const std = @import("std");
const log = std.log;

pub fn solve() !void {
    const content = @embedFile("./data/day01.txt");

    const part1 = try numTimesOnZero(content, false);
    const part2 = try numTimesOnZero(content, true);

    std.debug.print("Part 1 -> {d}\n", .{part1});
    std.debug.print("Part 2 -> {d}", .{part2});
}

fn numTimesOnZero(content: []const u8, countAny: bool) !u32 {
    var readIter = std.mem.tokenizeSequence(u8, content, "\n");
    var dialPosition: u32 = 50;
    var numZero: u32 = 0;
    while (readIter.next()) |line| {
        const direction: u8 = line[0];
        var num: u32 = try std.fmt.parseInt(u32, line[1..], 10);
        if (direction == 'L') {
            while (num > 0) {
                dialPosition = try getNewPositition(dialPosition, direction);
                num -= 1;
                if (countAny and dialPosition == 0) {
                    numZero += 1;
                }
            }
        }
        if (direction == 'R') {
            while (num > 0) {
                dialPosition = try getNewPositition(dialPosition, direction);
                num -= 1;
                if (countAny and dialPosition == 0) {
                    numZero += 1;
                }
            }
        }
        if (!countAny and dialPosition == 0) {
            numZero += 1;
        }
    }
    return numZero;
}

fn getNewPositition(currentPosition: u32, direction: u8) !u32 {
    if (direction == 'L') {
        if (currentPosition == 0) {
            return 99;
        } else {
            return currentPosition - 1;
        }
    }
    if (direction == 'R') {
        if (currentPosition == 99) {
            return 0;
        } else {
            return currentPosition + 1;
        }
    }
    return currentPosition;
}

test "part 1 test" {
    const sample =
        \\L68
        \\L30
        \\R48
        \\L5
        \\R60
        \\L55
        \\L1
        \\L99
        \\R14
        \\L82
    ;
    const result = numTimesOnZero(sample, false);
    try std.testing.expectEqual(3, result);
}

test "part 2 test" {
    const sample =
        \\L68
        \\L30
        \\R48
        \\L5
        \\R60
        \\L55
        \\L1
        \\L99
        \\R14
        \\L82
    ;
    const result = numTimesOnZero(sample, true);
    try std.testing.expectEqual(6, result);
}
