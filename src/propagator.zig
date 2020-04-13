const std = @import("std");
const warn = @import("std").debug.warn;
const CellID = @import("util.zig").CellID;
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;


pub fn Result(comptime T: type) type {
  const Value = struct { cell_id: CellID, content: T };

  return union(enum) {
    Nothing,
    Pure: Value
  };
}


pub fn Propagator(comptime T: type) type {
  return struct {
    const Self = @This();

    name: []const u8,
    procedure: fn([]T) T,
    inputs: ArrayList(CellID),
    output: CellID,
    allocator: *Allocator,

    pub fn init(allocator: *Allocator, procedure: fn([]T) T, inputs: []CellID, output: CellID) Self {
      var inputs_arr = ArrayList(CellID).init(allocator);
      
      for (inputs) |input| {
        inputs_arr.append(input) catch unreachable;
      }

      return Self {
        .name= "propagator",
        .procedure=procedure,
        .inputs=inputs_arr,
        .output= output,
        .allocator=allocator,
      };
    }

    pub fn invoke(self: *Self, contents: []T) !Result(T) {
        var inputs = try self.allocator.alloc(T, self.inputs.items.len);
        defer self.allocator.free(inputs);

        for (self.inputs.items) |cell_id, i| {
          const value = contents[cell_id];

          if (value.is_empty()) {
            return .Nothing;
          }

          inputs[i] = value;
        }

        const output = self.procedure(inputs);

        return Result(T) { 
          .Pure = .{ .cell_id=self.output, .content=output }
        };
    }
    
  };
}
