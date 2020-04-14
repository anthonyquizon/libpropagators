const std = @import("std");
const Network = @import("network.zig").Network;
const TruthManagementStore= @import("content_tms.zig").TruthManagementStore;
const TruthManagementContext= @import("context_tms.zig").TruthManagementContext;
const Arithmatic = @import("network_arithmatic.zig").Arithmatic;
const testing = std.testing;


test "add" {
    const propagator_add = Arithmatic(TruthManagementStore).add;
    var network = Network(TruthManagementStore).init(std.heap.page_allocator);

    const a = network.make_cell();
    const b = network.make_cell();
    const c = network.make_cell();

    _ = propagator_add(&network, a, b, c);

    network.write_cell(a, TruthManagementStore.from(1.0));
    network.write_cell(b, TruthManagementStore.from(2.0));

    network.run() catch unreachable;

    const expected = TruthManagementStore.from(3.0);
    const actual = network.read_cell(c);

    testing.expect(expected.equals(actual));
}
