const std = @import("std");
const fs = std.fs;
const print = std.debug.print;

pub fn solution1() !u16 {
    var ans: u16 = 0;

    // open and read data from file
    var file = try fs.cwd().openFile("src/solutions/inputs/day1.txt", .{});
    defer file.close();

    var buffered = std.io.bufferedReader(file.reader());
    var bufreader = buffered.reader();

    var buffer: [128]u8 = undefined;
    @memset(buffer[0..], 0);

    var c_measurement: u16 = undefined;
    var p_measurement: ?u16 = null;

    while (try bufreader.readUntilDelimiterOrEof(buffer[0..], '\n')) |line| {
        c_measurement = try std.fmt.parseInt(u16, line[0..], 10);
        if (p_measurement != null and p_measurement.? < c_measurement) {
            ans += 1;
        }
        p_measurement = c_measurement;
    }

    return ans;
}
