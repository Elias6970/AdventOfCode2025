use std::fs;
use std::io;

pub fn read_file_content(filename: &str) -> Result<String, io::Error> {
    let contents = fs::read_to_string(filename)?;

    Ok(contents)
}