const std = @import("std");
const stdout = std.io.getStdOut().outStream();
const warn = @import("std").debug.warn;
const testing = std.testing;
const Allocator = std.mem.Allocator;
const Cell = @import("cell.zig").Cell;
const CellID = @import("util.zig").CellID;
const Decimal = @import("content_decimal.zig").Decimal;
const Propagator = @import("propagator.zig").Propagator;
const Network = @import("network.zig").Network;

fn add(args: []Decimal) Decimal {
  return Decimal.add(args[0], args[1]);
}

test "init" {
    const allocator = std.heap.page_allocator;
    var network = Network(Decimal).init(allocator);

    const a = network.make_cell();
    const b = network.make_cell();
    const c = network.make_cell();

    var inputs = [_]CellID {a, b};
    _ = network.make_propagator(add, inputs[0..], c);

    network.write_cell(a, Decimal.from(1.0));
    network.write_cell(b, Decimal.from(2.0));

    network.run() catch unreachable;

    const expected = Decimal.from(3.0);
    const actual = network.read_cell(c);

    testing.expect(expected.equals(actual));
}
