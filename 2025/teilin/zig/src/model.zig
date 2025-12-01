pub const Puzzle = struct {
    Day: u64,

    pub fn init(day: u64) !@This() {
        return .{
            .Day = day,
        };
    }
};
