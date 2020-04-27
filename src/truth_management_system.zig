const std = @import("std");
const assert = @import("std").debug.assert;
const sort = @import("std").sort.sort;
const asc = @import("std").sort.asc;
const mem = @import("std").mem;
const ArrayList = std.ArrayList;
const ContentBase = @import("content.zig").Content;
const Generics = @import("content.zig").Generics;
const Allocator = std.mem.Allocator;
const ArraySet = @import("array_set.zig").ArraySet;
const CellID = @import("util.zig").CellID;

//TODO offset reserved premises
//TODO enum

pub fn TruthManagementSystem(
    comptime T: type, 
    comptime P: type
) type {
    return struct {
        pub const Premise = union(enum) {
            const Self = @This();

            HypotheticalTrue: CellID,
            HypotheticalFalse: CellID,
            Supplied: P,

            pub fn init(value: P) Self {
                return Self { .Supplied=value };
            }

            pub fn clone(self: *const Self) Self {
                switch (self.*) {
                    .HypotheticalTrue => |cell_id| {
                        return Self { .HypotheticalTrue=cell_id };
                    },
                    .HypotheticalFalse => |cell_id| {
                        return Self { .HypotheticalFalse=cell_id };
                    },
                    .Supplied => |value| {
                        return Self { .Supplied=value };
                    }
                }
            }

            pub fn eq(self: Self, other: Self) bool {
                switch (self) {
                    .HypotheticalTrue => |self_cell_id| {
                        switch (other) {
                            .HypotheticalTrue => |other_cell_id| {
                                return self_cell_id == other_cell_id;
                            },
                            else => return false
                        }
                    },
                    .HypotheticalFalse => |self_cell_id| {
                        switch (other) {
                            .HypotheticalFalse => |other_cell_id| {
                                return self_cell_id == other_cell_id;
                            },
                            else => return false
                        }
                    },
                    .Supplied => |self_value| {
                        switch (other) {
                            .Supplied => |other_value| {
                                return @enumToInt(self_value) == @enumToInt(other_value);
                            },
                            else => return false
                        }
                    }
                }
            }

            pub fn less_than(self: Self, other: Self) bool {
                switch (self) {
                    .HypotheticalTrue => |self_cell_id| {
                        switch (other) {
                            .HypotheticalTrue => |other_cell_id| {
                                return self_cell_id < other_cell_id;
                            },
                            else => return false
                        }
                    },
                    .HypotheticalFalse => |self_cell_id| {
                        switch (other) {
                            .HypotheticalFalse => |other_cell_id| {
                                return self_cell_id < other_cell_id;
                            },
                            else => return false
                        }
                    },
                    .Supplied => |self_value| {
                        switch (other) {
                            .Supplied => |other_value| {
                                return @enumToInt(self_value) < @enumToInt(other_value);
                            },
                            else => return false
                        }
                    }
                }
            }
        };

        const Action = union(enum) {
            AmbChoose: CellID
        };

        const NoGood = struct {
            set: ArrayList(Premise)
        };

        pub const Context = struct {
            const Self = @This();

            premise_outness: ArrayList(Premise),
            premise_nogoods: ArrayList(NoGood),

            pub fn init(allocator: *Allocator) Self {
                return Self {
                    .premise_outness=ArrayList(Premise).init(allocator),
                    .premise_nogoods=ArrayList(NoGood).init(allocator),
                };
            }

            pub fn is_premise_in(self: *const Self, premise: Premise) bool {
                for (self.premise_outness.items) |out_premise| {
                    if (out_premise.eq(premise)) {
                        return false;
                    }
                }

                return true;
            }

            pub fn run_action(self: *Self, action: Action) void {
                //switch (action) {
                    //Action.AmbChoose => |cell_id| {
                        //const true_premise = 
                    //}
                //}
                
            }
        };

        pub const Support = struct {
            const Self = @This();

            value: T,
            premises: []Premise,
            allocator: *Allocator,

            pub fn init(allocator: *Allocator, value: T, supplied_premises: []const P) Self {
                var premises = allocator.alloc(Premise, supplied_premises.len) catch unreachable;

                for (supplied_premises) |supplied, i| {
                    premises[i] = Premise.init(supplied);
                }

                sort(Premise, premises[0..], Premise.less_than);

                return Self {
                    .value=value,
                    .premises=premises,
                    .allocator=allocator
                };
            }

            pub fn less_than(self: Self, other: Self) bool {
                //std.debug.warn("{} {}", .{self, other});
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

                return ArraySet(Premise).eq(self.premises, other.premises);
            }

            pub fn add(self: Self, other: Self) Self {
                const new = self.clone();

                new.value = self.value.add(other.value);
                new.premises = ArraySet(Premise)._union(self.allocator, self.premises, other.premises);
                
                return new;
            }

            pub fn clone(self: Self) Self {
                var premises = self.allocator.alloc(Premise, self.premises.len) catch unreachable;

                mem.copy(Premise, premises[0..], self.premises);

                return Self {
                    .allocator=self.allocator,
                    .value=self.value,
                    .premises=premises
                };
            }

            pub fn merge(self: *Self, other: *Self) Self {
                const merged_value = self.value.merge(&other.value);

                if (merged_value.eq(self.value)) {
                    if (other.value.merge(&merged_value).eq(other.value)) {
                        if (!ArraySet(Premise).eq(self.premises, other.premises) and
                            ArraySet(Premise).is_subset(other.premises, self.premises)) 
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
                    const new = Self.init(self.allocator, merged_value, &[_]P {});

                    new.premises = ArraySet(Premise)._union(self.allocator, self.premises, other.premises);

                    return new;
                }
            }

            pub fn subsumes(self: *const Self, other: *const Self) bool {
                const is_value_eq = self.value.eq(self.value.merge(&other.value));
                const is_premise_subset = ArraySet(Premise).is_subset(self.premises, other.premises);

                return is_value_eq and is_premise_subset;
            }
        };

        pub const Store = struct {
            const Self = @This();

            context: *Context,
            supports: []Support,
            allocator: *Allocator,

            pub fn init(allocator: *Allocator, context: *Context, supports: []Support) Self {
                var support_arr = allocator.alloc(Support, supports.len) catch unreachable;

                mem.copy(Support, support_arr[0..], supports);
                
                sort(Support, support_arr[0..], Support.less_than);

                return Self {
                    .context=context,
                    .supports=support_arr,
                    .allocator=allocator
                };
            }

            pub fn clone(self: Self) Self {
                return Self.init(self.allocator, self.context, self.supports);
            }

            fn any_subsumes(self: *const Self, other_support: *const Support) bool {
                for (self.supports) |self_support| {
                    if (self_support.subsumes(other_support)) {
                        return true;
                    }
                }

                return false;
            }

            fn assimilate(self: *const Self, other: *const Self) Self {
                var new = Self.init(self.allocator, self.context, &[_]Support{});

                var i : usize = 0;
                var supports = self.allocator.alloc(Support, self.supports.len * other.supports.len) catch unreachable; //FIXME

                for (other.supports) |other_support| {
                    if (self.any_subsumes(&other_support)) { break; }

                    for (self.supports) |support| {
                        if (!support.subsumes(&other_support)) {
                            supports[i] = other_support.clone();
                            i += 1;
                        }
                    }
                }

                supports = self.allocator.realloc(supports, i) catch unreachable; //FIXME
                new.supports = ArraySet(Support).remove_duplicates(self.allocator, supports);
                
                return new;
            }

            fn strongest_consequence(self: *const Self) Self {
                if (self.supports.len == 0) {
                    return self.clone();
                }

                var support : Support = self.supports[0];
                var i : usize = 1;

                while (i < self.supports.len) {
                    var all_valid = true;

                    for (self.supports[i].premises) |premise| {
                        if (self.context.is_premise_in(premise)) {
                            all_valid = false;
                            break;
                        }
                    }

                    if (all_valid) {
                        support = support.merge(&self.supports[i]); 
                    }

                    i += 1;
                }
                
                return Self.init(self.allocator, self.context, &[_]Support{support});
            }

            pub fn merge(self: *const Self, other: *const Self) ?Self  {
                var candidate = self.assimilate(other);
                var consequence = candidate.strongest_consequence();

                return candidate.assimilate(&consequence);
            }

            pub fn eq(self: Self, other: Self) bool {
                if (self.supports.len != other.supports.len) {
                    return false;
                }

                for (self.supports) |support, i| {
                    if (!Support.eq(support, other.supports[i])) {
                        return false;
                    }
                }

                return true;
            }

            pub fn add(self: Self, other: Self) ?Self  {
                const n = self.supports.len + other.supports.len;
                var supports = self.allocator.alloc(Support, n) catch unreachable; //FIXME
                var i : usize = 0;

                for (self.supports) |self_support| {
                    for (other.supports) |other_support| {
                        supports[i] = Support.add(self_support, other_support);
                        i += 1;
                    }
                }

                @breakpoint();

                sort(Support, supports[0..], Support.less_than);
                
                return Self {
                    .context=self.context,
                    .supports=supports,
                    .allocator=self.allocator
                };
            }
        };

        pub const Content = ContentBase(Store, Generics(Store) {
            .merge=Store.merge,
            .eq=Store.eq,
            .add=Store.add,
        });
    };
}
