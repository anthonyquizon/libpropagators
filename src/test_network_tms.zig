const std = @import("std");

const Store = @import("content_tms.zig").TruthManagementStore(f64);
const Content = @import("content_tms.zig").TruthManagementContent(f64);
const Support = @import("content_tms.zig").Support(f64);
const Context = @import("context_tms.zig").TruthManagementContext;
const Premise = @import("context_tms.zig").Premise;

const Arithmatic = @import("network_arithmatic.zig").Arithmatic;
const Network = @import("network.zig").Network(Content, Context);
const testing = std.testing;


test "add" {
    const Foo=2;
    const Bar=3;

    var context = Context.init(std.heap.page_allocator);
    var network = Network.init(std.heap.page_allocator, &context);

    const propagator_add = Arithmatic(Network, Content).add;

    const a = network.make_cell();
    const b = network.make_cell();
    const c = network.make_cell();

    _ = propagator_add(&network, a, b, c);

    network.write_cell(a, 
        Content.from(
            Store.init(std.heap.page_allocator, &context, &[_] Support { 
                Support.init(std.heap.page_allocator, 1.0, ([_]Premise {Foo})[0..]),
                Support.init(std.heap.page_allocator, 2.0, ([_]Premise {Bar})[0..])
            })
        )
    );

    network.write_cell(b, 
        Content.from(
            Store.init(std.heap.page_allocator, &context, &[_] Support { 
                Support.init(std.heap.page_allocator, 2.0, ([_]Premise {Foo, Bar})[0..])
            })
        )
    );

    network.run() catch unreachable;

    //const expected = TruthManagementStore.from(3.0);
    //const actual = network.read_cell(c);

    //testing.expect(expected.equals(actual));
}
