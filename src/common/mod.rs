use std::{
    fs::File,
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
