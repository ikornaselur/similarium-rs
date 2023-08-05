use chrono::{DateTime, Utc};

const BASE_DAYS_SINCE_CE: i32 = 738_281;

/// Return a puzzle number for today
///
/// The puzzle number is the number of days that have passed since Similarium started, which was
/// the 6th of May 2022
pub fn get_puzzle_number(date: DateTime<Utc>) -> i64 {
    let base_date = chrono::NaiveDate::from_num_days_from_ce_opt(BASE_DAYS_SINCE_CE).unwrap();
    let naive_date = date.date_naive();
    let delta = naive_date - base_date;

    delta.num_days()
}
/// Return the date of a puzzle
///
/// The puzzle date is a nicely formatted date for the puzzle number, such as "Sunday November 13"
/// for puzzle 191
pub fn get_puzzle_date(puzzle_number: i64) -> String {
    let date =
        chrono::NaiveDate::from_num_days_from_ce_opt(BASE_DAYS_SINCE_CE + puzzle_number as i32)
            .unwrap();

    DateTime::<Utc>::from_utc(date.and_hms_opt(0, 0, 0).unwrap(), Utc)
        .format("%A %B %-d")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_that_base_days_const_is_6th_of_may_2022() {
        let base_date = chrono::NaiveDate::from_num_days_from_ce_opt(BASE_DAYS_SINCE_CE).unwrap();

        assert_eq!(
            base_date,
            chrono::NaiveDate::from_ymd_opt(2022, 5, 6).unwrap()
        );
    }

    #[test]
    fn test_get_puzzle_number() {
        // One day after the game started
        let naive_datetime = NaiveDate::from_ymd_opt(2022, 5, 7)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let datetime = DateTime::from_utc(naive_datetime, Utc);

        // Game start was considered puzzle 0
        assert_eq!(get_puzzle_number(datetime), 1);

        // Way later, to confirm the puzzle number matches the current date as this test was
        // written
        let naive_datetime = NaiveDate::from_ymd_opt(2022, 11, 13)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let datetime = DateTime::from_utc(naive_datetime, Utc);
        assert_eq!(get_puzzle_number(datetime), 191);
    }

    #[test]
    fn test_get_puzzle_date() {
        assert_eq!(get_puzzle_date(1), String::from("Saturday May 7"));
        assert_eq!(get_puzzle_date(191), String::from("Sunday November 13"));
    }
}
