const std = @import("std");
const warn = @import("std").debug.warn;
const PropagatorID = @import("util.zig").PropagatorID;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

pub fn Cell(comptime T: type) type {
    return struct {
        const Self = @This();

        name: []const u8,
        content: T,
        neighbours: ArrayList(PropagatorID),

        pub fn init(allocator: *Allocator) Self {
            return Self {
              .name = "cell", 
              .content = T.init(),
              .neighbours = ArrayList(PropagatorID).init(allocator)
            };
        }

        pub fn write(self: *Self, content: T) ?ArrayList(PropagatorID) {
            var new_content = self.content.merge(content);

            if (!self.content.equals(new_content)) {
              self.content = new_content;

              return self.neighbours;
            }
            else {
              return null;
            }
        }
    };
}
