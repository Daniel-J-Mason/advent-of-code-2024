use std::error::Error;
use std::{fs, io};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn parse_to_text(input_file_name: &str) -> Result<String, Box<dyn Error>> {
    let path = Path::new("inputs").join(input_file_name);
    let content = fs::read_to_string(path)?;
    Ok(content)
}

pub fn parse_to_array(input_file_name: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let path = Path::new("inputs").join(input_file_name);

    let file = File::open(path)?;

    let reader = BufReader::new(file);

    let lines: Result<Vec<String>, io::Error> = reader.lines().collect();

    lines.map_err(|e| e.into())
}