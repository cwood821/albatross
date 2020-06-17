use std::fs;
use std::fs::File;
use std::io::{self, BufRead, Error};
use std::io::{BufReader};
use std::path::Path;
extern crate yaml_rust;
use yaml_rust::{YamlLoader, YamlEmitter};

pub struct HTTPTest {
  host: String,
  path: String,
  status: i32 // this could be smaller
} 

pub fn parse_tests(config_path: String) -> Result<Vec<HTTPTest>, Error>  {
  let mut tests = Vec::new();

  let yaml = fs::read_to_string(config_path)?;
  let parsed = YamlLoader::load_from_str(&yaml).unwrap();
  
  let config = &parsed[0];

  let hosts = config["hosts"].as_vec();
  println!("{:#?}", hosts);
  // map structure to HTTPTest structs

  Ok(tests)
}
// Map 

// pub fn get_patterns(url: String) -> Vec<String> {
//   let mut patterns = Vec::new();

//   if let Ok(lines) = read_lines("./src/data/config_patterns") {
//     patterns = lines.filter_map(|line| line.ok())
//       .map(|line| format!("{}/{}", url, line))
//       .collect()
//   }

//   patterns
// }

// // The output is wrapped in a Result to allow matching on errors
// // Returns an Iterator to the Reader of the lines of the file.
// fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
//     let file = File::open(filename)?;
//     Ok(io::BufReader::new(file).lines())
// }