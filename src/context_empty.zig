
pub const EmptyContext = struct {
    const Self = @This();

    pub fn init() Self {
        return Self {};
    }
};

