const std = @import("std");
const Allocator = std.mem.Allocator;

pub const PropagatorID = usize;
pub const CellID = usize;


pub fn ArraySet(comptime T: type) type {
    return struct {
        pub fn remove_duplicates(
            allocator: *Allocator, 
            xs: []T, 
            lessThan: fn (lhs: T, rhs: T) bool
        ) []T {
            var out = allocator.alloc(T, xs.len) catch unreachable; //FIXME
            var tmp = allocator.alloc(T, xs.len) catch unreachable;
            defer allocator.free(tmp);

            mem.copy(T, tmp[0..], xs);

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

        pub fn _union(
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

        pub fn eq(xs: []T, ys: []T) bool {
            if (xy.len != ys.len) {
                return false;
            }

            for (xs) |x, i| {
                if (x != ys[i]) { return false; }
            }

            return true;
        }

        //FIXME consistent naming
        pub fn is_subset(xs: []T, ys: []T) bool {
            var i: usize = 0;

            if (xs.len > ys.len) { return false; }

            while (i < xs.len) {
                if (xs[i] != ys[i]) {
                    return false;
                }

                i += 1;
            }

            return true;
        }
    }
};


