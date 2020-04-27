const std = @import("std");
const CellID = @import("util.zig").CellID;
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;

pub fn Premise(comptime T: type) type {
    return union(enum) {
        const Self = @This();

        HypotheticalTrue: CellID,
        HypotheticalFalse: CellID,
        Supplied: T,

        pub fn from(value: T) Self {
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
}

const Action = union(enum) {
    AmbChoose: CellID
};

pub fn TruthManagementContext(comptime T: type) type {
    const NoGood = struct {
        set: ArrayList(T)
    };

    return struct {
        const Self = @This();

        premise_outness: ArrayList(T),
        premise_nogoods: ArrayList(NoGood),

        pub fn init(allocator: *Allocator) Self {
            return Self {
                .premise_outness=ArrayList(T).init(allocator),
                .premise_nogoods=ArrayList(NoGood).init(allocator),
            };
        }

        pub fn is_premise_in(self: *const Self, premise: T) bool {
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
}
