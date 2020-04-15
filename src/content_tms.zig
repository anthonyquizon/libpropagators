const std = @import("std");
const assert = @import("std").debug.assert;
const sort = @import("std").sort.sort;
const asc = @import("std").sort.asc;
const mem = @import("std").mem;
const ArrayList = std.ArrayList;
const Content = @import("content.zig").Content;
const Generics = @import("content.zig").Generics;
const Allocator = std.mem.Allocator;
const Context = @import("context_tms.zig").TruthManagementContext;
const Premise = @import("context_tms.zig").Premise;

//TODO offset reserved premises
//TODO enum
const False = 0;
const True = 1;

pub fn Support(comptime T: type) type {
    return struct {
        const Self = @This();

        value: T,
        premises: []Premise,

        pub fn init(allocator: *Allocator, value: T, premises: []const Premise) Self {
            var premise_arr = allocator.alloc(Premise, premises.len) catch unreachable;

            mem.copy(Premise, premise_arr[0..], premises);
            
            sort(Premise, premise_arr[0..], asc(Premise));

            return Self {
                .value=value,
                .premises=premise_arr
            };
        }

        pub fn compare_asc(self: Self, other: Self) bool {
            if (self.value < other.value) {
                return true;
            }

            if (self.premises.len < other.premises.len) {
                return true;
            }

            for (self.premises) |premise, i| {
                if (premise < other.premises[i]) {
                    return true;
                }
            }

            return false;
        }

        pub fn eq(self: Self, other: Self) bool {
            if (self.value != other.value) {
                return false;
            }

            if (self.premises.len != other.premises.len) {
                return false;
            }

            for (self.premises) |premise, i| {
                if (premise != other.premises[i]) {
                    return false;
                }
            }

            return true;
        }
    };
}

pub fn TruthManagementStore(comptime T: type) type {
    const SupportT = Support(T);

    return struct {
        const Self = @This();

        context: *Context,
        supports: []SupportT,
        allocator: *Allocator,

        pub fn init(allocator: *Allocator, context: *Context, supports: []SupportT) Self {
            var support_arr = allocator.alloc(SupportT, supports.len) catch unreachable;

            mem.copy(SupportT, support_arr[0..], supports);
            
            sort(SupportT, support_arr[0..], SupportT.compare_asc);

            return Self {
                .context=context,
                .supports=support_arr,
                .allocator=allocator
            };
        }

        pub fn merge(self: Self, other: Self) ?Self  {
            var support_arr = self.allocator.alloc(SupportT, self.supports.len) catch unreachable;

            mem.copy(SupportT, support_arr[0..], self.supports);

            sort(SupportT, support_arr[0..], SupportT.compare_asc);
            
            return Self {
                .context=self.context,
                .supports=support_arr,
                .allocator=self.allocator
            };
        }

        pub fn eq(self: Self, other: Self) bool {
            if (self.supports.len != other.supports.len) {
                return false;
            }

            for (self.supports) |support, i| {
                if (!SupportT.eq(support, other.supports[i])) {
                    return false;
                }
            }

            return true;
        }

        pub fn add(self: Self, other: Self) ?Self  {
            var support_arr = self.allocator.alloc(SupportT, self.supports.len) catch unreachable;

            mem.copy(SupportT, support_arr[0..], self.supports);

            sort(SupportT, support_arr[0..], SupportT.compare_asc);
            
            return Self {
                .context=self.context,
                .supports=support_arr,
                .allocator=self.allocator
            };
        }

        //pub fn sub(self: Self, other: Self) Self  {
            
        //}
    };
}

pub fn TruthManagementContent(comptime T: type) type {
    const Value = TruthManagementStore(T);

    return Content(Value, Generics(Value) {
        .merge=Value.merge,
        .eq=Value.eq,
        .add=Value.add,
    });
}
