const std = @import("std");
const fs = std.fs;
const mem = std.mem;

pub fn part1() !u32 {
    var depth: u32 = 0;
    var horizontal: u32 = 0;

    var file = try fs.cwd().openFile("src/solutions/inputs/day2.txt", .{});
    defer file.close();

    var buffred = std.io.bufferedReader(file.reader());
    var bufreader = buffred.reader();

    var buffer: [128]u8 = undefined;
    @memset(buffer[0..], 0);

    while (try bufreader.readUntilDelimiterOrEof(buffer[0..], '\n')) |line| {
        var it = mem.splitSequence(u8, line[0..], " ");
        const direction = it.first();
        const move = try std.fmt.parseInt(u32, it.next().?, 10);
        if (mem.eql(u8, direction[0..], "forward")) {
            horizontal += move;
        } else if (mem.eql(u8, direction[0..], "down")) {
            depth += move;
        } else if (mem.eql(u8, direction[0..], "up")) {
            depth -= move;
        } else {
            @panic("Unknown direction");
        }
    }

    return depth * horizontal;
}

pub fn part2() !u32 {
    var depth: u32 = 0;
    var horizontal: u32 = 0;
    var aim: u32 = 0;

    var file = try fs.cwd().openFile("src/solutions/inputs/day2.txt", .{});
    defer file.close();

    var buffred = std.io.bufferedReader(file.reader());
    var bufreader = buffred.reader();

    var buffer: [128]u8 = undefined;
    @memset(buffer[0..], 0);

    while (try bufreader.readUntilDelimiterOrEof(buffer[0..], '\n')) |line| {
        var it = mem.splitSequence(u8, line[0..], " ");
        const direction = it.first();
        const move = try std.fmt.parseInt(u32, it.next().?, 10);
        if (mem.eql(u8, direction[0..], "forward")) {
            horizontal += move;
            depth = depth + aim * move;
        } else if (mem.eql(u8, direction[0..], "down")) {
            aim += move;
        } else if (mem.eql(u8, direction[0..], "up")) {
            aim -= move;
        } else {
            @panic("Unknown direction");
        }
    }

    return depth * horizontal;
}
