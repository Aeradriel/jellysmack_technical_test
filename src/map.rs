#[derive(Debug)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub entries: Vec<String>,
    pub exits: Vec<String>,
    pub content: Vec<String>,
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
                content: content[1..=(height - 2)].into(),
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
}
