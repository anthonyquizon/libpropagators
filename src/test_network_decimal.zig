const std = @import("std");
const Decimal = @import("content_decimal.zig").Decimal;
const Context = @import("context_empty.zig").EmptyContext;
const Network = @import("network.zig").Network(Decimal, Context);
const Arithmatic = @import("network_arithmatic.zig").Arithmatic;
const testing = std.testing;

//FIXME use testing allocator

test "add" {
    const propagator_add = Arithmatic(Network, Decimal).add;
    var context = Context.init();
    var network = Network.init(std.heap.page_allocator, &context);

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
    const propagator_sub = Arithmatic(Network, Decimal).sub;
    var context = Context.init();
    var network = Network.init(std.heap.page_allocator, &context);

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
    const constraint_add = Arithmatic(Network, Decimal).constraint_add;
    var context = Context.init();
    var network = Network.init(std.heap.page_allocator, &context);

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
    const constraint_add = Arithmatic(Network, Decimal).constraint_add;
    var context = Context.init();
    var network = Network.init(std.heap.page_allocator, &context);

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


