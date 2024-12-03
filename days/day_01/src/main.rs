use std::collections::HashMap;

use util::load_puzzle_example;
use util::load_puzzle_input;

fn main() {
    let day = "01";

    match load_puzzle_example(day) {
        Ok(example) => {
            // println!("Puzzle example for Day {}:\n{}", day, example);
            // Not printing this anymore
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
            // println!("Puzzle input for Day {}:\n{}", day, input);
            // Not printing this anymore
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
    let lines: Vec<&str> = input.split('\n').collect();

    let mut col_01: Vec<i32> = Vec::new();
    let mut col_02: Vec<i32> = Vec::new();

    for line in lines {
        let data: Vec<&str> = line.split(' ').collect();
        //let (left, right) = (, data[data.len()-1])
        if let (Ok(value_01), Ok(value_02)) = (
            data[0].parse::<i32>(),
            data[data.len()-1].parse::<i32>(),
        ){
            col_01.push(value_01);
            col_02.push(value_02)
        }
    }

    col_01.sort();
    col_02.sort();

    let result: i32 = col_01
        .iter()
        .zip(col_02.iter())
        .map(|(x, y)| (y - x).abs())
        .sum();

    return result;
}

fn solve_02(input: &str) -> i32 {
    let lines: Vec<&str> = input.split('\n').collect();

    let mut col_01: Vec<i32> = Vec::new();
    let mut col_02: Vec<i32> = Vec::new();

    for line in lines {
        let data: Vec<&str> = line.split(' ').collect();
        //let (left, right) = (, data[data.len()-1])
        if let (Ok(value_01), Ok(value_02)) = (
            data[0].parse::<i32>(),
            data[data.len()-1].parse::<i32>(),
        ){
            col_01.push(value_01);
            col_02.push(value_02)
        }
    }

    // Factor these, utility function ?
    let mut counts_01: HashMap<i32, usize> = HashMap::new();
    let mut counts_02: HashMap<i32, usize> = HashMap::new();

    for &value in &col_01 {
        *counts_01.entry(value).or_insert(0) += 1;
    }

    for &value in &col_02 {
        *counts_02.entry(value).or_insert(0) += 1;
    }

    let mut result = 0;

    for (&value, &count_01) in &counts_01 {
        if let Some(&count_02) = counts_02.get(&value) {
            result += value * (count_01 as i32 * count_02 as i32);
        }
    }

    return result;
}