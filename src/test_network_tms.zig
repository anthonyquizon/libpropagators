const std = @import("std");

const Decimal = @import("content_decimal.zig").Decimal;

const TruthManagementSystemBase = @import("truth_management_system.zig").TruthManagementSystem;
const NetworkBase = @import("network.zig").Network;

const Arithmatic = @import("network_arithmatic.zig").Arithmatic;
const testing = std.testing;


test "add" {
    const Premise = enum(u8) {
        Foo,
        Bar
    };

    const TruthManagementSystem = TruthManagementSystemBase(Decimal, Premise);

    const Store = TruthManagementSystem.Store;
    const Content = TruthManagementSystem.Content;
    const Support = TruthManagementSystem.Support;
    const Context = TruthManagementSystem.Context;

    const Network = NetworkBase(Content, Context);

    var context = Context.init(std.heap.page_allocator);
    var network = Network.init(std.heap.page_allocator, &context);

    const propagator_add = Arithmatic(Network, Content).add;

    const a = network.make_cell();
    const b = network.make_cell();
    const c = network.make_cell();

    _ = propagator_add(&network, a, b, c);

    network.write_cell(a, 
        &Content.from(
            Store.init(std.heap.page_allocator, &context, &[_] Support { 
                Support.init(std.heap.page_allocator, Decimal.from(1.0), ([_]Premise {
                    Premise.Foo
                })[0..]),
                Support.init(std.heap.page_allocator, Decimal.from(2.0), ([_]Premise {
                    Premise.Bar
                })[0..])
            })
        )
    );

    network.write_cell(b, 
        &Content.from(
            Store.init(std.heap.page_allocator, &context, &[_] Support { 
                Support.init(std.heap.page_allocator, Decimal.from(2.0), ([_]Premise {
                    Premise.Foo, 
                    Premise.Bar
                })[0..])
            })
        )
    );

    network.run() catch unreachable;

    //const expected = TruthManagementStore.from(3.0);
    //const actual = network.read_cell(c);

    //testing.expect(expected.equals(actual));
}
