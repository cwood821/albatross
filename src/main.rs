use reqwest::Client;
use futures::future::join_all;
use reqwest::StatusCode;
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
    let failures: Vec<(&reqwest::Response, &http_tests::HTTPTest)> = results.iter().zip(tests.iter()).filter_map(|res| {
      res.0.as_ref().map_or(None, |result| { 
        // TODO: some more involved match generic match testing beyond only supporting status
        if result.status() != StatusCode::from_u16(res.1.status as u16).unwrap() { 
          Some((res.0.as_ref().unwrap(), res.1))
        }
        else {
          None
        }
      })
    }).collect();

    let request_count = results.len(); 

    if failures.is_empty() {
        println!("All tests passed");
        Ok(())
    } else {
        println!("Failures (expected:received)");
        for failure in failures {
            println!("{} {}:{}", 
                failure.0.to_owned().url(), 
                failure.1.to_owned().status, 
                failure.0.to_owned().status().as_u16());
        }
        // TODO: Return non-zero exit code 
        Ok(())
    }
  } else {
    eprintln!("Failed to parse configuration");
    Ok(())
  }

}
