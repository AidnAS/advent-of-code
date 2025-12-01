const std = @import("std");
const model = @import("./model.zig");

const Day01 = @import("./puzzels/day01.zig");

pub const Solver = struct {
    pub fn init(puzzel: model.Puzzle) !void {
        switch (puzzel.Day) {
            1 => {
                try Day01.solve();
            },
            else => {
                std.debug.print("No puzzels yet", .{});
            },
        }
    }
};
