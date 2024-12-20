use regex::Regex;

use std::collections::btree_map::Entry;
use std::collections::HashMap;

use util::load_puzzle_example;
use util::load_puzzle_input;

fn main() {
    let day = "05";

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
    let input_parts: Vec<&str> = input.split("\n\n").collect();
    let first_part = input_parts[0];
    let second_part = input_parts[1];

    let order = Regex::new(r"(\d+)\|(\d+)").unwrap();

    let mut before_map: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut after_map: HashMap<i32, Vec<i32>> = HashMap::new();

    for (_, [val_01, val_02]) in order.captures_iter(first_part).map(|c| c.extract()) {
        before_map.entry(val_01.parse::<i32>().ok().unwrap()).or_insert_with(Vec::new).push(val_02.parse::<i32>().ok().unwrap());
        after_map.entry(val_02.parse::<i32>().ok().unwrap()).or_insert_with(Vec::new).push(val_01.parse::<i32>().ok().unwrap());
    }

    second_part
    .lines()
    .map(|line| process(line, &before_map, &after_map))
    .sum()

}


fn solve_02(input: &str) -> i32 {
    let input_parts: Vec<&str> = input.split("\n\n").collect();
    let first_part = input_parts[0];
    let second_part = input_parts[1];

    let order = Regex::new(r"(\d+)\|(\d+)").unwrap();

    let mut before_map: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut after_map: HashMap<i32, Vec<i32>> = HashMap::new();

    for (_, [val_01, val_02]) in order.captures_iter(first_part).map(|c| c.extract()) {
        before_map.entry(val_01.parse::<i32>().ok().unwrap()).or_insert_with(Vec::new).push(val_02.parse::<i32>().ok().unwrap());
        after_map.entry(val_02.parse::<i32>().ok().unwrap()).or_insert_with(Vec::new).push(val_01.parse::<i32>().ok().unwrap());
    }

    second_part
    .lines()
    .filter(|line| process(line, &before_map, &after_map) == 0) 
    .map(|line| process(&sort(line, &before_map, &after_map), &before_map, &after_map))
    .sum()

}

fn process(line: &str, before_map: &HashMap<i32, Vec<i32>>, after_map: &HashMap<i32, Vec<i32>>) -> i32 {
    let pages: Vec<i32> = line
    .split(",")
    .map(|p| p.parse::<i32>().ok().unwrap())
    .collect();

    for (index, &page) in pages.iter().enumerate() {
        for &other_page in pages.iter().skip(index+1) {
            if before_map.get(&other_page).map_or(false, |v| v.contains(&page))
            || after_map.get(&page).map_or(false, |v| v.contains(&other_page)) {
                return 0;
            }
        }
    }
    pages[(pages.len() - 1) / 2]
}

fn sort(line: &str, before_map: &HashMap<i32, Vec<i32>>, after_map: &HashMap<i32, Vec<i32>>) -> String {
    let pages: Vec<i32> = line
    .split(",")
    .map(|p| p.parse::<i32>().ok().unwrap())
    .collect();

    let sorted_pages = recursive_sort(pages, before_map, after_map);

    sorted_pages
    .iter()
    .map(|p| p.to_string())
    .collect::<Vec<String>>()
    .join(",")
}

fn recursive_sort(list: Vec<i32>, before_map: &HashMap<i32, Vec<i32>>, after_map: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    if list.is_empty() {
        return vec![];
    }

    let head = list[0];
    let rest: Vec<i32> = list.into_iter().skip(1).collect();

    let smaller_than_head: Vec<i32> = after_map
        .get(&head)
        .unwrap_or(&vec![])
        .iter()
        .filter(|&&x| rest.contains(&x)) // Intersection
        .cloned()
        .collect();

    let larger_than_head: Vec<i32> = before_map
        .get(&head)
        .unwrap_or(&vec![])
        .iter()
        .filter(|&&x| rest.contains(&x)) // Intersection
        .cloned()
        .collect();

    let mut sorted_smaller = recursive_sort(smaller_than_head, before_map, after_map);
    let sorted_larger = recursive_sort(larger_than_head, before_map, after_map);

    sorted_smaller.push(head);
    sorted_smaller.extend(sorted_larger);
    sorted_smaller

}
