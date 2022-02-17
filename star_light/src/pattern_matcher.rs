pub struct PatternMatcher {}

impl PatternMatcher {
    /// Validates if a pattern is valid (only 0s and 1s and between 1 and 25 chars).
    pub fn pattern_is_valid(pattern: &str) -> bool {
        let pattern_length_valid = !pattern.is_empty() && pattern.len() <= 25;

        for c in pattern.chars() {
            if c != '1' && c != '0' {
                return false;
            }
        }
        pattern_length_valid
    }

    /// Validates if a couple from/to is valid. It checks if both have the same length and if pattern are valid.
    pub fn validate(from: &str, to: &str) -> Result<(), String> {
        let start_pattern_valid = PatternMatcher::pattern_is_valid(from);
        let end_pattern_valid = PatternMatcher::pattern_is_valid(to);

        if from.len() != to.len() {
            return Err("Start and end patterns do not have the same length".to_owned());
        } else if !start_pattern_valid {
            return Err("Start pattern is invalid".to_owned());
        } else if !end_pattern_valid {
            return Err("End pattern is invalid".to_owned());
        }
        Ok(())
    }

    /// Returns the closest possible pattern needed to be able to change the character at the given index.
    pub fn pattern_to_change_char_at_index(from: &str, idx: usize) -> String {
        let mut pattern: Vec<char> = vec![];

        if idx == from.len() - 1 {
            return from.to_owned();
        }
        for (i, c) in from.chars().enumerate() {
            if i <= idx {
                pattern.push(c);
            } else if i == idx + 1 {
                pattern.push('1');
            } else {
                pattern.push('0');
            }
        }
        pattern.into_iter().collect()
    }

    /// Returns the first character's index that does not match both in from and to.
    /// If both strings are identical, `None` is returned.
    fn first_char_that_does_not_match(from: &str, to: &str) -> Option<usize> {
        for (i, c) in from.chars().enumerate() {
            if c != to.chars().nth(i).expect("No char at index") {
                return Some(i);
            }
        }
        None
    }

    /// Toggle the char at the given index between 1 and 0.
    /// It also increase the number of iterations by one.
    /// This does not handle the check to know if you can do it or not.
    fn change_char_at_index(from: &str, idx: usize, iterations: &mut i32) -> String {
        let mut new_from = from.to_owned();
        let actual_char = from.chars().nth(idx).expect("No char at index");
        let new_char = match actual_char {
            '1' => '0',
            '0' => '1',
            _ => '0',
        };

        new_from.replace_range(idx..(idx + 1), &new_char.to_string());
        *iterations += 1;
        new_from
    }

    /// Search for the first char that does not fit the "to" pattern.
    /// It then identifies the pattern needed to modify the character.
    /// If the pattern matches the "from" string, it makes the change,
    /// else it keeps the same "from" but tries to match this new pattern.
    fn make_subpattern_match(from: &str, to: &str, iterations: &mut i32) -> String {
        if let Some(first_wrong_idx) = Self::first_char_that_does_not_match(from, to) {
            let closer_pattern = Self::pattern_to_change_char_at_index(from, first_wrong_idx);

            if closer_pattern == from {
                let new_from = Self::change_char_at_index(from, first_wrong_idx, iterations);

                Self::make_subpattern_match(&new_from, to, iterations)
            } else {
                Self::make_subpattern_match(from, &closer_pattern, iterations)
            }
        } else {
            from.to_owned()
        }
    }

    // Return the minimum number of iterations needed to reach `to` starting from `from`.
    pub fn iterations_for_patterns(from: &str, to: &str) -> i32 {
        let mut actual = from.to_owned();
        let mut iterations = 0;

        while actual != to {
            actual = Self::make_subpattern_match(&actual, to, &mut iterations);
        }
        iterations
    }
}
