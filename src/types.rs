use serde::{Deserialize, Serialize};

use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, Serialize)]
struct Activity {
    date: DateTime<Utc>,
    details: String,
    text: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct SkillValue {
    id: Skill,
    level: u32,
    rank: u32,
    xp: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct PlayerProfile {
    activities: Vec<Activity>,

    #[serde(rename = "combatlevel")]
    combat_level: u32,

    #[serde(rename = "loggedIn")]
    logged_in: bool,

    #[serde(rename = "magic")]
    magic_xp: u32,

    #[serde(rename = "melee")]
    melee_xp: u32,
    name: String,

    #[serde(rename = "questscomplete")]
    quests_complete: u32,

    #[serde(rename = "questsnotstarted")]
    quests_not_started: u32,

    #[serde(rename = "questsstarted")]
    quests_started: u32,

    #[serde(rename = "ranged")]
    ranged_xp: u32,
    rank: u32,

    #[serde(rename = "skillvalues")]
    skill_values: Vec<SkillValue>,

    #[serde(rename = "totalskill")]
    total_skill: u32,

    #[serde(rename = "total_xp")]
    total_xp: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct PlayerQuestStatus {
    difficulty: u32,
    members: bool,

    #[serde(rename = "questPoints")]
    quest_points: u32,
    status: QuestStatus,
    title: String,

    #[serde(rename = "userEligible")]
    user_eligible: bool,
}

#[derive(Debug, Deserialize, Serialize)]
enum Skill {
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
enum QuestStatus {
    Completed,
    Started,
    NotStarted,
}
