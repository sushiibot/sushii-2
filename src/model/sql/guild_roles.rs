use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Default, Serialize, Clone, Debug)]
pub struct GuildRole {
    pub search: String,
    pub primary: u64,
    pub secondary: Option<u64>,
}

#[derive(Deserialize, Default, Serialize, Clone, Debug)]
pub struct GuildGroup {
    pub limit: u64,
    pub roles: HashMap<String, GuildRole>,
}

#[derive(Deserialize, Default, Serialize, Clone, Debug)]
pub struct GuildRoles {
    #[serde(flatten)]
    pub groups: HashMap<String, GuildGroup>,
}
