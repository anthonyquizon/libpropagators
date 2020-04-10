const std = @import("std");
const Content = @import("content.zig").Content;


//FIXME equality checks
pub fn merge(a: f64, b: f64) ?f64 {
  return if (a == b) a else null;
}

pub const Decimal = Content(f64, merge);

//test "init" {
    //const testing = std.testing;
    //const Allocator = std.mem.Allocator;
    //const allocator = std.heap.direct_allocator;

    //var content = Float.init();


    //testing.expect(10 == 10);
//}
