const std = @import("std");
const log = std.log;

pub fn solve() !void {
    const content = @embedFile("./data/day02.txt");

    const part1 = try detectInvalidRanges(content, false);
    const part2 = try detectInvalidRanges(content, true);

    std.debug.print("Part 1 -> {d}\n", .{part1});
    std.debug.print("Part 2 -> {d}", .{part2});
}

fn detectInvalidRanges(content: []const u8, repedAtleastTwice: bool) !u128 {
    var it = std.mem.splitScalar(u8, content, ',');
    var sum: u128 = 0;
    while (it.next()) |range| {
        var rangeSplit = std.mem.splitScalar(u8, range, '-');

        const p1: []const u8 = rangeSplit.next() orelse "";
        const p2: []const u8 = rangeSplit.next() orelse "";

        const num1 = try std.fmt.parseInt(u64, p1, 10);
        const num2 = try std.fmt.parseInt(u64, p2, 10);
        const invalid = try detectInvalidId(num1, num2, repedAtleastTwice);

        sum += invalid;
    }
    return sum;
}

fn digitsLen(nn: usize) usize {
    var cnt: usize = 0;
    var n = nn;

    while (n > 0) : (cnt += 1) {
        n /= 10;
    }

    return cnt;
}

fn detectInvalidId(min: u64, max: u64, repedAtleastTwice: bool) !u64 {
    var sum: u64 = 0;
    if (!repedAtleastTwice) {
        var current_val: u64 = min;
        while (current_val <= max) {
            const isValid = try isInvalid(current_val);
            if (isValid) {
                sum += current_val;
            }
            current_val += 1;
        }
    } else {
        // Part 2
        outer: for (min..(max + 1)) |val| {
            const len = digitsLen(val);
            inner: for (1..((len / 2) + 1)) |dlen| {
                if (len % dlen != 0) continue;
                const pow = try std.math.powi(usize, 10, dlen);
                var vv = val;
                const sub = vv % pow;

                while (vv > 0) {
                    if ((vv % pow) != sub) continue :inner;
                    vv /= pow;
                }

                sum += val;
                continue :outer;
            }
        }
    }
    return sum;
}

test "test invalid detaction" {
    const sum: u64 = try detectInvalidId(1188511880, 1188511890, true);
    try std.testing.expectEqual(1188511885, sum);
}

fn isInvalid(val: u64) !bool {
    var buffer: [64]u8 = undefined;
    const str = try std.fmt.bufPrint(&buffer, "{}", .{val});
    if (@mod(str.len, 2) > 0) {
        return false;
    }
    const p1 = str[0 .. str.len / 2];
    const p2 = str[str.len / 2 .. str.len];

    if (std.mem.eql(u8, p1, p2)) {
        return true;
    }
    return false;
}

test "Is invalid test" {
    const invalid = try isInvalid(1188511885);
    try std.testing.expect(invalid);
}

test "part 1 test" {
    const sample = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    const suminvalid = detectInvalidRanges(sample, false);
    try std.testing.expectEqual(1227775554, suminvalid);
}

test "part 2 test" {
    const sample = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    const suminvalid = detectInvalidRanges(sample, true);
    //const suminvalid = try p2tmp(sample);
    try std.testing.expectEqual(4174379265, suminvalid);
}
