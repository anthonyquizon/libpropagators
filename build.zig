const Builder = @import("std").build.Builder;

const Example = struct { 
    name: []const u8, 
    src: []const u8 
};

pub fn build(b: *Builder) void {

    const mode = b.standardReleaseOptions();
    const lib = b.addStaticLibrary("propagator", "src/lib.zig");
    lib.setBuildMode(mode);
    lib.install();

    var main_tests = b.addTest("src/main.zig");
    main_tests.setBuildMode(mode);

    const test_step = b.step("test", "Run library tests");
    test_step.dependOn(&main_tests.step);

    const examples = [_]Example {
        Example { .name="tms_add", .src="examples/tms_add.zig" }
    };

    const example_step = b.step("examples", "Build examples");
    inline for ([_][]const u8{
        "tms_add",
    }) |example_name| {
        const example = b.addExecutable(example_name, "examples/" ++ example_name ++ ".zig");
        example.addPackagePath("propagators", "src/lib.zig");
        example.setBuildMode(mode);
        example.install();
        example_step.dependOn(&example.step);
    }

}
