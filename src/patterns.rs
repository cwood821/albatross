use std::fs::File;
use std::io::{self, BufRead};
use std::io::{BufReader};
use std::path::Path;

pub fn get_patterns(url: String) -> Vec<String> {
  let mut patterns = Vec::new();

  if let Ok(lines) = read_lines("./src/data/config_patterns") {
    patterns = lines.filter_map(|line| line.ok())
      .map(|line| format!("{}/{}", url, line))
      .collect()
  }

  patterns
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}