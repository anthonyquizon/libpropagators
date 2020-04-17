const std = @import("std");
const Allocator = std.mem.Allocator;

pub const PropagatorID = usize;
pub const CellID = usize;


pub const Array = struct {
    pub fn sorted_append(
        comptime T: type, 
        allocator: *Allocator, 
        xs: []T, 
        ys: []T, 
        lessThan: fn (lhs: T, rhs: T) bool
    ) []T {
        const n = xs.len + ys.len;
        var out = allocator.alloc(T, n) catch unreachable; //FIXME
        var tmp = allocator.alloc(T, n) catch unreachable;
        defer allocator.free(tmp);

        mem.copy(T, tmp[0..as.len], as);
        mem.copy(T, tmp[as.len..], bs);

        sort(T, tmp[0..], lessThan);

        var i : usize = 0;
        var j : usize = 0;

        while (i < n - 1) {
            if (tmp[i] != tmp[i + 1]) {
                out[j] = tmp[i];
                j += 1;
            }
            i += 1;
        }

        out[j] = tmp[n - 1];
        out = allocator.realloc(out, j + 1) catch unreachable; //FIXME

        return out;
    }
};
