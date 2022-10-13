use clap::Parser;
use anyhow::Result;
use projector::{opts::Opts, config::Config};

fn main() -> Result<()>{
    let opts: Config = Opts::parse().try_into()?;
    println!("{:?}", opts);

    return Ok(())
}
