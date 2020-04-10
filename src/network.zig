const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const propagator_id = @import("util.zig").propagator_id;
const cell_id = @import("util.zig").cell_id;

pub fn Network(comptime Cell: type) type {
    return struct {
        const Self = @This();

        cells: ArrayList(Cell),
        //propagators: ArrayList(T),
        allocator: *Allocator,

        pub fn init(allocator: *Allocator) Self {
            return Self {
              .cells = ArrayList(Cell).init(allocator),
              .allocator = allocator
            };
        }
    };
}


