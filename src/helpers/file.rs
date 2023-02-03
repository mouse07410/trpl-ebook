use std::io::prelude::*;
use std::error::Error;
use std::path::Path;
use std::fs::File;

pub fn get_file_content<P: AsRef<Path>>(path: P) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(buffer)
}

pub fn write_string_to_file(input: &str, name: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(&Path::new(name))?;

    file.write_all(input.as_bytes())?;

    Ok(())
}
