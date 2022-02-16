#[derive(Debug)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub entries: Vec<String>,
    pub exits: Vec<String>,
    pub content: Vec<Vec<char>>,
}

impl Map {
    /// Creates a `Map` struct from a size (width/height) and a content.
    pub fn from_size_and_content(
        width: usize,
        height: usize,
        content: Vec<String>,
    ) -> Result<Map, String> {
        if content.len() >= height {
            let entries = content[0]
                .split(' ')
                .map(|s| s.to_owned())
                .collect::<Vec<_>>();
            let exits = content[height - 1]
                .split(' ')
                .map(|s| s.to_owned())
                .collect::<Vec<_>>();

            Ok(Map {
                width,
                height: height - 2,
                entries,
                exits,
                content: content[1..=(height - 2)]
                    .iter()
                    .map(|l| l.split(' ').collect::<String>().chars().collect())
                    .collect::<Vec<_>>(),
            })
        } else {
            Err("Content with not enough lines".to_owned())
        }
    }

    /// Validates the correctness of the map.
    pub fn validate(&self) -> Result<(), String> {
        let has_same_entries_exits_count = self.entries.len() == self.exits.len();
        let has_correct_height = self.content.len() == self.height;

        if !has_same_entries_exits_count {
            Err("Map does not have the same number of entries and exits".to_owned())
        } else if !has_correct_height {
            Err("Map does not have the correct height".to_owned())
        } else {
            Ok(())
        }
    }

    pub fn content_coords_for_coords(&self, x: usize, y: usize) -> Result<(usize, usize), String> {
        if let Some(line) = self.content.get(y) {
            let mut col_count = 0;

            for (i, c) in line.iter().enumerate() {
                if c == &'|' {
                    if col_count == x {
                        return Ok((i, y));
                    }
                    col_count += 1;
                }
            }
            Err("Line has not enough columns for the required coordinates".to_owned())
        } else {
            Err("Map has not enough lines for the required coordinates".to_owned())
        }
    }

    /// Tells you if you can go to left for the provided coordinates. Coordinates MUST be CONTENT COORDINATES (cf. content_coords_for_coords).
    /// Using content coordinates allows not to call `Map.content_coords_for_coords` several times.
    fn can_go_left_for_content_coords(&self, x: usize, y: usize) -> Result<bool, String> {
        if let Some(line) = self.content.get(y) {
            // If x is 0 then there is no character on the left
            // If left char is a '-', then you can go left
            if x > 0 && line[x - 1] == '-' {
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Err("Map has not enough lines for the required coordinates".to_owned())
        }
    }

    /// Tells you if you can go to right for the provided coordinates. Coordinates MUST be CONTENT COORDINATES (cf. content_coords_for_coords).
    /// Using content coordinates allows not to call `Map.content_coords_for_coords` several times.
    fn can_go_right_for_content_coords(&self, x: usize, y: usize) -> Result<bool, String> {
        if let Some(line) = self.content.get(y) {
            // If x is equal to line lenght - 1 then there is no character on the right
            // If right char is a '-', then you can go left
            if x < line.len() - 1 && line[x + 1] == '-' {
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Err("Map has not enough lines for the required coordinates".to_owned())
        }
    }

    /// Tells you if you can go to left for the provided coordinates. Coordinates MUST be normal coords (col/line).
    // TODO: Remove allow dead code
    #[allow(dead_code)]
    pub fn can_go_to_sides_for_coords(&self, x: usize, y: usize) -> Result<(bool, bool), String> {
        let content_coords = self.content_coords_for_coords(x, y)?;

        Ok((
            self.can_go_left_for_content_coords(content_coords.0, content_coords.1)?,
            self.can_go_right_for_content_coords(content_coords.0, content_coords.1)?,
        ))
    }
}
