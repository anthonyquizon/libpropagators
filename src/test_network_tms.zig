const std = @import("std");

const Decimal = @import("content_decimal.zig").Decimal;

const StoreBase = @import("content_tms.zig").TruthManagementStore;
const ContentBase = @import("content_tms.zig").TruthManagementContent;
const SupportBase = @import("content_tms.zig").Support;

const ContextBase = @import("context_tms.zig").TruthManagementContext;
const PremiseBase = @import("context_tms.zig").Premise;
const NetworkBase = @import("network.zig").Network;

const Arithmatic = @import("network_arithmatic.zig").Arithmatic;
const testing = std.testing;


test "add" {
    const PremiseSupplied = enum(u8) {
        Foo,
        Bar
    };

    const Premise = PremiseBase(PremiseSupplied);

    const Store = StoreBase(Decimal, Premise);
    const Content = ContentBase(Decimal, Premise);
    const Support = SupportBase(Decimal, Premise);

    const Context = ContextBase(Premise);
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
                    Premise.from(PremiseSupplied.Foo)
                })[0..]),
                Support.init(std.heap.page_allocator, Decimal.from(2.0), ([_]Premise {
                    Premise.from(PremiseSupplied.Bar)
                })[0..])
            })
        )
    );

    network.write_cell(b, 
        &Content.from(
            Store.init(std.heap.page_allocator, &context, &[_] Support { 
                Support.init(std.heap.page_allocator, Decimal.from(2.0), ([_]Premise {
                    Premise.from(PremiseSupplied.Foo), 
                    Premise.from(PremiseSupplied.Bar)
                })[0..])
            })
        )
    );

    network.run() catch unreachable;

    //const expected = TruthManagementStore.from(3.0);
    //const actual = network.read_cell(c);

    //testing.expect(expected.equals(actual));
}
