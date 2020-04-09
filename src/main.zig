const std = @import("std");
const testing = std.testing;
const Allocator = std.mem.Allocator;
const Cell = @import("cell.zig").Cell;
const Float = @import("cell_float.zig").Float;

test "init" {
    const allocator = std.heap.direct_allocator;
    var a = Float.init();

    a.write(1.0);
    a.write(2.0);

    testing.expect(a.read() == null);
}
