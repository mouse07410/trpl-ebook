use std::error::Error;
use regex::Regex;

pub fn remove_file_title(input: &str) -> Result<String, Box<dyn Error>> {
    Ok(Regex::new(r"^%\s(.+)\n").unwrap().replace(input, ""))
}
