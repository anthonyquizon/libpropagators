const std = @import("std");
const testing = std.testing;
const Allocator = std.mem.Allocator;
const Cell = @import("cell.zig").Cell;
const Float = @import("cell_float.zig").Float;
const Network = @import("network.zig").Network;

test "init" {
    const allocator = std.heap.direct_allocator;
    var network = Network(Float).init(allocator);

    //a.write(1.0);
    //a.write(2.0);

    //testing.expect(a.read() == Float.Content.Contradiction);
}
