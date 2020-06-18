use reqwest::Client;
use futures::future::join_all;
use structopt::StructOpt;
extern crate yaml_rust;
mod conf;
mod http_tests;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let opt = conf::Opt::from_args();
  let config = opt.config;

  if let Ok(tests) = http_tests::parse_tests(config) {
    let client = Client::builder().build()?;

    let results = join_all(tests.iter().map(|test| {
      client.get(&format!("{}{}", test.host, test.path)).send() 
    })).await;

    // Results are in same order, so tests and results are paired
    let test_results = results.iter().zip(tests.iter());

    let request_count = results.len(); 

    println!("Requests: {}", request_count);
    // println!("Results {:#?}", results);
    println!("Results {:#?}", test_results);
    Ok(())
  } else {
    eprintln!("Failed to parse configuration");
    Ok(())
  }

}
