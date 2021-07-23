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

        // Need to + start_idx since the rfind returns the index starting from start_idx
        let end_idx = s[start_idx..end_idx_max]
            .rfind('\n')
            .map_or(end_idx_max, |idx| idx + start_idx);

        if start_idx == end_idx {
            break;
        }

        v.push(&s[start_idx..end_idx]);
        start_idx = end_idx;
    }

    v
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn splits_embed_messages_single_line_single() {
        let s = "hello world ".repeat(200);
        let v = split_embed_messages(&s);

        assert_eq!(v[0].len(), 2400);
    }

    #[test]
    fn splits_embed_messages_single_line_two() {
        let s = "hello world ".repeat(500);
        let v = split_embed_messages(&s);

        assert_eq!(v[0].len(), 4096);
        assert_eq!(v[1].len(), 6000 - 4096);
    }

    #[test]
    fn splits_embed_messages_single_line_multiple() {
        let s = "hello world ".repeat(1000);
        let v = split_embed_messages(&s);

        assert_eq!(v.len(), 3);
        assert_eq!(v[2].len(), 12000 - 4096 - 4096);
    }

    #[test]
    fn splits_embed_messages_multi_line() {
        let s = "hello world\n".repeat(1000);
        let v = split_embed_messages(&s);

        assert_eq!(v.len(), 3);

        for m in &v {
            assert!(m.len() < 4096);
        }

        assert_eq!(v.into_iter().collect::<String>(), s.trim());
    }

    #[test]
    fn splits_embed_messages_large() {
        let s = format!("{}\n", "asodifj".repeat(200)).repeat(10);
        let v = split_embed_messages(&s);

        for m in &v {
            assert!(m.len() < 4096);
        }

        assert_eq!(v.into_iter().collect::<String>(), s.trim());
    }
}
