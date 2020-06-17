use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "albatross")]
pub struct Opt {
    /// URL to scan
    #[structopt(short = "c", long = "config", required(true))]
    pub config: String,
}