extern crate rustrunemetrics;
use rustrunemetrics::client;

use anyhow::{format_err, Result};
use clap::Parser;
use std::cmp::Ordering;

#[derive(Parser, Debug)]
#[command(version, about = "Interact with the RuneMetrics API.", long_about = None)]
struct Args {
    /// Name of the player whose stats you want to query
    #[arg(short, long)]
    player_name: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let player_name = args.player_name.clone();
    let player_profile = client::get_profile(player_name)?;

    let minimum_skill = player_profile.skill_values.iter().min_by(|a, b| {
        let Some(a_rank) = a.rank else {
            // If we can't get a's rank, skip this iteration
            return Ordering::Greater;
        };

        let Some(b_rank) = b.rank else {
            // If we can't get b's rank, skip this iteration
            return Ordering::Greater;
        };

        // Reverse compare to sort for the minimum skill
        b_rank.cmp(&a_rank)
    });

    let Some(minimum_skill) = minimum_skill else {
        return Err(format_err!(
            "Could not get the minimum skill for player \"{}\"",
            args.player_name
        ));
    };

    if minimum_skill.level >= 99 {
        println!(
            "\"{}\" has maxed. Way to go {}!",
            args.player_name, args.player_name
        );
    } else {
        println!("\"{}\" has not maxed... yet. 😎", args.player_name);
    }

    Ok(())
}
