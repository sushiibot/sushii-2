use chrono::{DateTime, Duration, Utc};
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
    humantime::parse_duration(&s).map_err(|e| match e {
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

// Formats a duration and truncates smaller units if longer than 1 day
pub fn format_duration(now: &DateTime<Utc>, before: &DateTime<Utc>) -> String {
    let dur_secs = Duration::seconds(now.signed_duration_since(*before).num_seconds());
    let days = dur_secs.num_days();

    if days >= 7 {
        format!("{} days", days)
    } else {
        humantime::format_duration(dur_secs.to_std().unwrap()).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_str_with_end() {
        let s = point_str("0123456789", 0, Some(9));

        // End is exclusive so it won't point at 9
        assert_eq!(
            s,
            "```
0123456789
^^^^^^^^^
```"
        );
    }

    #[test]
    fn point_str_substr() {
        let s = point_str("0123456789", 1, Some(6));

        assert_eq!(
            s,
            "```
0123456789
 ^^^^^
```"
        );
    }

    #[test]
    fn point_str_single() {
        let s = point_str("0123456789", 7, None);

        assert_eq!(
            s,
            "```
0123456789
       ^
```"
        );
    }
}
