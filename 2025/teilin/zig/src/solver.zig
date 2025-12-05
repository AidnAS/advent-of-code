const std = @import("std");
const model = @import("./model.zig");

const Day01 = @import("./puzzels/day01.zig");
const Day02 = @import("./puzzels/day02.zig");
const Day03 = @import("./puzzels/day03.zig");
const Day04 = @import("./puzzels/day04.zig");
const Day05 = @import("./puzzels/day05.zig");

pub const Solver = struct {
    pub fn init(allocator: std.mem.Allocator, puzzel: model.Puzzle) !void {
        switch (puzzel.Day) {
            1 => {
                try Day01.solve();
            },
            2 => {
                try Day02.solve();
            },
            3 => {
                try Day03.solve(allocator);
            },
            4 => {
                try Day04.solve(allocator);
            },
            5 => {
                try Day05.solve(allocator);
            },
            else => {
                std.debug.print("No puzzels yet", .{});
            },
        }
    }
};
