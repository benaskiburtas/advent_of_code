use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

const PUZZLE_INPUT_FILE: &str = "puzzle_input.txt";

pub fn read_puzzle_input(initial_path: &str) -> Vec<String> {
    let puzzle_input_path = get_puzzle_input_path(initial_path);
    let path = puzzle_input_path.to_str().unwrap();

    read_lines(path)
}

pub fn read_lines(file_path: &str) -> Vec<String> {
    let path = Path::new(file_path);

    if !path.exists() {
        panic!("{}", format!("File '{}' does not exist.", file_path));
    }

    if !path.is_file() {
        panic!("{}", format!("'{}' is not a file.", file_path));
    }

    let file: File = File::open(path).unwrap();
    let reader = io::BufReader::new(file);

    reader
        .lines()
        .collect::<Result<Vec<String>, io::Error>>()
        .unwrap()
}

fn get_puzzle_input_path(initial_path: &str) -> PathBuf {
    Path::new(initial_path)
        .parent()
        .unwrap()
        .join(PUZZLE_INPUT_FILE)
}
