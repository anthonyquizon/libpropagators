const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const propagator_id = @import("util.zig").propagator_id;
const Cell = @import("cell.zig").Cell;
const cell_id = @import("util.zig").cell_id;

pub fn Network(comptime T: type) type {
    return struct {
        const Self = @This();

        cells: ArrayList(Cell(T)),
        //propagators: ArrayList(T),
        allocator: *Allocator,

        pub fn init(allocator: *Allocator) Self {
            return Self {
              .cells = ArrayList(Cell(T)).init(allocator),
              .allocator = allocator
            };
        }
    };
}


