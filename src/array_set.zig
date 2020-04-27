const std = @import("std");
const sort = std.sort.sort;
const mem = std.mem;
const Allocator = std.mem.Allocator;

pub const PropagatorID = usize;
pub const CellID = usize;


pub fn ArraySet(comptime T: type) type {
    return struct {
        pub fn remove_duplicates(
            allocator: *Allocator, 
            xs: []const T
        ) []T {
            var out = allocator.alloc(T, xs.len) catch unreachable; //FIXME
            var tmp = allocator.alloc(T, xs.len) catch unreachable;
            defer allocator.free(tmp);

            mem.copy(T, tmp[0..], xs);

            sort(T, tmp[0..], T.less_than);

            var i : usize = 0;
            var j : usize = 0;

            while (i < xs.len - 1) {
                if (!tmp[i].eq(tmp[i + 1])) {
                    out[j] = tmp[i].clone();
                    j += 1;
                }
                i += 1;
            }

            out[j] = tmp[xs.len - 1];
            out = allocator.realloc(out, j + 1) catch unreachable; //FIXME

            return out;
        }

        pub fn _union(
            allocator: *Allocator, 
            xs: []const T, 
            ys: []const T
        ) []T {
            const n = xs.len + ys.len;
            var out = allocator.alloc(T, n) catch unreachable; //FIXME
            var tmp = allocator.alloc(T, n) catch unreachable;
            defer allocator.free(tmp);

            mem.copy(T, tmp[0..xs.len], xs);
            mem.copy(T, tmp[xs.len..], ys);

            sort(T, tmp[0..], T.less_than);

            var i : usize = 0;
            var j : usize = 0;

            while (i < n - 1) {
                if (!tmp[i].eq(tmp[i + 1])) {
                    out[j] = tmp[i].clone();
                    j += 1;
                }
                i += 1;
            }

            out[j] = tmp[n - 1];
            out = allocator.realloc(out, j + 1) catch unreachable; //FIXME

            return out;
        }

        pub fn eq(xs: []T, ys: []T) bool {
            if (xs.len != ys.len) {
                return false;
            }

            for (xs) |x, i| {
                if (!x.eq(ys[i])) { return false; }
            }

            return true;
        }

        //FIXME consistent naming
        pub fn is_subset(xs: []const T, ys: []const T) bool {
            var i: usize = 0;

            if (xs.len > ys.len) { return false; }

            while (i < xs.len) {
                if (!xs[i].eq(ys[i])) {
                    return false;
                }

                i += 1;
            }

            return true;
        }
    };
}


