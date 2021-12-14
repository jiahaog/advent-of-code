use std::{
    fs::{read_to_string, File},
    io::{BufRead, BufReader},
    path::Path,
};

/// Reads the puzzle input.
///
/// The file is expected to be placed in `$crate_root/src/$day/input`.
pub fn read_input(day: &str) -> impl Iterator<Item = String> {
    let input_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join(day)
        .join("input");

    BufReader::new(File::open(input_path).unwrap())
        .lines()
        .map(|line| line.unwrap())
}

/// Reads the puzzle input to a string.
///
/// The file is expected to be placed in `$crate_root/src/$day/input`.
pub fn read_input_to_string(day: &str) -> String {
    let input_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join(day)
        .join("input");

    read_to_string(input_path).unwrap()
}
