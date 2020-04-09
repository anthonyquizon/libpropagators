const std = @import("std");
const propagator_id = @import("util").propagator_id;

pub fn Cell(comptime T: type) type {
    return struct {
      name: []const u8,
      content: T,
      neighbours: []propagator_id
    };

    pub fn init(allocator: *std.mem.Allocator, content) Cell {
    }
}
