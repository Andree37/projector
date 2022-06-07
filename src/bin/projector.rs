use clap::Parser;
use projector::config::Config;
use projector::opts::Opts;

use anyhow::Result;

fn main() -> Result<()> {
    let opts: Config = Opts::parse().try_into()?;
    println!("{:?}", opts);

    return Ok(());
}
