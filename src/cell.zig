const std = @import("std");
const propagator_id = @import("util.zig").propagator_id;

pub fn Cell(comptime T: type) type {
    return struct {
        const Self = @This();

        name: []const u8,
        content: T,
        neighbours: []propagator_id,

        pub fn init() Self {
            return Self {
              .name = "cell", 
              .content = T.init(),
              .neighbours = undefined
            };
        }

        pub fn read(self: *Self) T {
            return self.content;
        }

        pub fn write(self: *Self, content: T) void {
            var new_content = self.content.merge(content);
            self.content = content;
        }
    };
}
