use anyhow::Result;
use clap::StructOpt;
use rgrep::*;

fn main() -> Result<()>{
    let config: GrepConfig = GrepConfig::parse();
    config.match_with_default_strategy()?;
    Ok(())
}
