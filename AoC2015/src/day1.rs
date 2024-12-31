
pub fn day1_task_p1(input: &str) -> i32 {
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

pub fn day1_task_p2(input: &str) -> i32 {
    let mut floor: i32 = 0;

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
