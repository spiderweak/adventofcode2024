use std::fs;

use std::path::{Path, PathBuf};

/// Resolves the full path to the input file for a specific day.
///
/// # Arguments
///
/// * `day` - A zero-padded string representing the day (e.g., "01", "02", ..., "25").
///
/// # Returns
///
/// * `PathBuf` containing the resolved path to the input file.
fn resolve_input_path(day: &str) -> PathBuf {
    let mut path = std::env::current_dir().expect("Failed to get current directory");

    // Traverse to the project root by appending a relative path if needed
    while !path.join("inputs").exists() {
        path = path
            .parent()
            .expect("Reached filesystem root without finding 'inputs'")
            .to_path_buf();
    }

    path.join(format!("inputs/day_{}", day))
}

/// Loads the puzzle example for a specific day.
///
/// # Arguments
///
/// * `day` - A zero-padded string representing the day (e.g., "01", "02", ..., "25").
///
/// # Returns
///
/// * `Ok(String)` containing the input contents if the file exists and is readable.
/// * `Err(String)` with an error message if the input file cannot be found or read.

pub fn load_puzzle_example(day: &str) -> Result<String, String> {
    let path = resolve_input_path(day);
    let example_path = path.join("example");

    if example_path.exists() && example_path.is_file() {
        fs::read_to_string(example_path)
        .map_err(|err| format!("Failed to read example: {}", err))
    } else {
        Err(format!("Example file not found for day {}", day))
    }
}

/// Loads the puzzle input for a specific day.
///
/// # Arguments
///
/// * `day` - A zero-padded string representing the day (e.g., "01", "02", ..., "25").
///
/// # Returns
///
/// * `Ok(String)` containing the input contents if the file exists and is readable.
/// * `Err(String)` with an error message if the input file cannot be found or read.

pub fn load_puzzle_input(day: &str) -> Result<String, String> {
    let path = resolve_input_path(day);
    let input_path = path.join("input");

    if input_path.exists() && input_path.is_file() {
        fs::read_to_string(input_path)
        .map_err(|err| format!("Failed to read input: {}", err))
    } else {
        Err(format!("Input file not found for day {}", day))
    }
}




// Default content for syntax

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
