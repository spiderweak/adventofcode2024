use std::collections::btree_map::Range;
use std::vec;

use nalgebra::Dyn;
use nalgebra::{Matrix, DMatrix};

use util::load_puzzle_example;
use util::load_puzzle_input;

fn main() {
    let day = "04";

    match load_puzzle_example(day) {
        Ok(example) => {
            println!("{}", example);
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

fn load_data(input: &str) -> Matrix<char, Dyn, Dyn, nalgebra::VecStorage<char, Dyn, Dyn>> {
    let height = input.trim().lines().count();

    let data: Vec<char> = input
    .trim()
    .chars()
    .filter(|c| *c != '\n')
    .collect();

    let width = data.len() / height;

    DMatrix::from_vec(height, width, data)
}

fn solve_01(input: &str) -> i32 {
    let input_matrix = load_data(input);

    let mut count_xmas = 0;

    // Original orientation
    count_xmas += detect_xmas_01(&input_matrix);

    // Rotated orientations
    let mut rotated_matrix = input_matrix.clone();
    for _ in 0..3 {
        rotated_matrix = rotate_matrix(&rotated_matrix);
        count_xmas += detect_xmas_01(&rotated_matrix);
    }
    count_xmas
}


fn solve_02(input: &str) -> i32 {
    let input_matrix = load_data(input);

    detect_xmas_02(&input_matrix)
}



fn detect_xmas_01(matrix: &Matrix<char, Dyn, Dyn, nalgebra::VecStorage<char, Dyn, Dyn>>) -> i32{
    let mut count_xmas = 0;

    let height = matrix.nrows();
    let width = matrix.ncols();

    for y in 0..height {
        for x in 0..width-3 {
            if matrix[(y,x)] == 'X'
            && matrix[(y,x+1)] == 'M'
            && matrix[(y,x+2)] == 'A'
            && matrix[(y,x+3)] == 'S' {
                count_xmas +=1
            }
            if y < height - 3
            && matrix[(y,x)] == 'X'
            && matrix[(y+1,x+1)] == 'M'
            && matrix[(y+2,x+2)] == 'A'
            && matrix[(y+3,x+3)] == 'S' {
                count_xmas +=1
            }
        }
    }
    count_xmas
}

fn rotate_matrix(
    matrix: &Matrix<char, Dyn, Dyn, nalgebra::VecStorage<char, Dyn, Dyn>>,
) -> Matrix<char, Dyn, Dyn, nalgebra::VecStorage<char, Dyn, Dyn>> {
    let height = matrix.nrows();
    let width = matrix.ncols();
    let mut rotated = DMatrix::from_element(width, height, ' ');

    for y in 0..height {
        for x in 0..width {
            rotated[(x, height - y - 1)] = matrix[(y,x)];
        }
    }
    rotated
}

fn detect_xmas_02(matrix: &Matrix<char, Dyn, Dyn, nalgebra::VecStorage<char, Dyn, Dyn>>) -> i32{

    let mut count_xmas = 0;

    let height = matrix.nrows();
    let width = matrix.ncols();

    for y in 1..height-1 {
        for x in 1..width-1 {
            if matrix[(y,x)] == 'A'{
                if (matrix[(y-1,x-1)] == 'M' || matrix[(y-1,x-1)] == 'S')
                && (matrix[(y+1,x+1)] == 'M' || matrix[(y+1,x+1)] == 'S')
                && matrix[(y-1,x-1)] != matrix[(y+1,x+1)]
                && (matrix[(y-1,x+1)] == 'M' || matrix[(y-1,x+1)] == 'S')
                && (matrix[(y+1,x-1)] == 'M' || matrix[(y+1,x-1)] == 'S')
                && matrix[(y-1,x+1)] != matrix[(y+1,x-1)] {
                    count_xmas +=1
                }
            }
        }
    }
    count_xmas

}

