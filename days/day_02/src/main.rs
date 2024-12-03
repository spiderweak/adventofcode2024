use util::load_puzzle_example;
use util::load_puzzle_input;

fn main() {
    let day = "02";

    match load_puzzle_example(day) {
        Ok(example) => {
            let (e_01, e_02) = solve(example);
            println!("Example, part 1 : {}", e_01);
            println!("Example, part 2 : {}", e_02);

        }
        Err(err) => {
            eprintln!("Error: {}", err)
        }

    }
    match load_puzzle_input(day) {
        Ok(input) => {
            let (s_01, s_02) = solve(input);
            println!("Solution, part 1 : {}", s_01);
            println!("Solution, part 2 : {}", s_02);

        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}

fn solve(input: String) -> (i32, i32) {
    let s_01: i32 = solve_01(&input);
    let s_02: i32 = solve_02(&input);

    return (s_01, s_02)
}

fn solve_01(input: &str) -> i32 {
    input
    .split('\n')
    .map(|line|{
        line
        .split(" ")
        .filter_map(|x| x.parse::<i32>().ok())
        .collect::<Vec<i32>>()
    })
    .filter(|report| safe_01(report))
    .count() as i32
}

fn safe_01(report : &[i32]) -> bool {
    !report.is_empty()
    && (report.iter().is_sorted() || report.iter().rev().is_sorted())
    && report
        .iter()
        .zip(report.iter().skip(1))
        .all( |(&cur, &next)| (1..=3).contains(&(next - cur).abs()))
}

fn solve_02(input: &str) -> i32 {
    input
    .split('\n')
    .map(|line|{
        line
        .split(" ")
        .filter_map(|x| x.parse::<i32>().ok())
        .collect::<Vec<i32>>()
    })
    .filter(|report| safe_02(report))
    .count() as i32
}

fn safe_02(report : &[i32]) -> bool {
    safe_01(report)
    || (0..report.len())
        .any(|index| {
            safe_01(
                &report
                .iter()
                .enumerate()
                .filter_map(|(i, &val)| if i != index {Some(val)} else {None})
                .collect::<Vec<i32>>()
            )
        })
}
