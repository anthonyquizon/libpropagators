

pub fn Content(comptime T: type, merge: fn(T, T) ?T) type {
    return union(enum) {
        const Self = @This();

        Nothing,
        Value: T,
        Contradiction,

        pub fn init() Self {
          return .Nothing;
        }

        pub fn merge(self: *Self, value: T) Self {
            switch (self.content) {
                .Nothing => {
                  return Self { .Value = value };
                },
                Content.Value => |old_value| {
                  var new_value = merge(old_value, value);

                  if (new_value) |val| {
                    return Self { .Value = val };
                  } else {
                    return .Contradiction;
                  }
                },
                .Contradiction => {
                    return .Contradiction;
                }
            }
        }
    };
}


