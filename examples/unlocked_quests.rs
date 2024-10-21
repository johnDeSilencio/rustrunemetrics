extern crate rustrunemetrics;
use rustrunemetrics::{client, types::PlayerQuestStatus, types::QuestStatus};

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
    let player_name = args.player_name.clone();
    let player_quests = client::get_quests(player_name)?;

    let incomplete_unlocked_p2p_quests: Vec<&PlayerQuestStatus> = player_quests
        .quests
        .iter()
        .filter(|quest| {
            quest.user_eligible && quest.status != QuestStatus::Completed && quest.members
        })
        .collect();

    let incomplete_unlocked_f2p_quests: Vec<&PlayerQuestStatus> = player_quests
        .quests
        .iter()
        .filter(|quest| {
            quest.user_eligible && quest.status != QuestStatus::Completed && !quest.members
        })
        .collect();

    // Check if there are any incomplete but unlocked F2P quests
    if incomplete_unlocked_f2p_quests.is_empty() {
        println!(
            "\"{}\" has completed all unlocked free quests.",
            args.player_name
        );
    } else {
        println!(
            "\"{}\" can complete the following unlocked free quests:\n",
            args.player_name
        );

        for quest in &incomplete_unlocked_f2p_quests {
            println!("  - {}", quest.title);
        }
    }

    println!();

    // Check if there are any incomplete but unlocked P2P quests
    if incomplete_unlocked_p2p_quests.is_empty() {
        println!("\"{}\" has completed all members quests.", args.player_name);
    } else {
        println!(
            "\"{}\" can complete the following unlocked members quests:\n",
            args.player_name
        );

        for quest in &incomplete_unlocked_p2p_quests {
            println!("  - {}", quest.title);
        }
    }

    println!();

    if incomplete_unlocked_f2p_quests.is_empty() && incomplete_unlocked_p2p_quests.is_empty() {
        println!("Way to go {}!", args.player_name);
    } else {
        println!("Get to work {}!", args.player_name);
    }

    Ok(())
}
