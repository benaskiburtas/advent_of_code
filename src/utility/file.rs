use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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

    reader.lines().collect::<Result<Vec<String>, io::Error>>().unwrap()
}