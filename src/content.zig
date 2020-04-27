const std = @import("std");
const warn = @import("std").debug.warn;

pub fn Generics(comptime T: type) type {
  return struct {
    merge: fn(*const T, *const T) ?T,
    eq: fn(T, T) bool = null,
    add: ?fn(T, T) ?T = null,
    sub: ?fn(T, T) ?T = null,
    mul: ?fn(T, T) ?T = null,
    less_than: ?fn(T, T) bool = null,
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

        pub fn eq(self: Self, other: Self) bool {
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
                            return fns.eq(self_value, other_value);
                        },
                        else => { return false; }
                    }
                }
            }
        }

        pub fn less_than(self: Self, other: Self) bool {
            switch (self) {
                .Nothing => {
                    return false;
                },
                .Contradiction => {
                    //FIXME
                    if (other == .Value) { 
                        return true;
                    }

                    return false;
                },
                .Value => |self_value| {
                    switch (other) {
                        .Value => |other_value| {
                            if (fns.less_than) |f| {
                                return f(self_value, other_value);
                            }
                            else {
                                return self_value < other_value;
                            }
                        },
                        else => { return false; }
                    }
                }
            }
        }

        pub fn add(self: Self, other: Self) Self {
          if (fns.add) |f| {
            return lift(self, other, f);
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

        pub fn clone(self: *const Self) Self {
            switch (self.*) {
                .Nothing => { 
                    return .Nothing;
                },
                .Value => |value| {
                    return Self { .Value = value };
                },
                .Contradiction => {
                    return .Contradiction;
                }
            }
        }

        pub fn merge(self: *const Self, other: *const Self) Self {
            switch (self.*) {
                .Nothing => { 
                    return other.clone(); 
                },
                .Value => |old_value| {
                  switch (other.*) {
                      .Nothing => { return .Nothing; },
                      .Value => |value| {
                          var new_value = fns.merge(&old_value, &value);

                          if (new_value) |val| {
                            return self.clone();
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


