const std = @import("std");
const Content = @import("content.zig").Content;
const Generics = @import("content.zig").Generics;

//FIXME equality checks
pub fn merge(a: f64, b: f64) ?f64 {
  return if (a == b) a else null;
}

pub fn add(a: f64, b: f64) ?f64 {
  return a + b;
}

pub fn sub(a: f64, b: f64) ?f64 {
  return a - b;
}

pub const Decimal = Content(f64, f64, Generics(f64, f64) { 
    .merge=merge, 
    .add=add,
    .sub=sub 
});

