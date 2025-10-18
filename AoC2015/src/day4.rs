use md5;
use std::fs;

fn read_input() -> String {
    fs::read_to_string("src/inputs/day4.txt").unwrap()
}

pub fn solution_part1() -> u32 {
    let input: String = read_input().trim().to_string();
    let target = "00000";
    let mut i: u32 = 0;

    loop {
        i += 1;

        let candidate = format!("{}{}", input, i);
        let candidate_hash = md5::compute(candidate);
        let str_hash = format!("{:x}", candidate_hash);

        if str_hash[0..5] == target[..] {
            return i;
        }
    }
}

pub fn solution_part2() -> u32 {
    let input: String = read_input().trim().to_string();
    let target = "000000";
    let mut i: u32 = 0;

    loop {
        i += 1;

        let candidate = format!("{}{}", input, i);
        let candidate_hash = md5::compute(candidate);
        let str_hash = format!("{:x}", candidate_hash);

        if str_hash[0..6] == target[..] {
            return i;
        }
    }
}
