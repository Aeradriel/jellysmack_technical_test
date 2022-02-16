#[cfg(test)]
mod tests {
    use crate::map::Map;

    fn simple_map() -> Map {
        Map::from_size_and_content(
            5,
            5,
            vec![
                "A  B  C".to_owned(),
                "|--|  |".to_owned(),
                "|  |  |".to_owned(),
                "|  |--|".to_owned(),
                "1  2  3".to_owned(),
            ],
        )
        .expect("Could not create map")
    }

    mod from_size_and_content {
        use super::*;

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
            let map = simple_map();

            assert_eq!(map.height, 3);
            assert_eq!(
                map.content,
                vec![
                    "|--||".chars().collect::<Vec<_>>(),
                    "|||".chars().collect::<Vec<_>>(),
                    "||--|".chars().collect::<Vec<_>>()
                ]
            );
        }
    }

    mod validate {
        use super::*;

        #[test]
        fn invalid_height_is_not_validated() {
            let map = Map {
                width: 3,
                height: 1,
                entries: vec!["A".to_owned(), "B".to_owned(), "C".to_owned()],
                exits: vec!["1".to_owned(), "2".to_owned(), "3".to_owned()],
                content: vec![
                    "".chars().collect::<Vec<_>>(),
                    "".chars().collect::<Vec<_>>(),
                ],
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
                content: vec!["".chars().collect::<Vec<_>>()],
            };

            assert!(map.validate().is_err());
        }
    }

    mod content_coords_for_coords {
        use super::*;

        #[test]
        fn content_coords_for_coords_returns_correct_coords() {
            let map = simple_map();

            assert_eq!(map.content_coords_for_coords(0, 0), Ok((0, 0)));
            assert_eq!(map.content_coords_for_coords(2, 0), Ok((4, 0)));
            assert_eq!(map.content_coords_for_coords(2, 1), Ok((2, 1)));
            assert_eq!(map.content_coords_for_coords(1, 1), Ok((1, 1)));
            assert_eq!(map.content_coords_for_coords(2, 2), Ok((4, 2)));
        }
    }

    mod can_go_to_sides_for_coords {
        use super::*;

        #[test]
        fn works_with_simple_cases() {
            let map = simple_map();

            assert_eq!(map.can_go_to_sides_for_coords(1, 0), Ok((true, false)));
            assert_eq!(map.can_go_to_sides_for_coords(1, 1), Ok((false, false)));
            assert_eq!(map.can_go_to_sides_for_coords(1, 2), Ok((false, true)));
        }

        #[test]
        fn works_at_line_edges() {
            let map = simple_map();

            assert_eq!(map.can_go_to_sides_for_coords(0, 0), Ok((false, true)));
            assert_eq!(map.can_go_to_sides_for_coords(2, 0), Ok((false, false)));
            assert_eq!(map.can_go_to_sides_for_coords(0, 1), Ok((false, false)));
            assert_eq!(map.can_go_to_sides_for_coords(2, 1), Ok((false, false)));
            assert_eq!(map.can_go_to_sides_for_coords(0, 2), Ok((false, false)));
            assert_eq!(map.can_go_to_sides_for_coords(2, 2), Ok((true, false)));
        }
    }
}
