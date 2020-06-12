use reqwest::Client;
use std::fs::File;
use std::io::{self, BufRead};
use std::io::{BufReader};
use std::path::Path;
use futures::future::join_all;
use structopt::StructOpt;
mod conf;
mod patterns;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let opt = conf::Opt::from_args();
  let target_url = opt.target_url;

  let patterns = patterns::get_patterns(target_url);

  let client = Client::builder().build()?;
  let values = futures::future::join_all(
    patterns.iter()
    .map(|url| {
        client.get(url).send()
    })
  ).await;

  println!("{:#?}", values);
  Ok(())
}