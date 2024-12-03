use regex::Regex;

use util::load_puzzle_example;
use util::load_puzzle_input;

fn main() {
    let day = "03";

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
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let mut results = vec![];
    for (_, [val_01, val_02]) in re.captures_iter(input).map(|c| c.extract()) {
        results.push((val_01.parse::<i32>().ok().unwrap(), val_02.parse::<i32>().ok().unwrap()));
    }

    results
    .iter()
    .map(|(a,b)| a*b)
    .sum()
}


fn solve_02(input: &str) -> i32 {

    let dontdo = Regex::new(r"don't\(\).*?do\(\)").unwrap();
    let dontend = Regex::new(r"don't\(\).*$").unwrap();

    // For whatever reasons the regexp stops maching at end of lines, so we get rid of those
    let preprocessed_input = input.replace('\n', " ");

    let temp_results: std::borrow::Cow<'_, str> = dontdo.replace_all(&preprocessed_input, "");
    let results = dontend.replace_all(&temp_results, "");

    solve_01(&results)
}
