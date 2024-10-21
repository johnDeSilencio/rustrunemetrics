use crate::types::*;
use anyhow::Result;
use const_format::formatcp;

const BASE_API_URL: &str = "https://apps.runescape.com/runemetrics";
const PROFILE_API_URL: &str = formatcp!("{BASE_API_URL}/profile/profile");
const QUESTS_API_URL: &str = formatcp!("{BASE_API_URL}/quests");

fn get_profile(player_name: String) -> Result<PlayerProfile> {
    todo!()
}

fn get_quests(player_name: String) -> Result<Vec<PlayerQuestStatus>> {
    todo!()
}
