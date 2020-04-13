const std = @import("std");
const stdout = std.io.getStdOut().outStream();
const warn = @import("std").debug.warn;
const testing = std.testing;
const Allocator = std.mem.Allocator;
const CellID = @import("util.zig").CellID;
const Decimal = @import("content_decimal.zig").Decimal;
const Arithmatic = @import("network_arithmatic.zig").Arithmatic;
const Network = @import("network.zig").Network;

test "init" {
    const allocator = std.heap.page_allocator;
    const propagator_add = Arithmatic(Decimal).add;
    var network = Network(Decimal).init(allocator);

    const a = network.make_cell();
    const b = network.make_cell();
    const c = network.make_cell();

    _ = propagator_add(&network, a, b, c);

    network.write_cell(a, Decimal.from(1.0));
    network.write_cell(b, Decimal.from(2.0));

    network.run() catch unreachable;

    const expected = Decimal.from(3.0);
    const actual = network.read_cell(c);

    testing.expect(expected.equals(actual));
}
