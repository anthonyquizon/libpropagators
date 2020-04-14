const std = @import("std");
const ArrayList = std.ArrayList;

pub fn TruthManagementContext(comptime T: type) type {
    const Mutation = union(enum) {
        AmbChoose: CellID
    };

    const NoGood = struct {
        set: ArrayList(T)
    };

    return struct {
        const Self = @This;

        premise_outness: ArrayList(T),
        premise_nogoods: ArrayList(T, NoGood),

        pub fn init(allocator: *Allocator) Self {
            return Self {
                premise_outness: ArrayList(T).init(allocator),
                premise_nogoods: ArrayList(T).init(allocator),
            };
        }

        pub fn premise_in(self: *Self, premise: T) Self {
            for (self.premise_outness.items) |premise| {
                if (premise == T) {
                    return true;
                }
            }

            return false;
        }

        pub fn run_mutation(self: *Self) void {
        }
    }
}
