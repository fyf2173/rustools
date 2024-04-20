use std::{collections::HashMap, hash::Hash, vec};

pub fn vec_field_to_vec<T, R>(refs: &Vec<T>, test_fn: fn(val: &T) -> Option<R>) -> Vec<R> {
    let mut data: Vec<R> = vec![];
    for r in refs.iter() {
        if let Some(item) = test_fn(&r) {
            data.push(item)
        }
    }
    data
}

pub fn vec_field_hashmap<T, K: Eq + Hash, R>(
    refs: &Vec<T>,
    test_fn: fn(val: &T) -> Option<(K, R)>,
) -> HashMap<K, R> {
    let mut data = HashMap::new();
    for r in refs.iter() {
        if let Some((key, val)) = test_fn(&r) {
            data.insert(key, val);
        }
    }
    data
}

pub fn hashmap_field_to_vec<K, V, R>(
    refs: &HashMap<K, V>,
    test_fn: fn(key: &K, val: &V) -> Option<R>,
) -> Vec<R> {
    let mut data = vec![];
    for (k, v) in refs {
        if let Some(val) = test_fn(k, v) {
            data.push(val)
        }
    }
    data
}

pub fn vec_find_ok<T: Eq, V: Eq>(
    refs: &Vec<T>,
    target: V,
    test_fn: fn(val: &T, target: &V) -> bool,
) -> bool {
    for r in refs.iter() {
        if test_fn(r, &target) == true {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn it_works_0() {
        let mut hash1 = HashMap::new();
        hash1.insert("k1", 1);
        hash1.insert("k2", 2);
        hash1.insert("k3", 3);

        dbg!(hashmap_field_to_vec(&hash1, |_k, v| {
            if *v == 2 {
                return Some(*v);
            }
            None
        }));
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct UserInfo {
        id: i64,
        name: String,
    }

    #[test]
    fn it_works_1() {
        let mut hash1 = HashMap::new();
        hash1.insert(
            "k1",
            UserInfo {
                id: 1,
                name: "n1".to_string(),
            },
        );
        hash1.insert(
            "k2",
            UserInfo {
                id: 2,
                name: "n2".to_string(),
            },
        );
        hash1.insert(
            "k3",
            UserInfo {
                id: 3,
                name: "n3".to_string(),
            },
        );

        dbg!(hashmap_field_to_vec(&hash1, |_k, v| {
            if v.clone().id == 2 {
                return Some(v.clone().name);
            }
            None
        }));
    }

    #[test]
    fn it_works_2() {
        let data = vec![
            UserInfo {
                id: 1,
                name: "n1".to_string(),
            },
            UserInfo {
                id: 2,
                name: "n2".to_string(),
            },
            UserInfo {
                id: 3,
                name: "n3".to_string(),
            },
        ];

        dbg!(vec_find_ok(&data, "n4".to_string(), |v1, v2| {
            if v1.name == *v2 {
                return true;
            }
            return false;
        }));
    }

    #[test]
    fn it_works_3() {
        let data = vec![
            "test1".to_string(),
            "test2".to_string(),
            "test2".to_string(),
        ];
        let test_char = "test3";
        dbg!(vec_find_ok(&data, test_char, |v1, v2| {
            if v1 == v2 {
                return true;
            }
            return false;
        }));
    }
}
