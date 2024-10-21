use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::str::FromStr;

#[derive(Debug, Deserialize, Serialize)]
pub struct Activity {
    #[serde(deserialize_with = "parse_date_time")]
    pub date: DateTime<Utc>,
    pub details: String,
    pub text: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SkillValue {
    pub id: Skill,
    pub level: u32,

    // #[serde(deserialize_with = "parse_rank")]
    pub rank: Option<u32>,
    pub xp: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerProfile {
    pub activities: Vec<Activity>,

    #[serde(rename = "combatlevel")]
    pub combat_level: u32,

    #[serde(rename = "loggedIn", deserialize_with = "parse_string_bool")]
    pub logged_in: bool,

    #[serde(rename = "magic")]
    pub magic_xp: u64,

    #[serde(rename = "melee")]
    pub melee_xp: u64,
    pub name: String,

    #[serde(rename = "questscomplete")]
    pub quests_complete: u32,

    #[serde(rename = "questsnotstarted")]
    pub quests_not_started: u32,

    #[serde(rename = "queststarted")]
    pub quests_started: Option<u32>,

    #[serde(rename = "ranged")]
    pub ranged_xp: u64,

    #[serde(deserialize_with = "parse_rank")]
    pub rank: Option<u32>,

    #[serde(rename = "skillvalues")]
    pub skill_values: Vec<SkillValue>,

    #[serde(rename = "totalskill")]
    pub total_skill: u32,

    #[serde(rename = "totalxp")]
    pub total_xp: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerQuests {
    pub quests: Vec<PlayerQuestStatus>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerQuestStatus {
    pub difficulty: u32,
    pub members: bool,

    #[serde(rename = "questPoints")]
    pub quest_points: u32,
    pub status: QuestStatus,
    pub title: String,

    #[serde(rename = "userEligible")]
    pub user_eligible: bool,
}

#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum Skill {
    Attack = 0,
    Defence,
    Strength,
    Constitution,
    Ranged,
    Prayer,
    Magic,
    Cooking,
    Woodcutting,
    Fletching,
    Fishing,
    Firemaking,
    Crafting,
    Smithing,
    Mining,
    Herblore,
    Agility,
    Thieving,
    Slayer,
    Farming,
    Runecrafting,
    Hunter,
    Construction,
    Summoning,
    Dungeoneering,
    Divination,
    Invention,
    Archaeology,
    Necromancy,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum QuestStatus {
    #[serde(rename = "COMPLETED")]
    Completed,

    #[serde(rename = "STARTED")]
    Started,

    #[serde(rename = "NOT_STARTED")]
    NotStarted,
}

fn parse_rank<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw_rank: Option<String> = Option::deserialize(deserializer)?;

    let Some(raw_rank) = raw_rank else {
        return Ok(None);
    };

    // The rank is returned as a string rather than a number,
    // so we have to remove the commas and convert to a u32.
    //
    // For example, "123,456" becomes 123456u32.
    let raw_rank = raw_rank.as_str().replace(",", "");

    let Ok(rank) = str::parse(raw_rank.as_str()) else {
        return Ok(None);
    };

    Ok(Some(rank))
}

fn parse_string_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;

    // The metrics server encodes these bools in a string, for some strange reason
    let raw_bool: String = Deserialize::deserialize(deserializer)?;
    let parsed_bool = FromStr::from_str(raw_bool.as_str()).map_err(Error::custom)?;

    Ok(parsed_bool)
}

fn parse_date_time<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;

    let raw_date_time: String = Deserialize::deserialize(deserializer)?;
    let parsed_date_time = NaiveDateTime::parse_from_str(raw_date_time.as_str(), "%d-%b-%Y %H:%M")
        .map_err(Error::custom)?
        .and_utc();

    Ok(parsed_date_time)
}
