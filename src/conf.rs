use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "albatross")]
pub struct Opt {
    /// URL to scan
    #[structopt(short = "t", long = "target", required(true))]
    pub target_url: String,
}