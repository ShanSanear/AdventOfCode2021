use std::fs;
use std::fs::File;
use std::io::{BufReader, BufRead, Read};
use std::path::Path;

pub fn load_file(file_path : &Path) -> Vec<String> {
 let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let lines = reader.lines().map(|line| line.unwrap()).collect();

    lines
}