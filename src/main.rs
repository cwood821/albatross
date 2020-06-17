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

    let request_count = results.len(); 

    println!("Requests: {}", request_count);
    println!("Results {:#?}", results);
    Ok(())
  } else {
    eprintln!("Failed to parse configuration");
    Ok(())
  }

}
