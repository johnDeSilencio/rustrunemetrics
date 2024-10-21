use crate::types::*;
use anyhow::Result;
use const_format::formatcp;

const BASE_API_URL: &str = "https://apps.runescape.com/runemetrics";
const PROFILE_API_URL: &str = formatcp!("{BASE_API_URL}/profile/profile");
const QUESTS_API_URL: &str = formatcp!("{BASE_API_URL}/quests");

pub fn get_profile(player_name: String) -> Result<PlayerProfile> {
    let player_profile: PlayerProfile =
        ureq::get(format!("{PROFILE_API_URL}?user={player_name}").as_str())
            .call()?
            .into_json()?;

    Ok(player_profile)
}

pub fn get_quests(player_name: String) -> Result<Vec<PlayerQuestStatus>> {
    let player_quest_status: Vec<PlayerQuestStatus> =
        ureq::get(format!("{QUESTS_API_URL}?user={player_name}").as_str())
            .call()?
            .into_json()?;

    Ok(player_quest_status)
}
