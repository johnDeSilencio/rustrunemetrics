use serde::{Deserialize, Serialize};

use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, Serialize)]
pub struct Activity {
    pub date: DateTime<Utc>,
    pub details: String,
    pub text: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SkillValue {
    pub id: Skill,
    pub level: u32,
    pub rank: u32,
    pub xp: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerProfile {
    pub activities: Vec<Activity>,

    #[serde(rename = "combatlevel")]
    pub combat_level: u32,

    #[serde(rename = "loggedIn")]
    pub logged_in: bool,

    #[serde(rename = "magic")]
    pub magic_xp: u32,

    #[serde(rename = "melee")]
    pub melee_xp: u32,
    pub name: String,

    #[serde(rename = "questscomplete")]
    pub quests_complete: u32,

    #[serde(rename = "questsnotstarted")]
    pub quests_not_started: u32,

    #[serde(rename = "questsstarted")]
    pub quests_started: u32,

    #[serde(rename = "ranged")]
    pub ranged_xp: u32,
    pub rank: u32,

    #[serde(rename = "skillvalues")]
    pub skill_values: Vec<SkillValue>,

    #[serde(rename = "totalskill")]
    pub total_skill: u32,

    #[serde(rename = "total_xp")]
    pub total_xp: u32,
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

#[derive(Debug, Deserialize, Serialize)]
pub enum Skill {
    Attack,
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

#[derive(Debug, Deserialize, Serialize)]
pub enum QuestStatus {
    Completed,
    Started,
    NotStarted,
}
