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
const ArraySet = @import("array_set.zig").ArraySet;

//TODO offset reserved premises
//TODO enum

pub fn Support(comptime T: type, comptime P: type) type {
    return struct {
        const Self = @This();

        value: T,
        premises: []P,
        allocator: *Allocator,

        pub fn init(allocator: *Allocator, value: T, premises: []const P) Self {
            var premise_arr = allocator.alloc(P, premises.len) catch unreachable;

            mem.copy(P, premise_arr[0..], premises);
            
            sort(P, premise_arr[0..], P.less_than);

            return Self {
                .value=value,
                .premises=premise_arr,
                .allocator=allocator
            };
        }

        pub fn compare_asc(self: Self, other: Self) bool {
            if (self.value.less_than(other.value)) {
                return true;
            }

            if (self.premises.len < other.premises.len) {
                return true;
            }

            for (self.premises) |premise, i| {
                if (premise.less_than(other.premises[i])) {
                    return true;
                }
            }

            return false;
        }

        pub fn eq(self: Self, other: Self) bool {
            if (!self.value.eq(other.value)) {
                return false;
            }

            return ArraySet(P).eq(self.premises, other.premises);
        }

        pub fn add(self: Self, other: Self) Self {
            const premises = ArraySet(P)._union(self.allocator, self.premises, other.premises);
            const value = self.value.add(other.value);
            
            return Self.init(self.allocator, value, premises);
        }

        pub fn clone(self: Self) Self {
            return Self.init(self.allocator, self.value, self.premises);
        }

        pub fn merge(self: *Self, other: *Self) Self {
            const merged_value = self.value.merge(other.value);

            if (merged_value.eq(self.value)) {
                if (other.value.merge(merged_value).eq(other.value)) {
                    if (!ArraySet(T).eq(P, self.premises, other.premises) and
                        ArraySet(T).is_subset(P, other.premises, self.premises)) 
                    {
                        return other.clone();
                    }
                    else {
                        return self.clone();
                    }
                }
                else {
                    return self.clone();
                }
            }
            else if (merged_value.eq(other.value)) {
                return other.clone();
            }
            else {
                const premises = ArraySet(P)._union(P, self.allocator, self.premises, other.premises);

                return Self.init(self.allocator, merged_value, premises);
            }
        }

        pub fn subsumes(self: *Self, other: *Self) bool {
            const is_value_eq = self.value.eq(self.value.merge(other.value));
            const is_premise_subset = ArraySet(P).is_subset(self.premises, other.premises);

            return is_value_eq and is_premise_subset;
        }
    };
}

pub fn TruthManagementStore(comptime T: type, comptime P: type) type {
    return struct {
        const Self = @This();

        context: *Context(P),
        supports: []Support(T, P),
        allocator: *Allocator,

        pub fn init(allocator: *Allocator, context: *Context(P), supports: []Support(T, P)) Self {
            var support_arr = allocator.alloc(Support(T, P), supports.len) catch unreachable;

            mem.copy(Support(T, P), support_arr[0..], supports);
            
            sort(Support(T, P), support_arr[0..], Support(T, P).compare_asc);

            return Self {
                .context=context,
                .supports=support_arr,
                .allocator=allocator
            };
        }

        pub fn clone(self: Self) Self {
            return Self.init(self.allocator, self.context, self.supports);
        }

        fn any_subsumes(self: *Self, other_support: *Support(T, P)) bool {
            for (self.supports) |self_support| {
                if (self_support.subsumes(other_support)) {
                    return true;
                }
            }

            return false;
        }

        fn assimilate(self: *const Self, other: *const Self) Self {
            var new = Self.init(self.allocator, self.context, &[_]Support(T, P){});

            var i : usize = 0;
            var supports = self.allocator.alloc(Support(T, P), self.supports.len * other.supports.len) catch unreachable; //FIXME

            for (other.supports) |other_support| {
                if (self.any_subsumes(&other_support)) { break; }

                for (self.supports) |support| {
                    if (!support.subsumes(other_support)) {
                        supports[i] = self.supported;
                        i += 1;
                    }
                }
            }

            supports[i] = other_support;
            supports = self.allocator.realloc(supports, i) catch unreachable; //FIXME
            ArraySet(T).remove_duplicates(supports);

            new.supports = supports;
            
            return new;
        }

        fn strongest_consequence(self: *const Self) Self {
            var support = undefined;

            for (self.supports) |current_support| {
                var all_valid = true;

                for (current_support.premises) |premise| {
                    if (self.context.is_premise_in(premise)) {
                        all_valid = false;
                        break;
                    }
                }

                if (all_valid) {
                    if (support) { support = support.merge(current_support); } 
                    else { support = current_support; }
                }
            }
            
            return Self.init(self.allocator, self.contest, Support(T, P)[_] {support});
        }

        pub fn merge(self: *const Self, other: *const Self) ?Self  {
            //var candidate = self.assimilate(other);
            //var consequence = candidate.strongest_consequence();

            //return candidate.assimilate(consequence);
            return self.clone();
        }

        pub fn eq(self: Self, other: Self) bool {
            if (self.supports.len != other.supports.len) {
                return false;
            }

            for (self.supports) |support, i| {
                if (!Support(T, P).eq(support, other.supports[i])) {
                    return false;
                }
            }

            return true;
        }

        pub fn add(self: Self, other: Self) ?Self  {
            const n = self.supports.len + other.supports.len;
            var supports = self.allocator.alloc(Support(T, P), n) catch unreachable; //FIXME
            var i : usize = 0;

            for (self.supports) |self_support| {
                for (other.supports) |other_support| {
                    supports[i] = Support(T, P).add(self_support, other_support);
                    i += 1;
                }
            }

            sort(Support(T, P), supports[0..], Support(T, P).compare_asc);
            
            return Self {
                .context=self.context,
                .supports=supports,
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
