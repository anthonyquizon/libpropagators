const std = @import("std");
const warn = @import("std").debug.warn;

pub fn Generics(comptime T: type) type {
  return struct {
    merge: fn(T, T) ?T,
    add: ?fn(T, T) ?T = null,
    sub: ?fn(T, T) ?T = null,
    mul: ?fn(T, T) ?T = null,
  };
}

pub fn Content(comptime T: type, fns: Generics(T)) type {
    return union(enum) {
        const Self = @This();

        Nothing,
        Value: T,
        Contradiction,

        pub fn init() Self {
          return .Nothing;
        }

        pub fn from(value: T) Self {
          return Self { .Value = value };
        }

        pub fn equals(self: Self, other: Self) bool {
          switch (self) {
            .Nothing => {
              return other == .Nothing;
            },
            .Contradiction => {
              return other == .Contradiction;
            },
            .Value => |self_value| {
                switch (other) {
                    .Value => |other_value| {
                      return self_value == other_value;
                    },
                    else => { return false; }
                }
            }
          }
        }

        pub fn add(self: Self, other: Self) Self {
          if (fns.add) |add_value| {
            return lift(self, other, add_value);
          }
          else {
            return .Nothing;
          }
        }

        pub fn sub(self: Self, other: Self) Self {
          if (fns.sub) |f| {
            return lift(self, other, f);
          }
          else {
            return .Nothing;
          }
        }

        pub fn lift(self: Self, other: Self, f: fn(T, T) ?T) Self {
          switch (self) {
            .Value => |self_value| {
                switch (other) {
                    .Value => |other_value| {
                      const new_value = f(self_value, other_value);

                      if (new_value) |val| {
                        return Self { .Value = val };
                      }
                      else { 
                        return .Contradiction;
                      }
                    },
                    else => { 
                      return self;
                    }
                }
            },
            else => {
              return self;
            }
          }
        }

        pub fn is_empty(self: Self) bool {
          if (self == .Nothing) {
            return true;
          }

          return false;
        }

        pub fn merge(self: Self, other: Self) Self {
            switch (self) {
                .Nothing => { return other; },
                .Value => |old_value| {
                  switch (other) {
                      .Nothing => { return .Nothing; },
                      .Value => |value| {
                          var new_value = fns.merge(old_value, value);

                          if (new_value) |val| {
                            return Self { .Value = val };
                          } else {
                            return Self.Contradiction;
                        }
                      },
                      .Contradiction => {
                          return .Contradiction;
                      }
                  }
                },
                .Contradiction => {
                    return .Contradiction;
                }
            }
        }
    };
}


