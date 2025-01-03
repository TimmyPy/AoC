use std::fs;

fn read_input() -> String {
    fs::read_to_string("src/inputs/day1.txt").unwrap()

}

pub fn solution_part1() -> i32 {
    let input = read_input();
    let mut ans: i32 = 0;

    for c in input.chars() {
        if c == '(' {
            ans = ans + 1;
        }
        else if c == ')' {
            ans = ans - 1;
        }
        else {
            panic!("Unexpected character was given: {c}");
        }
    }

    ans
}

pub fn solution_part2() -> i32 {
    let mut floor: i32 = 0;
    let input = read_input();

    for (i, c) in input.chars().enumerate() {
        if c == '(' {
            floor = floor + 1;
        }
        else if c == ')' {
            floor = floor - 1;
        }
        else {
            panic!("Unexpected character was given: {c}");
        }

        if floor == -1 {
            return i32::try_from(i + 1).unwrap()
        }
    }

    return i32::try_from(input.len() + 1).unwrap()
}
