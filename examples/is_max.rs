extern crate rustrunemetrics;
use rustrunemetrics::client;

use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "Interact with the RuneMetrics API.", long_about = None)]
struct Args {
    /// Name of the player whose stats you want to query
    #[arg(short, long)]
    player_name: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let player_name = args.player_name;
    let player_profile = client::get_profile(player_name)?;

    println!("{:?}", player_profile);

    Ok(())
}
