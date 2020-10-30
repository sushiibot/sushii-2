use std::fmt;
use std::str::FromStr;

use crate::error::Error;

pub enum GuildSetting {
    JoinMsg,
    JoinReact,
    LeaveMsg,
    MsgChannel,
    MsgLog,
    ModLog,
    MemberLog,
    MuteDm,
}

impl fmt::Display for GuildSetting {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GuildSetting::JoinMsg => "join message",
                GuildSetting::JoinReact => "join react",
                GuildSetting::LeaveMsg => "leave message",
                GuildSetting::MsgChannel => "message channel",
                GuildSetting::MsgLog => "message log",
                GuildSetting::ModLog => "mod log",
                GuildSetting::MemberLog => "member log",
                GuildSetting::MuteDm => "mute DMs",
            }
        )
    }
}

impl FromStr for GuildSetting {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let setting = match s {
            "joinmsg" => Self::JoinMsg,
            "joinreact" => Self::JoinReact,
            "leavemsg" => Self::LeaveMsg,
            "msgchannel" => Self::MsgChannel,
            "msglog" => Self::MsgLog,
            "modlog" => Self::ModLog,
            "memberlog" => Self::MemberLog,
            "mutedm" => Self::MuteDm,
            _ => return Err(Error::Sushii("Invalid guild setting".into())),
        };

        Ok(setting)
    }
}

pub enum GuildSettingAction {
    Set,
    On,
    Off,
    Toggle,
    Show,
}

impl FromStr for GuildSettingAction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let action = match s {
            "set" => Self::Set,
            "on" => Self::On,
            "off" => Self::Off,
            "toggle" => Self::Toggle,
            "show" => Self::Show,
            _ => return Err(Error::Sushii("Invalid guild setting action".into())),
        };

        Ok(action)
    }
}
