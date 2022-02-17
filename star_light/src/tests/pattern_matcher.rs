#[cfg(test)]
mod tests {
    use crate::pattern_matcher::PatternMatcher;

    mod pattern_is_valid {
        use super::*;

        #[test]
        fn with_valid_pattern() {
            assert!(PatternMatcher::pattern_is_valid("10101011101"));
            assert!(PatternMatcher::pattern_is_valid("000000"));
            assert!(PatternMatcher::pattern_is_valid("111111111111111"));
            assert!(PatternMatcher::pattern_is_valid(
                "1010101010101010101010101"
            ));
        }

        #[test]
        fn with_invalid_characters() {
            assert!(!PatternMatcher::pattern_is_valid("101010f11101"));
            assert!(!PatternMatcher::pattern_is_valid("000é000"));
            assert!(!PatternMatcher::pattern_is_valid("111111111=111111"));
        }

        #[test]
        fn width_invalid_size() {
            assert!(!PatternMatcher::pattern_is_valid(""));
            assert!(!PatternMatcher::pattern_is_valid(
                "10101010101010101010101010"
            ));
        }
    }

    mod validate {
        use super::*;

        #[test]
        fn with_valid_patterns() {
            assert!(PatternMatcher::validate("10111", "11111").is_ok());
            assert!(PatternMatcher::validate("00010", "00101").is_ok());
            assert!(PatternMatcher::validate("100", "110").is_ok());
        }

        #[test]
        fn with_patterns_of_different_size() {
            assert!(PatternMatcher::validate("10000", "00").is_err());
            assert!(PatternMatcher::validate("00010", "001010111").is_err());
            assert!(PatternMatcher::validate("10011111111", "01010110").is_err());
        }

        #[test]
        fn with_invalid_patterns() {
            assert!(PatternMatcher::validate("101a11", "111011").is_err());
            assert!(PatternMatcher::validate("", "").is_err());
            assert!(PatternMatcher::validate(
                "11111111111111111111111111",
                "11111111111111111111111111"
            )
            .is_err());
        }
    }

    mod pattern_to_change_char_at_index {
        use super::*;

        #[test]
        fn no_change_pattern() {
            assert_eq!(
                PatternMatcher::pattern_to_change_char_at_index("110000", 0),
                "110000"
            );
        }

        #[test]
        fn with_change_pattern() {
            assert_eq!(
                PatternMatcher::pattern_to_change_char_at_index("110000", 3),
                "110010"
            );
            assert_eq!(
                PatternMatcher::pattern_to_change_char_at_index("110000", 5),
                "110000"
            );
            assert_eq!(
                PatternMatcher::pattern_to_change_char_at_index("111111", 1),
                "111000"
            );
            assert_eq!(
                PatternMatcher::pattern_to_change_char_at_index("101010", 2),
                "101100"
            );
        }
    }

    mod iterations_for_patterns {
        use super::*;

        #[test]
        fn no_change_needed() {
            assert_eq!(
                PatternMatcher::iterations_for_patterns("0000000", "0000000"),
                0
            );
            assert_eq!(
                PatternMatcher::iterations_for_patterns("0000110", "0000110"),
                0
            );
            assert_eq!(
                PatternMatcher::iterations_for_patterns("1100101", "1100101"),
                0
            );
        }

        #[test]
        fn small_patterns() {
            assert_eq!(PatternMatcher::iterations_for_patterns("0101", "0100"), 1);
            assert_eq!(PatternMatcher::iterations_for_patterns("1101", "0100"), 2);
            assert_eq!(
                PatternMatcher::iterations_for_patterns("101010", "010101"),
                26
            );
        }

        #[test]
        fn medium_patterns() {
            assert_eq!(
                PatternMatcher::iterations_for_patterns("11001001000", "10000110011"),
                877
            );
        }
    }
}
