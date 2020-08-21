use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Default, Serialize, Clone, Debug)]
pub struct GuildRole {
    /// Main role id that has the highest priority for the colour
    pub primary_id: u64,

    /// Secondary role that is lower in priority than **all** other primary roles
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_id: Option<u64>,
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
