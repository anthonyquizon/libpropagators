const std = @import("std");
const CellID = @import("util.zig").CellID;
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;

pub const Premise = usize;

const Mutation = union(enum) {
    AmbChoose: CellID
};

const NoGood = struct {
    set: ArrayList(Premise)
};

pub const TruthManagementContext = struct {
    const Self = @This();

    premise_outness: ArrayList(Premise),
    premise_nogoods: ArrayList(NoGood),

    pub fn init(allocator: *Allocator) Self {
        return Self {
            .premise_outness=ArrayList(Premise).init(allocator),
            .premise_nogoods=ArrayList(NoGood).init(allocator),
        };
    }

    pub fn is_premise_in(self: *Self, premise: Premise) bool {
        for (self.premise_outness.items) |premise| {
            if (premise == Premise) {
                return true;
            }
        }

        return false;
    }

    pub fn run_mutation(self: *Self) void {
    }
};
