use std::fs;

fn check_rules(line: &str) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut vowels_count: u8 = 0;
    let noughty_pairs = ["ab", "cd", "pq", "xy"];
    let mut has_double = false;

    for (i, c) in line.chars().enumerate() {
        if vowels.contains(&c) {
            vowels_count += 1;
        }
        if i > 0 && i < line.len() {
            let pair = &line[i - 1..i + 1];
            if noughty_pairs.contains(&pair) {
                println!("The line {} contains a noughty_pair {}", line, pair);
                return false;
            }

            if !has_double {
                let pair_bytes = &pair.as_bytes();
                if pair_bytes[0] as char == c && pair_bytes[1] as char == c {
                    has_double = true;
                }
            }
        }
    }

    has_double && vowels_count >= 3

    /*
        if vowels_count < 3 {
            println!("Line: {} does not contains 3 vowels", line);
            return false;
        }

        if !has_double {
            println!("Line: {} does not contains doubles", line);
            return false;
        }

        println!("The line {} is nice", line,);

        true
    */
}

fn check_rules_part2(line: &str) -> bool {
    let mut has_pairs = false;
    let mut has_repeats = false;
    for (c, i) in line.chars().enumerate() {}

    true
}

fn read_line() -> Vec<String> {
    fs::read_to_string("src/inputs/day5.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

pub fn solution_part1() -> u32 {
    let mut counter: u32 = 0;
    for line in read_line() {
        if check_rules(&line) {
            counter += 1;
        }
    }
    counter
}

pub fn solution_part2() -> u32 {
    let mut counter: u32 = 0;
    for line in read_line() {
        if check_rules_part2(&line) {
            counter += 1;
        }
    }
    counter
}
