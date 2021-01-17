use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct FeedList {
    pub feeds: Vec<FeedKindAttrs>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeedKindAttrs {
    /// Type of feed, e.g "twitter" or "youtube"
    /// Does NOT include vlive, vlive is hardcoded since it's more trouble to
    /// deal with and doesn't need to be flexible
    pub kind: String,
    pub attributes: Attributes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attributes {
    /// RSS feed path with strfmt replacement parameters
    pub feed_path: String,
    /// RSS feed name with strfmt replacement parameters
    pub name: String,
    /// Data source url, e.g. twitter.com/username
    pub source_url: String,
    /// strfmt replacement parameters names
    pub params: HashMap<String, Param>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ParameterKind {
    String,
    Bool,
    Channel,
    Role,
}

/// Query parameters, eg. noretweets, noreplies
#[derive(Debug, Serialize, Deserialize)]
pub struct Param {
    /// Parameter type to validate inputs
    pub kind: ParameterKind,
    /// Actual parameter
    pub param: String,
}
