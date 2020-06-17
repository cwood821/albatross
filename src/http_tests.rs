use std::fs;
use std::io::{Error};
extern crate yaml_rust;
use yaml_rust::{YamlLoader};

#[derive(Debug)]
pub struct HTTPTest {
  pub host: String,
  pub path: String,
  pub status: i64 
} 

pub fn parse_tests(config_path: String) -> Result<Vec<HTTPTest>, Error>  {
  let mut mapped_tests: Vec<HTTPTest> = Vec::new();

  let yaml = fs::read_to_string(config_path)?;
  let parsed = YamlLoader::load_from_str(&yaml).unwrap();
  
  let config = &parsed[0];

  // TODO: log failures to parse and refactor to a map structure
  if let Some(hosts) = config["hosts"].to_owned().into_vec() {
    for host in hosts {
      let cur_host = host["host"].as_str().unwrap();
      if let Some(tests) = host["tests"].to_owned().into_vec() {
        for test in tests {
          let new_test = HTTPTest {
            host: cur_host.to_string(),
            path: test["path"].as_str().unwrap().to_string(),
            status: test["status"].as_i64().unwrap()
          };
  
          mapped_tests.push(new_test); 
        }
      }
    }
  }

  Ok(mapped_tests)
}