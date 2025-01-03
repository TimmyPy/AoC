use std::fs;

fn read_input() -> Vec<String> {
    fs::read_to_string("src/inputs/day2.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

pub fn solution_part1() -> u32 {
    let mut ans: u32 = 0;
    let input = read_input();
    for line in input {
        let parts: Vec<u32> = line.split('x')
            .map(|s| s.parse().expect("Failed to parse to i32"))
            .collect();
        if parts.len() == 3 {
            let l = parts[0];
            let w = parts[1];
            let h = parts[2];


            let max_val = std::cmp::max(h, std::cmp::max(l, w));
            let mut extra_val = 0;

            if max_val == l {
                extra_val = h * w;
            } else if max_val == w {
                extra_val = h * l;
            } else {
                extra_val = w * l;
            }

            ans = ans + 2 * l * w + 2 * w * h + 2 * h * l + extra_val;

        } else {
            panic!("Input line contains more or less values than expected: {:?}", parts)

        }
    }
    ans
}

pub fn solution_part2() -> u32 {
    let mut ans: u32 = 0;
    let input = read_input();
    for line in input {
        let mut parts: Vec<u32> = line.split('x')
            .map(|s| s.parse().expect("Failed to parse to i32"))
            .collect();
        if parts.len() == 3 {
            let l = parts[0];
            let w = parts[1];
            let h = parts[2];

            parts.sort();

            let mut bow_len: u32 = 1;
            let mut perimeter: u32 = 0;

            for (i, part) in parts.iter().enumerate() {
                if i < 2 {
                    perimeter += part;
                }
                bow_len *= part;
            }

            ans = ans + 2 * perimeter + bow_len;
        } else {
            panic!("Input line contains more or less values than expected: {:?}", parts)

        }
    }
    ans
}
