const CellID = @import("util.zig").CellID;
const PropagatorID = @import("util.zig").PropagatorID;

pub fn Arithmatic(comptime Network: type, comptime T: type) type {
  return struct {
      pub fn Binary(f: fn(T, T) T) type {
          return struct {
              pub fn propagator(network: *Network, a: CellID, b: CellID, c: CellID) PropagatorID {
                  var inputs = [_]CellID {a, b};

                  const Context = struct {
                      fn f_wrapper(args: []T) T {
                          return f(args[0], args[1]);
                      }
                  };

                  return network.make_propagator(Context.f_wrapper, inputs[0..], c);
              }
          };
      }

      pub const add = Binary(T.add).propagator;
      pub const sub = Binary(T.sub).propagator;

      pub fn constraint_add(network: *Network, a: CellID, b: CellID, c: CellID) void {
        _ = add(network, a, b, c);
        _ = sub(network, c, a, b);
        _ = sub(network, c, b, a);
      }
  };
}
