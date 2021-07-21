use std::cmp;

pub fn escape_markdown(s: &str) -> String {
    s.replace("*", "\\*")
        .replace("_", "\\_")
        .replace("`", "\\`")
        .replace("~", "\\~")
        .replace("|", "\\|")
}

const MAX_DESC_LEN: usize = 4096;

/// Splits string into separated smaller strings to fit in an embed
/// first item is the description, up to 4096 chars
pub fn split_embed_messages(s: &str) -> Vec<&str> {
    let mut v = Vec::new();

    if s.len() < MAX_DESC_LEN {
        return vec![s];
    }

    let mut start_idx = 0;

    while start_idx < s.len() {
        let end_idx_max = cmp::min(s.len(), start_idx + MAX_DESC_LEN);

        let end_idx = s[start_idx..end_idx_max].rfind('\n').unwrap_or(end_idx_max);

        v.push(&s[start_idx..end_idx]);
        start_idx = end_idx;
    }

    v
}
