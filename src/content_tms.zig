const std = @import("std");
const ArrayList = std.ArrayList;
const Content = @import("content.zig").Content;
const Generics = @import("content.zig").Generics;
const Allocator = std.mem.Allocator;
const TruthManagementContext = @import("context_tms.zig").TruthManagementContext;

pub fn Support(comptime T: type, comptime P: type) type {
    return struct {
        const Self = @This();

        value: T,
        premises: ArrayList(P),

        pub fn init(allocator: *Allocator, value: T, premises: []const P) Self  {
            return Self {
                .value=value,
                .premises=ArrayList(P).init(allocator)
            };
        }
    };
}

pub fn TruthManagementStore(comptime T: type, comptime P: type) type {
    const Context = TruthManagementContext(P);
    const SupportTP = Support(T, P);

    return struct {
        const Self = @This();

        context: *Context,
        supports: ArrayList(SupportTP),
        allocator: *Allocator,

        pub fn init(allocator: *Allocator, context: *Context, supports: []SupportTP) Self {
            //var supports = ArrayList(SupportTP).init(allocator);

            //for (values) |value| {
                //std.debug.warn("{} {}\n,", .{value[0], value[1]});
                //std.debug.warn("{}\n,", .{value});
            //}
            //comptime var i = 0;
            //inline while (i < values.len) : (i += 1) {
                //std.debug.warn("{} {}\n,", .{values[i]});
            //}

            return Self {
                .context=context,
                .supports=undefined,
                .allocator=allocator
            };
        }

        pub fn merge(self: Self, other: Self) ?Self  {
            return Self {
                .context=self.context,
                .supports=undefined,
                .allocator=self.allocator
            };
        }

        pub fn eq(self: Self, other: Self) bool {
            return true;
        }

        pub fn add(self: Self, other: Self) ?Self  {
            return Self {
                .context=self.context,
                .supports=undefined,
                .allocator=self.allocator
            };
        }

        //pub fn sub(self: Self, other: Self) Self  {
            
        //}
    };
}

pub fn TruthManagementContent(comptime T: type, comptime P: type) type {
    const Value = TruthManagementStore(T, P);

    return Content(Value, Generics(Value) {
        .merge=Value.merge,
        .eq=Value.eq,
        .add=Value.add,
    });
}
