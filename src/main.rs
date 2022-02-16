use std::io;

mod map;
mod tests;

use map::Map;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

fn get_map() -> Result<Map, String> {
    let mut input_line = String::new();

    io::stdin()
        .read_line(&mut input_line)
        .expect("Could not get map info");

    let inputs: Vec<&str> = input_line.split(" ").collect::<Vec<_>>();
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

    let map = Map::from_size_and_content(width, height, content)?;

    map.validate()?;
    Ok(map)
}

fn main() {
    match get_map() {
        Ok(map) => {
            println!("{:?}", map);
        }
        Err(err) => {
            eprintln!("{}", err)
        }
    }
}
