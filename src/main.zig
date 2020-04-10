const std = @import("std");
const testing = std.testing;
const Allocator = std.mem.Allocator;
const Cell = @import("cell.zig").Cell;
const Decimal = @import("content_decimal.zig").Decimal;
const Network = @import("network.zig").Network;

test "init" {
    const allocator = std.heap.direct_allocator;
    var network = Network(Decimal).init(allocator);

    //a.write(1.0);
    //a.write(2.0);

    //testing.expect(a.read() == Float.Content.Contradiction);
}
