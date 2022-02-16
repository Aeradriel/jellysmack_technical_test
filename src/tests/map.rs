#[cfg(test)]
mod tests {
    use crate::map::Map;

    #[test]
    fn valid_params_generate_the_map() {
        let map = Map::from_size_and_content(
            4,
            4,
            vec![
                "A B C".to_owned(),
                "".to_owned(),
                "".to_owned(),
                "1 2 3".to_owned(),
            ],
        );

        assert!(map.is_ok());
    }

    #[test]
    fn invalid_height_does_not_create_a_map() {
        let map = Map::from_size_and_content(
            4,
            4,
            vec!["A B C".to_owned(), "".to_owned(), "1 2 3".to_owned()],
        );

        assert!(map.is_err());
    }

    #[test]
    fn map_from_content_remove_entries_and_exits_from_content_and_size() {
        let map = Map::from_size_and_content(
            4,
            4,
            vec![
                "A  B  C".to_owned(),
                "|--|  |".to_owned(),
                "|  |--|".to_owned(),
                "1  2  3".to_owned(),
            ],
        )
        .expect("Could not create map");

        assert_eq!(map.height, 2);
        assert_eq!(
            map.content,
            vec!["|--|  |".to_owned(), "|  |--|".to_owned()]
        );
    }

    #[test]
    fn invalid_height_is_not_validated() {
        let map = Map {
            width: 3,
            height: 1,
            entries: vec!["A".to_owned(), "B".to_owned(), "C".to_owned()],
            exits: vec!["1".to_owned(), "2".to_owned(), "3".to_owned()],
            content: vec!["".to_owned(), "".to_owned()],
        };

        assert!(map.validate().is_err());
    }

    #[test]
    fn different_cout_of_entires_exits_is_not_validated() {
        let map = Map {
            width: 3,
            height: 1,
            entries: vec!["A".to_owned(), "B".to_owned(), "C".to_owned()],
            exits: vec!["1".to_owned(), "2".to_owned()],
            content: vec!["".to_owned()],
        };

        assert!(map.validate().is_err());
    }
}
