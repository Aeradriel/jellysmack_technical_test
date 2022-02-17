use std::io;

mod pattern_matcher;
mod tests;

use pattern_matcher::PatternMatcher;

fn get_next_line() -> Result<String, std::io::Error> {
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line)?;
    Ok(input_line.trim_end().into())
}

fn main() {
    let from = get_next_line().expect("Could not read start pattern");
    let to = get_next_line().expect("Could not read end pattern");

    PatternMatcher::validate(&from, &to).expect("Invalid pattern(s)");
    println!("{}", PatternMatcher::iterations_for_patterns(&from, &to));
}
