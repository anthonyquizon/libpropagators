const Network = @import("network.zig").Network;
const CellID = @import("util.zig").CellID;
const PropagatorID = @import("util.zig").PropagatorID;

pub fn Binary(comptime T: type, f: fn(T, T) T) type {
  return struct {
    pub fn propagator(network: *Network(T), a: CellID, b: CellID, c: CellID) PropagatorID {
      var inputs = [_]CellID {a, b};

      const Context = struct {
          fn f_wrapper(args: []T) T {
              return f(args[0], args[1]);
          }
      };

      return network.make_propagator(Context.f_wrapper, inputs[0..], c);
    }
  };
}

pub fn Arithmatic(comptime T: type) type {
  return struct {
      pub const add = Binary(T, T.add).propagator;
      pub const sub = Binary(T, T.sub).propagator;

      pub fn constraint_add(network: *Network(T), a: CellID, b: CellID, c: CellID) void {
        _ = add(network, a, b, c);
        _ = sub(network, c, a, b);
        _ = sub(network, c, b, a);
      }
  };
}


test "add" {
    const std = @import("std");
    const Decimal = @import("content_decimal.zig").Decimal;
    const testing = std.testing;
    const propagator_add = Arithmatic(Decimal).add;
    //TODO use testing allocator
    var network = Network(Decimal).init(std.heap.page_allocator);

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

test "sub" {
    const std = @import("std");
    const Decimal = @import("content_decimal.zig").Decimal;
    const testing = std.testing;
    const propagator_sub = Arithmatic(Decimal).sub;
    //TODO use testing allocator
    var network = Network(Decimal).init(std.heap.page_allocator);

    const a = network.make_cell();
    const b = network.make_cell();
    const c = network.make_cell();

    _ = propagator_sub(&network, a, b, c);

    network.write_cell(a, Decimal.from(1.0));
    network.write_cell(b, Decimal.from(2.0));

    network.run() catch unreachable;

    const expected = Decimal.from(-1.0);
    const actual = network.read_cell(c);

    testing.expect(expected.equals(actual));
}

test "constaint_add forwards" {
    const std = @import("std");
    const Decimal = @import("content_decimal.zig").Decimal;
    const testing = std.testing;
    const constraint_add = Arithmatic(Decimal).constraint_add;
    //TODO use testing allocator
    var network = Network(Decimal).init(std.heap.page_allocator);

    const a = network.make_cell();
    const b = network.make_cell();
    const c = network.make_cell();

    _ = constraint_add(&network, a, b, c);

    network.write_cell(a, Decimal.from(1.0));
    network.write_cell(b, Decimal.from(2.0));

    network.run() catch unreachable;

    const expected = Decimal.from(3.0);
    const actual = network.read_cell(c);

    testing.expect(expected.equals(actual));
}

test "constaint_add backwards" {
    const std = @import("std");
    const Decimal = @import("content_decimal.zig").Decimal;
    const testing = std.testing;
    const constraint_add = Arithmatic(Decimal).constraint_add;
    //TODO use testing allocator
    var network = Network(Decimal).init(std.heap.page_allocator);

    const a = network.make_cell();
    const b = network.make_cell();
    const c = network.make_cell();

    _ = constraint_add(&network, a, b, c);

    network.write_cell(b, Decimal.from(2.0));
    network.write_cell(c, Decimal.from(3.0));

    network.run() catch unreachable;

    const expected = Decimal.from(1.0);
    const actual = network.read_cell(a);

    testing.expect(expected.equals(actual));
}
