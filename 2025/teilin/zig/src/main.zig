const std = @import("std");
const model = @import("./model.zig");
const solver = @import("./solver.zig");

const usage_text =
    \\Usage: aoc-zig [options]
    \\
    \\Run the selected AOC solution
    \\
    \\Options:
    \\ -d, --day <day>
    \\
;

const Command = struct { raw_cmd: []const u8, argv: []const []const u8 };

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const args = try std.process.argsAlloc(allocator);

    var commands = std.array_list.Managed(Command).init(allocator);

    var d: u64 = 1;

    var arg_i: usize = 1;
    while (arg_i < args.len) : (arg_i += 1) {
        const arg = args[arg_i];
        if (!std.mem.startsWith(u8, arg, "-")) {
            var cmd_argv = std.array_list.Managed([]const u8).init(allocator);
            try parseCmd(&cmd_argv, arg);
            try commands.append(.{
                .raw_cmd = arg,
                .argv = try cmd_argv.toOwnedSlice(),
            });
        } else if (std.mem.eql(u8, arg, "-d") or std.mem.eql(u8, arg, "--day")) {
            arg_i += 1;
            if (arg_i >= args.len) {
                std.debug.print("'{s}' requires a day.\n{s}", .{ arg, usage_text });
                return std.process.exit(1);
            }
            const next = args[arg_i];
            const day = std.fmt.parseInt(u64, next, 10) catch |err| {
                std.debug.print("unable to parse --day argument '{s}: {s}\n", .{ next, @errorName(err) });
                std.process.exit(1);
            };
            d = day;
        } else {
            std.debug.print("unrecognized argument: '{s}'\n{s}", .{ arg, usage_text });
            std.process.exit(1);
        }
    }

    const puzzel = try model.Puzzle.init(d);
    try solver.Solver.init(allocator, puzzel);
}

fn parseCmd(list: *std.array_list.Managed([]const u8), cmd: []const u8) !void {
    var it = std.mem.tokenizeScalar(u8, cmd, ' ');
    while (it.next()) |s| try list.append(s);
}
