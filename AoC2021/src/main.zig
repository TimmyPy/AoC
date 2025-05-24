const std = @import("std");

const day1 = @import("solutions/day1.zig");
const day2 = @import("solutions/day2.zig");

pub fn main() !void {
    // Prints to stderr (it's a shortcut based on `std.io.getStdErr()`)

    // stdout is for the actual output of your application, for example if you
    // are implementing gzip, then only the compressed bytes should be sent to
    // stdout, not any debugging messages.
    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    // day1
    const day1_ans: u16 = try day1.solution1();
    try stdout.print("Solution day 1: {}\n", .{day1_ans});
    const day1_part2_ans: u16 = try day1.solution2();
    try stdout.print("Solution day 1 part 2: {}\n", .{day1_part2_ans});

    // day2
    const day2_part1: u64 = try day2.part1();
    try stdout.print("Solution day 2: {}\n", .{day2_part1});
    const day2_part2: u64 = try day2.part2();
    try stdout.print("Solution day 2 part 2: {}\n", .{day2_part2});

    try bw.flush(); // Don't forget to flush!

}

/// This imports the separate module containing `root.zig`. Take a look in `build.zig` for details.
const lib = @import("AoC2021_lib");
