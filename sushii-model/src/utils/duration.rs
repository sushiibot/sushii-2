use chrono::Duration;
use core::time::Duration as StdDuration;
use humantime::DurationError;
use lazy_static::lazy_static;
use regex::Regex;

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

/// Returns start and end byte range of the duration
pub fn find_duration(s: &str) -> Option<regex::Match> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?:(?:\d+\s*(?:nanos|nsec|ns|usec|us|millis|msec|ms|seconds|second|secs|sec|s|minutes|minute|min|mins|m|hours|hour|hrs|hr|h|days|day|d|weeks|week|w|months|month|M|years|year|y))(?:\s|\b))+"
        ).unwrap();
    }

    RE.find(s)
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

    #[test]
    fn finds_durations() {
        let strs = vec![
            "3000s",
            "300sec",
            "300 secs",
            "50seconds",
            "1 second",
            "100m",
            "12min",
            "12mins",
            "1minute",
            "7minutes",
            "2h",
            "7hr",
            "7hrs",
            "1hour",
            "24hours",
            "1day",
            "2days",
            "365d",
            "1week",
            "7weeks",
        ];

        for s in strs {
            assert!(find_duration(s).is_some());
        }
    }
}
