use chrono::{DateTime, Utc};
use lingua::Language;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use twilight_model::channel::message::Message;
use twilight_model::gateway::event::DispatchEvent;
use twilight_model::user::User;

use crate::error::Result;
use crate::model::RuleContext;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, JsonSchema)]
#[serde(rename_all(serialize = "UPPERCASE", deserialize = "UPPERCASE"))]
#[serde(remote = "Language")]
pub enum LanguageType {
    Afrikaans,
    Albanian,
    Arabic,
    Armenian,
    Azerbaijani,
    Basque,
    Belarusian,
    Bengali,
    Bokmal,
    Bosnian,
    Bulgarian,
    Catalan,
    Chinese,
    Croatian,
    Czech,
    Danish,
    Dutch,
    English,
    Esperanto,
    Estonian,
    Finnish,
    French,
    Ganda,
    Georgian,
    German,
    Greek,
    Gujarati,
    Hebrew,
    Hindi,
    Hungarian,
    Icelandic,
    Indonesian,
    Irish,
    Italian,
    Japanese,
    Kazakh,
    Korean,
    Latin,
    Latvian,
    Lithuanian,
    Macedonian,
    Malay,
    Maori,
    Marathi,
    Mongolian,
    Nynorsk,
    Persian,
    Polish,
    Portuguese,
    Punjabi,
    Romanian,
    Russian,
    Serbian,
    Shona,
    Slovak,
    Slovene,
    Somali,
    Sotho,
    Spanish,
    Swahili,
    Swedish,
    Tagalog,
    Tamil,
    Telugu,
    Thai,
    Tsonga,
    Tswana,
    Turkish,
    Ukrainian,
    Urdu,
    Vietnamese,
    Welsh,
    Xhosa,
    Yoruba,
    Zulu,
}

// This is needed so that we can use the remote Language struct
#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct LanguageWrapper(#[serde(with = "LanguageType")] Language);

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum StringConstraint {
    /// # Equals
    /// Equals some text
    Equals(String),
    /// Does not equal some text
    NotEquals(String),
    /// Contains some text
    Contains(String),
    /// Contains all of the given texts
    ContainsAll(Vec<String>),
    /// Contains at least one of the given texts
    ContainsAny(Vec<String>),
    /// Does not contain the given text
    DoesNotContain(String),
    /// Does not contain any of the given texts
    DoesNotContainAny(Vec<String>),
    /// Is any of the of given texts
    In(Vec<String>),
    /// Is not any of the given texts
    NotIn(Vec<String>),
    /// Starts with given text
    StartsWith(String),
    /// Does not start with given text
    DoesNotStartsWith(String),
    /// Ends with given text
    EndsWith(String),
    /// Does not end with given text
    DoesNotEndsWith(String),
    // Expensive constraints, rules should short circuit with these last
    // TODO: Implement Ord on these so that language constraints are last, then
    // sort the rule constraints so that these are last
    /// # Is language
    ///
    /// This will only match if the relative difference between multiple
    /// language matches are high enough, basically only when sushii is
    /// confident it is a single language.
    ///
    /// Short text is likely to have multiple languages that may match e.g.
    /// prologue matches English and French. So this will *not* match unless the
    /// most likely language has a significantly higher probability than other
    /// languages. This means the longer the text the more likely to have a
    /// language detected.
    #[serde(with = "LanguageType")]
    IsLanguage(Language),
    /// # Is not a language
    /// This will also not match unless the probability of a single language
    /// match is significantly higher than others, e.g. for longer text.
    #[serde(with = "LanguageType")]
    IsNotLanguage(Language),
    /// # Is any of the given languages
    IsInLanguage(Vec<LanguageWrapper>),
    /// # Is not any of the given languages
    IsNotInLanguage(Vec<LanguageWrapper>),
}

impl StringConstraint {
    #[rustfmt::skip]
    pub async fn check_string(&self, ctx: &RuleContext, in_str: &str) -> Result<bool> {
        let res = match self {
            Self::Equals(s) => {
                in_str == *s
            }
            Self::NotEquals(s) => {
                in_str != *s
            }
            Self::Contains(s) => {
                in_str.contains(s)
            }
            Self::ContainsAll(strs) => {
                strs.iter().all(|s| in_str.contains(s))
            },
            Self::ContainsAny(strs) => {
                strs.iter().any(|s| in_str.contains(s))
            },
            Self::DoesNotContain(s) => {
                !in_str.contains(s)
            },
            Self::DoesNotContainAny(strs) => {
                !strs.iter().all(|s| in_str.contains(s))
            },
            Self::In(strs) => {
                strs.iter().all(|s| s.contains(&in_str))
            }
            Self::NotIn(strs) => {
                !strs.iter().all(|s| s.contains(&in_str))
            }
            Self::StartsWith(s) => {
                in_str.starts_with(s)
            }
            Self::DoesNotStartsWith(s) => {
                !in_str.starts_with(s)
            }
            Self::EndsWith(s) => {
                in_str.ends_with(s)
            }
            Self::DoesNotEndsWith(s) => {
                !in_str.ends_with(s)
            }
            Self::IsLanguage(lang) => {
                ctx.language_client
                    .detect_language(in_str)
                    .await?
                    .map(|detected_lang| detected_lang == *lang)
                    .unwrap_or(false)
            }
            Self::IsNotLanguage(lang) => {
                ctx.language_client
                    .detect_language(in_str)
                    .await?
                    .map(|detected_lang| detected_lang != *lang)
                    .unwrap_or(false)
            }
            Self::IsInLanguage(langs) => {
                ctx.language_client
                    .detect_language(in_str)
                    .await?
                    .map(|detected_lang| langs.contains(&LanguageWrapper(detected_lang)))
                    .unwrap_or(false)
            }
            Self::IsNotInLanguage(langs) => {
                ctx.language_client
                    .detect_language(in_str)
                    .await?
                    .map(|detected_lang| !langs.contains(&LanguageWrapper(detected_lang)))
                    .unwrap_or(false)
            }
        };

        Ok(res)
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum IntegerConstraint {
    /// # Equals
    /// Equals given number
    Equals(u64),
    /// # Not equals
    /// Does not equal given number
    NotEquals(u64),
    /// # Greater than
    /// Is greater than given number
    GreaterThan(u64),
    /// # Less than 
    /// Is less than given number
    LessThan(u64),
    /// # Inclusive between
    /// Is between given range, including the range boundaries
    InclusiveBetween { lower: u64, upper: u64 },
    /// # Exclusive between
    /// Is between given range, excluding the range boundaries
    ExclusiveBetween { lower: u64, upper: u64 },
}

impl IntegerConstraint {
    #[rustfmt::skip]
    pub async fn check_integer(&self, ctx: &RuleContext, input: u64) -> Result<bool> {
        let res = match *self {
            Self::Equals(target) => input == target,
            Self::NotEquals(target) => input != target,
            Self::GreaterThan(target) => input > target,
            Self::LessThan(target) => input < target,
            Self::InclusiveBetween { lower, upper } => lower <= input && input <= upper,
            Self::ExclusiveBetween { lower, upper } => lower < input && input < upper,
        };

        Ok(res)
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum IntegerListConstraint {
    /// # Includes
    /// List of numbers includes given number
    Includes(u64),
    /// # Does not include
    /// List of numbers does not include given number
    DoesNotInclude(u64),
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum BoolConstraint {
    /// # Equals
    /// Equals given value
    Equals(bool),
    /// # Not equals
    /// Does not equal given value
    NotEquals(bool),
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DateConstraint {
    /// # Equals
    /// Equals given date
    Equals(DateTime<Utc>),
    /// # Not equals
    /// Does not equal given date
    NotEquals(DateTime<Utc>),
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum UserConstraint {
    /// # Username
    Username(StringConstraint),
    /// # ID
    Id(IntegerConstraint),
    /// # Is bot
    IsBot(BoolConstraint),
    /// # Is verified bot
    IsVerifiedBot(BoolConstraint),
    /// # Server level
    ServerLevel(IntegerConstraint),
    /// # Server XP
    ServerXp(IntegerConstraint),
    /// # Global level
    /// The user's level in **all** servers they share with sushii combined
    GlobalLevel(IntegerConstraint),
    /// # Global XP
    /// The user's XP in **all** servers they share with sushii combined
    GlobalXp(IntegerConstraint),
}

impl UserConstraint {
    async fn check_event(&self, ctx: &RuleContext, user: &User) -> Result<bool> {
        let val = match self {
            UserConstraint::Username(s) => s.check_string(ctx, &user.name).await?,
            UserConstraint::Id(s) => s.check_integer(ctx, user.id.0).await?,
            _ => {
                tracing::warn!("Unhandled author constraint check");

                false
            }
        };

        Ok(val)
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MemberConstraint {
    /// # Deaf
    /// If member can hear in voice channels
    Deaf(BoolConstraint),
    /// # Mute
    /// If member can mute in voice channels
    Mute(BoolConstraint),
    /// # Joined date
    /// When a member joined the server
    JoinedAt(DateConstraint),
    /// # Nickname
    /// Member's nickname in the server
    Nickname(StringConstraint),
    /// # Roles
    /// Member's roles
    Roles(IntegerListConstraint),
    /// # Pending
    /// If the member hasn't accepted the rules of the server yet
    Pending(BoolConstraint),
    /// # Boosting date
    /// When the member boosted the server
    PremiumSince(DateConstraint),
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MessageConstraint {
    /// # Message content
    Content(StringConstraint),
    /// # Message author
    Author(UserConstraint),
    /// # Member
    Member(MemberConstraint),
    /// # Created at
    /// When this message was sent
    CreatedAt(DateConstraint),
}

impl MessageConstraint {
    async fn check_event(&self, ctx: &RuleContext, msg: &Message) -> Result<bool> {
        let val = match self {
            MessageConstraint::Content(s) => s.check_string(ctx, &msg.content).await?,
            MessageConstraint::Author(author) => author.check_event(ctx, &msg.author).await?,
            _ => {
                tracing::warn!("Unhandled message constraint check");

                return Ok(false);
            }
        };

        Ok(val)
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Constraint {
    /// # On message
    Message(MessageConstraint),
}

impl Constraint {
    pub async fn check_event(&self, event: Arc<DispatchEvent>, ctx: &RuleContext) -> Result<bool> {
        let val = match event.as_ref() {
            // MESSAGE_CREATE
            DispatchEvent::MessageCreate(msg) => match self {
                Constraint::Message(msg_constraint) => {
                    msg_constraint.check_event(ctx, &msg.0).await?
                }
            },
            _ => {
                tracing::warn!("Unhandled event");

                return Ok(false);
            }
        };

        Ok(val)
    }
}
