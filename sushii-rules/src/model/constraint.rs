use lingua::Language;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use twilight_model::gateway::event::DispatchEvent;

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
    /// Is language
    #[serde(with = "LanguageType")]
    IsLanguage(Language),
    #[serde(with = "LanguageType")]
    IsNotLanguage(Language),
    IsInLanguage(Vec<LanguageWrapper>),
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
    Equals(u64),
    NotEquals(u64),
    GreaterThan(u64),
    LessThan(u64),
    Between { lower: u64, upper: u64 },
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum UserConstraint {
    /// # Username
    Username(StringConstraint),
    /// # User ID
    Id(IntegerConstraint),
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MessageConstraint {
    /// # Message content
    Content(StringConstraint),
    /// # Message author
    Author(UserConstraint),
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
            DispatchEvent::MessageCreate(msg) => {
                match self {
                    Constraint::Message(MessageConstraint::Content(s)) => {
                        s.check_string(ctx, &msg.content).await?
                    }
                    Constraint::Message(MessageConstraint::Author(UserConstraint::Username(s))) => {
                        s.check_string(ctx, &msg.author.name).await?
                    }
                    // Add more later, not unimplemented! since that has a lot of panics
                    _ => {
                        tracing::warn!("Unhandled constraint check");

                        return Ok(false);
                    }
                }
            }
            _ => {
                tracing::warn!("Unhandled constraint event");

                return Ok(false);
            }
        };

        Ok(val)
    }
}
