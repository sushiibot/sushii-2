pub fn escape_markdown(s: &str) -> String {
    s.replace("*", "\\*")
        .replace("_", "\\_")
        .replace("`", "\\`")
        .replace("~", "\\~")
        .replace("|", "\\|")
}
