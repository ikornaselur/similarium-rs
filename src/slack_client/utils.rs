use crate::SimilariumError;
use std::cmp::min;

const SPACE: &str = " ";
const PARTIAL_EMOJIS: usize = 8;
const P: [&str; 9] = [
    ":p0:", ":p1:", ":p2:", ":p3:", ":p4:", ":p5:", ":p6:", ":p7:", ":p8:",
];

/// Calculate the rank prefix for a given rank
///
/// Single digit values will have 8 spaces
/// 2 digit values will have 6 spaces
/// 3 digit values will have 4 spaces
/// 4 digit values will have 2 spaces
/// 5 digit values will have no space
pub fn rank_prefix(rank: i64) -> String {
    if rank > 9999 {
        return String::new();
    }
    let space_count = 5 - rank.to_string().len();
    SPACE.repeat(space_count * 2)
}

/// Generate a progress bar using custom emojis from :p0: to :p8:
///
/// :p0: is a transparent emoji with :p1: up to :p8: filling in from the left
/// side, 16 pixels out of 128 at each step
///
/// :p1: is 16/128 pixels
/// :p2: is 32/128 pixels
/// ...
/// :p7: is 112/128 pixels
/// :p8: is 128/128 pixels
///
/// The width is the number of emojis to use to represent the total amount
///
/// The first emoji is only :p0: when amount is 0, no matter the total.
/// When amount is 1, the first emoji will be at least :p1:.
///
/// The last emoji is never :p8:, unless the amount is equal to total.
pub fn get_progress_bar(
    amount: usize,
    total: usize,
    width: usize,
) -> Result<String, SimilariumError> {
    if width < 1 {
        return Err(SimilariumError::value_error("width must be at least 1"));
    }

    // Handle full and empty bars
    if amount >= total {
        return Ok(P[8].repeat(width));
    }
    if amount == 0 {
        return Ok(P[0].repeat(width));
    }

    // Calculate how many sections there are. Each "width" can have the length of PARTIAL_EMOJIS,
    // subtracting 1 to account for the final state of amount == total
    // That is, we want to do a gradual progress for all values except the last, so that no matter
    // how high the amount is, as long as it's below total the progress is not completed. The
    // progress is only completed when amount equals to total
    let section_count = (PARTIAL_EMOJIS * width) - 1;

    // Calculate how large each section is, which are all values except the last
    // one, hence subtract 1 from total
    let section_size = (total - 1) as f64 / section_count as f64;

    // Calculate how many "sections" are filled
    let filled_sections = min(
        (amount as f64 / section_size).ceil() as usize,
        section_count,
    );

    // Calculate how many filled emojis we need first
    let full_emojis = filled_sections / PARTIAL_EMOJIS;

    // Calculate how many sections are needed for the partial
    let partial_units = filled_sections % PARTIAL_EMOJIS;

    // Put together the bar
    let output =
        P[8].repeat(full_emojis) + P[partial_units] + &P[0].repeat(width - full_emojis - 1);

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rank_prefix() {
        assert_eq!(rank_prefix(1).len(), 8);
        assert_eq!(rank_prefix(12).len(), 6);
        assert_eq!(rank_prefix(123).len(), 4);
        assert_eq!(rank_prefix(1234).len(), 2);
        assert_eq!(rank_prefix(12345).len(), 0);
        assert_eq!(rank_prefix(123456).len(), 0);
    }

    #[test]
    fn test_get_progress_bar_no_progress() {
        assert_eq!(
            get_progress_bar(0, 100, 8).unwrap(),
            ":p0::p0::p0::p0::p0::p0::p0::p0:"
        );
        assert_eq!(get_progress_bar(0, 100, 4).unwrap(), ":p0::p0::p0::p0:");
    }

    #[test]
    fn test_get_progress_bar_full_progress() {
        assert_eq!(
            get_progress_bar(100, 100, 8).unwrap(),
            ":p8::p8::p8::p8::p8::p8::p8::p8:"
        );
        assert_eq!(get_progress_bar(100, 100, 4).unwrap(), ":p8::p8::p8::p8:");
    }

    #[test]
    fn test_get_progress_bar_base_cases() {
        assert_eq!(get_progress_bar(0, 8, 1).unwrap(), ":p0:");
        assert_eq!(get_progress_bar(1, 8, 1).unwrap(), ":p1:");
        assert_eq!(get_progress_bar(2, 8, 1).unwrap(), ":p2:");
        assert_eq!(get_progress_bar(3, 8, 1).unwrap(), ":p3:");
        assert_eq!(get_progress_bar(4, 8, 1).unwrap(), ":p4:");
        assert_eq!(get_progress_bar(5, 8, 1).unwrap(), ":p5:");
        assert_eq!(get_progress_bar(6, 8, 1).unwrap(), ":p6:");
        assert_eq!(get_progress_bar(7, 8, 1).unwrap(), ":p7:");
        assert_eq!(get_progress_bar(8, 8, 1).unwrap(), ":p8:");
    }

    #[test]
    fn test_get_progress_bar_base_cases_larger_total() {
        let total = 22;
        let checks = vec![
            (":p0:", vec![0]),
            (":p1:", vec![1, 2, 3]),
            (":p2:", vec![4, 5, 6]),
            (":p3:", vec![7, 8, 9]),
            (":p4:", vec![10, 11, 12]),
            (":p5:", vec![13, 14, 15]),
            (":p6:", vec![16, 17, 18]),
            (":p7:", vec![19, 20, 21]),
            (":p8:", vec![22]),
        ];
        for (emoji, range) in checks {
            for i in range {
                let actual = get_progress_bar(i, total, 1).unwrap();
                assert_eq!(actual, emoji);
            }
        }
    }

    #[test]
    fn test_get_progress_bar_over_multiple_emojis() {
        assert_eq!(get_progress_bar(0, 16, 2).unwrap(), ":p0::p0:");
        assert_eq!(get_progress_bar(1, 16, 2).unwrap(), ":p1::p0:");
        assert_eq!(get_progress_bar(2, 16, 2).unwrap(), ":p2::p0:");
        assert_eq!(get_progress_bar(3, 16, 2).unwrap(), ":p3::p0:");
        assert_eq!(get_progress_bar(4, 16, 2).unwrap(), ":p4::p0:");
        assert_eq!(get_progress_bar(5, 16, 2).unwrap(), ":p5::p0:");
        assert_eq!(get_progress_bar(6, 16, 2).unwrap(), ":p6::p0:");
        assert_eq!(get_progress_bar(7, 16, 2).unwrap(), ":p7::p0:");
        assert_eq!(get_progress_bar(8, 16, 2).unwrap(), ":p8::p0:");
        assert_eq!(get_progress_bar(9, 16, 2).unwrap(), ":p8::p1:");
        assert_eq!(get_progress_bar(10, 16, 2).unwrap(), ":p8::p2:");
        assert_eq!(get_progress_bar(11, 16, 2).unwrap(), ":p8::p3:");
        assert_eq!(get_progress_bar(12, 16, 2).unwrap(), ":p8::p4:");
        assert_eq!(get_progress_bar(13, 16, 2).unwrap(), ":p8::p5:");
        assert_eq!(get_progress_bar(14, 16, 2).unwrap(), ":p8::p6:");
        assert_eq!(get_progress_bar(15, 16, 2).unwrap(), ":p8::p7:");
        assert_eq!(get_progress_bar(16, 16, 2).unwrap(), ":p8::p8:");
    }

    #[test]
    fn test_get_progress_bar_longer() {
        assert_eq!(
            get_progress_bar(7, 128, 8).unwrap(),
            ":p4::p0::p0::p0::p0::p0::p0::p0:"
        );
        assert_eq!(
            get_progress_bar(23, 128, 8).unwrap(),
            ":p8::p4::p0::p0::p0::p0::p0::p0:"
        );
        assert_eq!(
            get_progress_bar(33, 128, 8).unwrap(),
            ":p8::p8::p1::p0::p0::p0::p0::p0:"
        );
        assert_eq!(
            get_progress_bar(85, 128, 8).unwrap(),
            ":p8::p8::p8::p8::p8::p3::p0::p0:"
        );
        assert_eq!(
            get_progress_bar(91, 128, 8).unwrap(),
            ":p8::p8::p8::p8::p8::p6::p0::p0:"
        );
        assert_eq!(
            get_progress_bar(127, 128, 8).unwrap(),
            ":p8::p8::p8::p8::p8::p8::p8::p7:"
        );
        assert_eq!(
            get_progress_bar(128, 128, 8).unwrap(),
            ":p8::p8::p8::p8::p8::p8::p8::p8:"
        );
    }

    #[test]
    fn test_get_progress_bar_immediately_shows_progress() {
        assert_eq!(get_progress_bar(0, 1000, 4).unwrap(), ":p0::p0::p0::p0:");
        assert_eq!(get_progress_bar(1, 1000, 4).unwrap(), ":p1::p0::p0::p0:");
    }

    #[test]
    fn test_get_progress_bar_only_shows_complete_if_full() {
        assert_eq!(get_progress_bar(999, 1000, 4).unwrap(), ":p8::p8::p8::p7:");
        assert_eq!(get_progress_bar(1000, 1000, 4).unwrap(), ":p8::p8::p8::p8:");
    }

    #[test]
    fn test_get_progress_bar_issue1() {
        assert_eq!(
            get_progress_bar(0, 1000, 6).unwrap(),
            ":p0::p0::p0::p0::p0::p0:"
        );
        assert_eq!(
            get_progress_bar(998, 1000, 6).unwrap(),
            ":p8::p8::p8::p8::p8::p7:"
        );
        assert_eq!(
            get_progress_bar(1000, 1000, 6).unwrap(),
            ":p8::p8::p8::p8::p8::p8:"
        );
    }

    #[test]
    fn test_get_progress_bar_width_is_always_correct() {
        let total = 100;
        for units in 0..=total + 1 {
            for width in 1..=10 {
                // Each emoji is 4 characters
                let expected_width = width * 4;
                assert_eq!(
                    get_progress_bar(units, total, width).unwrap().len(),
                    expected_width
                );
            }
        }
    }
}
