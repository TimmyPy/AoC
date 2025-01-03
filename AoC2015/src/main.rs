mod day1;
mod day2;
mod day3;

fn main() {
    let ans_day1_p1 = day1::solution_part1();
    let ans_day1_p2 = day1::solution_part2();

    println!("Day 1 p1 answer: {ans_day1_p1}");
    println!("Day 1 p2 answer: {ans_day1_p2}");

    let ans_day2_p1 = day2::solution_part1();
    let ans_day2_p2 = day2::solution_part2();

    println!("Day 2 p1 answer: {ans_day2_p1}");
    println!("Day 2 p2 answer: {ans_day2_p2}");

    let ans_day3_p1 = day3::solution_part1();
    let ans_day3_p2 = day3::solution_part2();

    println!("Day 3 p1 answer: {ans_day3_p1}");
    println!("Day 3 p2 answer: {ans_day3_p2}");

}

