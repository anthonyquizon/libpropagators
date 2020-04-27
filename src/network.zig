const std = @import("std");
const warn = @import("std").debug.warn;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;
const C = @import("cell.zig");
const P = @import("propagator.zig");
const CellID = @import("util.zig").CellID;
const PropagatorID = @import("util.zig").PropagatorID;

pub fn Network(comptime T: type, comptime Context: type) type {
    const Cell = C.Cell(T);
    const Propagator = P.Propagator(T);
    const Result = P.Result(T);

    return struct {
        const Self = @This();

        cells: ArrayList(Cell),
        propagators: ArrayList(Propagator),
        alerted: ArrayList(PropagatorID),
        context: *Context,
        allocator: *Allocator,

        pub fn init(allocator: *Allocator, context: *Context) Self {
            return Self {
              .cells = ArrayList(Cell).init(allocator),
              .propagators = ArrayList(Propagator).init(allocator),
              .alerted = ArrayList(PropagatorID).init(allocator),
              .context = context,
              .allocator = allocator,
            };
        }

        pub fn deinit(self: *Self) void {}

        pub fn make_cell(self: *Self) CellID {
          var cell = Cell.init(self.allocator);
          self.cells.append(cell) catch unreachable;
          
          return self.cells.items.len - 1;
        }

        pub fn write_cell(self: *Self, cell_id: CellID, content: *const T) void {
          var cell = self.cells.items[cell_id];

          const alerted = cell.write(content);

          self.cells.items[cell_id] = cell;
        
          if (alerted) |propagator_ids| {
              for (propagator_ids.items) |propagator_id| {
                self.alerted.append(propagator_id) catch unreachable;
              }
          }
        }

        pub fn read_cell(self: *Self, cell_id: CellID) T {
          var cell = self.cells.items[cell_id];

          return cell.content;
        }

        pub fn make_propagator(self: *Self, procedure: fn([]T) T, inputs: []CellID, output: CellID) PropagatorID {
            var propagator = Propagator.init(self.allocator, procedure, inputs, output) ;

            self.propagators.append(propagator) catch unreachable;

            const propagator_id = self.propagators.items.len - 1;

            for (inputs) |cell_id| {
                self.cells.items[cell_id].neighbours.append(propagator_id) catch unreachable; //FIXME
            }

            return propagator_id;
        }
      
        pub fn run(self: *Self) !void {
          while (self.alerted.items.len > 0) {
            var writes = try self.allocator.alloc(Result, self.alerted.items.len);
            var contents = try self.allocator.alloc(T, self.cells.items.len);
            var cur : usize = 0;
            //TODO defererr?

            for (self.cells.items) |*cell, i| {
              contents[i] = cell.content;
            }

            for (self.alerted.items[0..self.alerted.items.len]) |propagator_id| {
              var propagator = self.propagators.items[propagator_id];

              writes[cur] = try propagator.invoke(contents);
              cur += 1;
            }

            self.alerted = ArrayList(PropagatorID).init(self.allocator);

            for (writes) |value| {
              switch (value) {
                Result.Nothing => {},
                Result.Pure => |result| {
                  self.write_cell(result.cell_id, &result.content);
                },
              }
            }

            //TODO realloc?
            self.allocator.free(writes); 
            self.allocator.free(contents); 
          }
        }
    };
}


