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
const ArraySet = @import("array_set.zig").ArraySet;

//TODO offset reserved premises
//TODO enum
const False = 0;
const True = 1;

pub fn Support(comptime T: type) type {
    return struct {
        const Self = @This();

        value: T,
        premises: []Premise,
        allocator: *Allocator,

        pub fn init(allocator: *Allocator, value: T, premises: []const Premise) Self {
            var premise_arr = allocator.alloc(Premise, premises.len) catch unreachable;

            mem.copy(Premise, premise_arr[0..], premises);
            
            sort(Premise, premise_arr[0..], asc(Premise));

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
                if (premise < other.premises[i]) {
                    return true;
                }
            }

            return false;
        }

        pub fn eq(self: Self, other: Self) bool {
            if (!self.value.eq(other.value)) {
                return false;
            }

            return ArraySet(T).eq(Premise, self.premises, other.premises);
        }

        pub fn add(self: Self, other: Self) Self {
            const premises = ArraySet(T)._union(Premise, self.allocator, self.premises, other.premises, asc(Premise));
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
                    if (!ArraySet(T).eq(Premise, self.premises, other.premises) &&
                        ArraySet(T).is_subset(Premise, other.premises, self.premises)) 
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
                const premises = ArraySet(T)._union(Premise, self.allocator, self.premises, other.premises);

                return Self.init(self.allocator, merged_value, premises);
            }
        }

        pub fn subsumes(self: *Self, other: *Self) bool {
            const is_value_eq = self.value == self.value.merge(other.value);
            const is_premise_subset = ArraySet(T).is_subset(self.premises, other.premises);

            return is_value_eq and is_premise_subset;
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

        fn any_subsumes(self: *Self, other_support: SupportT) bool {
            for (self.supports) |self_support| {
                if (self_support.subsumes(other_support)) {
                    return true;
                }
            }
        }

        fn assimilate(self: *Self, other: *Self) Self {
            var new = Self.init(self.allocator, self.context, [_]SupportT{});
            var any_subsumes = false;

            var i : usize = 0;
            var supports = self.allocator.alloc(SupportT, self.supports.len * other.supports.len) catch unreachable; //FIXME

            for (other.supports) |other_support| {
                if (self.any_subsumes(other_support)) { break; }

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

        fn strongest_consequence(self: *Self) Self {
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
                    if (support) { support = support.merge(&current_support); } 
                    else { support = current_support; }
                }
            }
            
            return Self.init(self.allocator, self.contest, [support]);
        }

        pub fn merge(self: Self, other: Self) ?Self  {
            var candidate = self.assimilate(&other);
            var consequence = candidate.strongest_consequence();

            return candidate.assimilate(&consequence);
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
            const n = self.supports.len + other.supports.len;
            var supports = self.allocator.alloc(SupportT, n) catch unreachable; //FIXME
            var i : usize = 0;

            for (self.supports) |self_support| {
                for (other.supports) |other_support| {
                    supports[i] = SupportT.add(self.allocator, self_support, other_support);
                    i += 1;
                }
            }

            sort(SupportT, supports[0..], SupportT.compare_asc);
            
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

pub fn TruthManagementContent(comptime T: type) type {
    const Value = TruthManagementStore(T);

    return Content(Value, Generics(Value) {
        .merge=Value.merge,
        .eq=Value.eq,
        .add=Value.add,
    });
}
