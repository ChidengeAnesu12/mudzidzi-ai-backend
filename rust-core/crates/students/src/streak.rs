use chrono::NaiveDate;

/// Computes the current consecutive-day study streak from a
/// descending list of distinct activity dates. The streak counts
/// backward from today; if there's no activity yet today, it still
/// counts backward starting from yesterday — so a streak doesn't drop
/// to 0 the instant midnight passes, before the student has had a
/// chance to study that day.
pub fn compute_streak(dates_desc: &[NaiveDate], today: NaiveDate) -> i32 {
    if dates_desc.is_empty() {
        return 0;
    }

    let yesterday = today.pred_opt().unwrap_or(today);

    let mut expected = if dates_desc[0] == today {
        today
    } else if dates_desc[0] == yesterday {
        yesterday
    } else {
        return 0;
    };

    let mut streak = 0;
    for &date in dates_desc {
        if date == expected {
            streak += 1;
            expected = expected.pred_opt().unwrap_or(expected);
        } else if date < expected {
            break;
        }
    }
    streak
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    #[test]
    fn empty_history_has_no_streak() {
        assert_eq!(compute_streak(&[], date(2026, 6, 27)), 0);
    }

    #[test]
    fn studied_today_only() {
        let dates = vec![date(2026, 6, 27)];
        assert_eq!(compute_streak(&dates, date(2026, 6, 27)), 1);
    }

    #[test]
    fn consecutive_days_including_today() {
        let dates = vec![date(2026, 6, 27), date(2026, 6, 26), date(2026, 6, 25)];
        assert_eq!(compute_streak(&dates, date(2026, 6, 27)), 3);
    }

    #[test]
    fn streak_continues_if_not_yet_studied_today() {
        let dates = vec![date(2026, 6, 26), date(2026, 6, 25)];
        assert_eq!(compute_streak(&dates, date(2026, 6, 27)), 2);
    }

    #[test]
    fn gap_breaks_the_streak() {
        let dates = vec![date(2026, 6, 27), date(2026, 6, 25)];
        assert_eq!(compute_streak(&dates, date(2026, 6, 27)), 1);
    }

    #[test]
    fn stale_history_has_no_current_streak() {
        let dates = vec![date(2026, 6, 1)];
        assert_eq!(compute_streak(&dates, date(2026, 6, 27)), 0);
    }
}