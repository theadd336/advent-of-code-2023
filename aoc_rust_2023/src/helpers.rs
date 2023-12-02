use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use crate::AocResult;

pub fn open_data_file(
    problem_num: u8,
) -> AocResult<impl Iterator<Item = Result<String, std::io::Error>>> {
    let mut path = PathBuf::from("aoc_rust_2023/src/data");
    path.push(format!("problem_{problem_num}.txt"));
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader.lines())
}
