pub struct PatternMatcher {}

impl PatternMatcher {
    fn pattern_is_valid(pattern: &str) -> bool {
        !pattern.is_empty() && pattern.len() <= 25
    }

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

    pub fn first_char_that_does_not_match(from: &str, to: &str) -> Option<usize> {
        for (i, c) in from.chars().enumerate() {
            if c != to.chars().nth(i).expect("No char at index") {
                return Some(i);
            }
        }
        None
    }

    pub fn change_char_at_index(from: &str, idx: usize, iterations: &mut i32) -> String {
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

    /// Return
    pub fn make_subpattern_match(from: &str, to: &str, iterations: &mut i32) -> String {
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

    pub fn iterations_for_patterns(from: &str, to: &str) -> i32 {
        let mut actual = from.to_owned();
        let mut iterations = 0;

        while actual != to {
            actual = Self::make_subpattern_match(&actual, to, &mut iterations);
        }
        iterations
    }
}
