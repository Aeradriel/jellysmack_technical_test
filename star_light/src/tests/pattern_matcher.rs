#[cfg(test)]
mod tests {
    use crate::pattern_matcher::PatternMatcher;

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
}
