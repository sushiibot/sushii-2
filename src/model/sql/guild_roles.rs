use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

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
    /// Limit default is 0 which means disabled
    #[serde(default)]
    pub limit: u64,
    pub roles: HashMap<String, GuildRole>,
}

#[derive(Deserialize, Default, Serialize, Clone, Debug)]
pub struct GuildRoles {
    #[serde(flatten)]
    pub groups: HashMap<String, GuildGroup>,
}

impl fmt::Display for GuildRoles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "**Role Groups**")?;

        for (i, (group_name, group)) in self.groups.iter().enumerate() {
            writeln!(f, "> **{}**", group_name)?;

            if group.limit > 0 {
                writeln!(f, "> Limit: `{}`", group.limit)?;
            }

            writeln!(
                f,
                "> Roles: {}",
                group
                    .roles
                    .keys()
                    .map(|s| format!("`{}`", s))
                    .collect::<Vec<String>>()
                    .join(", ")
            )?;

            // Add blank line, excluding last one,
            // (groups.len() - 1 won't underflow since loop requires at least 1)
            if i < self.groups.len() - 1 {
                writeln!(f, "")?;
            }
        }

        Ok(())
    }
}

impl GuildRoles {
    pub fn get_examples_string(&self) -> String {
        let roles: Vec<&str> = self
            .groups
            .values()
            .map(|g| g.roles.keys())
            .flatten()
            .map(|r| r.as_str())
            .take(2)
            .collect();

        // Not enough roles for an example so just skip this
        if roles.len() < 2 {
            return "".into();
        }

        format!(
            "Adding a single role: `+{}`\n\
            Removing a single role: `-{}`\n\
            Adding multiple roles: `+{} +{}`\n\
            Adding and removing multiple roles `+{} -{}`",
            roles[0], roles[0], roles[0], roles[1], roles[0], roles[1]
        )
    }
}
