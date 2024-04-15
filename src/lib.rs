pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    // use super::*;
    use ahash::RandomState;
    use dashmap::DashMap;
    use std::collections::BTreeMap;

    #[test]
    fn dashmap_insert_usage() {
        let state = DashMap::with_hasher(RandomState::new());
        let mut map = BTreeMap::new();
        map.insert("k0", 1.5);
        map.insert("k1", 2.5);
        state.insert("src.symbol", map);
        println!("{:?}", serde_json::to_string(&state));
        match state.get_mut("src.symbol") {
            Some(mut map) => {
                map.insert("k2", 3.5);
            }
            None => {
                assert!("key not found" == "");
            }
        }
        println!("after insert: {:?}", serde_json::to_string(&state));
        let m = state.get("src.symbol");
        match m {
            Some(map) => {
                assert_eq!(map.get("k0"), Some(&1.5));
                assert_eq!(map.get("k1"), Some(&2.5));
                assert_eq!(map.get("k2"), Some(&3.5));
            }
            None => {
                assert!("key not found" == "");
            }
        }
        // assert!(false);
    }

    #[test]
    fn dashmap_extend_usage() {
        let state = DashMap::with_hasher(RandomState::new());
        let mut map = BTreeMap::new();
        map.insert("k0", 1.5);
        map.insert("k1", 2.5);
        state.insert("src.symbol", map);
        println!("{:?}", serde_json::to_string(&state));
        let mut new_map = BTreeMap::new();
        new_map.insert("k2", 3.5);
        new_map.insert("k3", 4.5);
        let updated_map = match state.get_mut("src.symbol") {
            Some(mut map) => {
                // use another map to extend
                map.extend(new_map);
                map.clone()
                // map.extend(vec![("k2", 3.5), ("k3", 4.5)]);
            }
            None => {
                assert!("key not found" == "");
                new_map
            }
        };
        println!("updated map: {:?}", updated_map);
        println!("after extend: {:?}", serde_json::to_string(&state));
        let m = state.get("src.symbol");
        match m {
            Some(map) => {
                assert_eq!(map.get("k0"), Some(&1.5));
                assert_eq!(map.get("k1"), Some(&2.5));
                assert_eq!(map.get("k2"), Some(&3.5));
                assert_eq!(map.get("k3"), Some(&4.5));
            }
            None => {
                assert!("key not found" == "");
            }
        }
        // assert!(false);
    }
}
