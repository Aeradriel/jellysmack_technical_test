use std::io;

mod map;
mod tests;

use map::Map;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

/// Reads the standards input to construct a `Map` struct.
/// Returns an `Err` if the input is incorrect or the map could not be validated.
fn get_map() -> Result<Map, String> {
    // Reads standard input (first line defines the number of lines read)
    let mut input_line = String::new();

    io::stdin()
        .read_line(&mut input_line)
        .expect("Could not get map info");

    let inputs: Vec<&str> = input_line.split(' ').collect::<Vec<_>>();
    let width: usize = parse_input!(inputs[0], usize);
    let height: usize = parse_input!(inputs[1], usize);
    let mut content: Vec<String> = vec![];

    for _ in 0..height {
        let mut input_line = String::new();

        io::stdin()
            .read_line(&mut input_line)
            .expect("Could not read line from standard input");

        content.push(input_line.trim_end().to_owned());
    }

    // Construct the map struct and validates it
    let map = Map::from_size_and_content(width, height, content)?;

    map.validate()?;
    Ok(map)
}

fn main() {
    match get_map() {
        Ok(map) => {
            for entry in &map.entries {
                println!(
                    "{}{}",
                    entry,
                    map.exit_for_entry(entry)
                        .expect("Could not find exit for entry")
                );
            }
        }
        Err(err) => {
            eprintln!("{}", err)
        }
    }
}
