use reqwest::Client;
use reqwest::Response;
use futures::future::join_all;
use structopt::StructOpt;
extern crate yaml_rust;
use yaml_rust::{YamlLoader, YamlEmitter};
mod conf;
mod http_tests;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let opt = conf::Opt::from_args();
  let config = opt.config;

  // let patterns = patterns::get_patterns(target_url);
  let patterns: Vec<String> = Vec::new();


  let client = Client::builder().build()?;
  let results = join_all(
    patterns.iter()
    .map(|url| {
        client.get(url).send()
    })
  ).await;

  let request_count = results.len(); 
  let warnings: Vec<Response> = results.into_iter()
    .filter_map(|res| res.ok())
    .filter(|res| res.status() == 200)
    .collect();

  http_tests::parse_tests(config);
  

  println!("Results:");
  println!("Requests: {}", request_count);
  println!("{:#?}", warnings);
  Ok(())
}
