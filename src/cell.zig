const std = @import("std");
const propagator_id = @import("util.zig").propagator_id;

pub fn Cell(comptime T: type, merge: fn(T, T) ?T) type {
    return struct {
        pub const Content = union(enum) {
          Nothing,
          Value: T,
          Contradiction,
        };

        const Self = @This();

        name: []const u8,
        content: Content,
        neighbours: []propagator_id,

        pub fn init() Self {
            return Self {
              .name = "cell", 
              .content = Content.Nothing,
              .neighbours = undefined
            };
        }

        pub fn read(self: *Self) Content {
            return self.content;
        }

        pub fn write(self: *Self, value: T) void {
            switch (self.content) {
                .Nothing => {
                  self.content = Content { .Value = value };
                },
                Content.Value => |old_value| {
                  var new_value = merge(old_value, value);

                  if (new_value) |val| {
                    self.content = Content { .Value = val };
                  } else {
                    self.content = Content.Contradiction;
                  }
                },
                .Contradiction => {}
            }
        }
    };
}
