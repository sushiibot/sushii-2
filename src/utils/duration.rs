use chrono::Duration;
use core::time::Duration as StdDuration;
use humantime::DurationError;

fn point_str(s: &str, pos: usize, end: Option<usize>) -> String {
    format!(
        "```\n{}\n{}{}\n```",
        s,
        " ".repeat(pos),
        "^".repeat(end.map_or(1, |e| e - pos)) // End is exclusive
    )
}

pub fn parse_duration_std(s: &str) -> Result<StdDuration, String> {
    humantime::parse_duration(&s)
        .map_err(|e| match e {
            DurationError::InvalidCharacter(pos) => format!(
                "Invalid character (only alphanumeric characters are allowed):\n{}",
                point_str(&s, pos, None),
            ),
            DurationError::NumberExpected(pos) => format!(
                "Expected a number.\nThis usually means that either the time \
                        unit is separated (e.g. `m in` instead of `min`) \
                        or a number is omitted (e.g. `2 hours min` instead \
                        of `2 hours 1 min`):\n\
                        {}",
                point_str(&s, pos, None),
            ),
            DurationError::UnknownUnit { start, end, .. } => format!(
                "Invalid time unit, valid units are:\n\
                    `seconds (second, sec, s),\n\
                    minutes (minute, min, m),\n\
                    hours (hour, hr, h),\n\
                    days (day, d),\n\
                    weeks (week, w),\n\
                    months (month, M)`:\n{}",
                point_str(&s, start, Some(end)),
            ),
            DurationError::NumberOverflow => "Duration is too long".into(),
            DurationError::Empty => "Duration cannot be empty".into(),
        })
}

pub fn parse_duration(s: &str) -> Result<Duration, String> {
    parse_duration_std(s)
        .and_then(|d| Duration::from_std(d).map_err(|_| "Duration is too long".into()))
}
