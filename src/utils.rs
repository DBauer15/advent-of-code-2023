use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

pub fn read_lines_in_file(file_name: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(file_name).expect("File not found {file_name}");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|line| line.expect("Unable to parse line"))
        .collect()
}
