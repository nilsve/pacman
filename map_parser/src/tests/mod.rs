#[cfg(test)]
mod tests {
    use crate::GameMap;

    #[test]
    fn it_parses_a_map() {
        let map_str = include_str!("./map1.json");
        let map = GameMap::load_from_string(map_str).expect("Cant open json file");
        assert_eq!(map.tiles
            .get(1).expect("Row one not found")
            .get(1).expect("Column not found")
            .tile_type, 'A')
    }
}
