const std = @import("std");
const ArrayList = std.ArrayList;
const Content = @import("content.zig").Content;
const Generics = @import("content.zig").Generics;
const TruthManagementContext = @import("content.zig").TruthManagementContext;

pub fn TruthManagementStore(comptime T: type, comptime P: type) type {
    const Context = TruthManagementContext(P);
    const Support = struct {
        const Self = @This();

        value: T,
        premises: ArrayList(P),
    };

    const Instance = struct {
        const Self = @This();

        context: *Context,
        supports: ArrayList(Support),
        allocator: *Allocator,

        pub fn from(allocator: *Allocator, context: *Context, value: []T) Self {
            Self {
                context: context,
                supports: ArrayList(Support).init(allocator),
                allocator: allocator
            }
        }

        pub fn merge(self: Self, other: Self) Self  {
            Self {
                context: self.context,
                supports: 
            }
        }

        //pub fn add(self: Self, other: Self) Self  {
            
        //}

        //pub fn sub(self: Self, other: Self) Self  {
            
        //}
    };

    return Content(T, Self, Generics(T, Self) {
        .merge=Instance.merge,
        .from=Instance.from
    });
}
