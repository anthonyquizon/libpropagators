const std = @import("std");
const Content = @import("content.zig").Content;
const Generics = @import("content.zig").Generics;

//FIXME equality checks
pub fn merge(a: *const f64, b: *const f64) ?f64 {
  return if (a.* == b.*) a.* else null;
}

pub fn eq(a: f64, b: f64) bool {
  return a == b;
}

pub fn add(a: f64, b: f64) ?f64 {
  return a + b;
}

pub fn sub(a: f64, b: f64) ?f64 {
  return a - b;
}

pub fn less_than(a: f64, b: f64) bool {
  return a < b;
}

pub const Decimal = Content(f64, Generics(f64) { 
    .merge=merge, 
    .eq=eq,
    .add=add,
    .sub=sub,
    .less_than=less_than
});

