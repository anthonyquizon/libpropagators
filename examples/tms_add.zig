const std = @import("std");

const Decimal = @import("propagators").Decimal;
const TruthManagementSystemBase = @import("propagators").TruthManagementSystem;
const NetworkBase = @import("propagators").Network;
const Arithmatic = @import("propagators").Arithmatic;

const testing = std.testing;


//pub fn main() anyerror!void {
    //const PremiseSupplied = enum(u8) {
        //Foo,
        //Bar
    //};

    //const Premise = PremiseBase(PremiseSupplied);

    //const Store = StoreBase(Decimal, Premise);
    //const Content = ContentBase(Decimal, Premise);
    //const Support = SupportBase(Decimal, Premise);

    //const Context = ContextBase(Premise);
    //const Network = NetworkBase(Content, Context);

    //var context = Context.init(std.heap.page_allocator);
    //var network = Network.init(std.heap.page_allocator, &context);

    //const propagator_add = Arithmatic(Network, Content).add;

    //const a = network.make_cell();
    //const b = network.make_cell();
    //const c = network.make_cell();

    //_ = propagator_add(&network, a, b, c);

    //network.write_cell(a, 
        //&Content.from(
            //Store.init(std.heap.page_allocator, &context, &[_] Support { 
                //Support.init(std.heap.page_allocator, Decimal.from(1.0), ([_]Premise {
                    //Premise.from(PremiseSupplied.Foo)
                //})[0..]),
                //Support.init(std.heap.page_allocator, Decimal.from(2.0), ([_]Premise {
                    //Premise.from(PremiseSupplied.Bar)
                //})[0..])
            //})
        //)
    //);

    //network.write_cell(b, 
        //&Content.from(
            //Store.init(std.heap.page_allocator, &context, &[_] Support { 
                //Support.init(std.heap.page_allocator, Decimal.from(2.0), ([_]Premise {
                    //Premise.from(PremiseSupplied.Foo), 
                    //Premise.from(PremiseSupplied.Bar)
                //})[0..])
            //})
        //)
    //);

    //network.run() catch unreachable;

    ////const expected = TruthManagementStore.from(3.0);
    ////const actual = network.read_cell(c);

    ////testing.expect(expected.equals(actual));
//}

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

pub fn main() anyerror!void {
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


    std.debug.warn("wooo!\n", .{});
}
