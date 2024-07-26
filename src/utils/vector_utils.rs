use std::collections::HashMap;
use std::hash::Hash;

pub fn group_by<T, K, F>(items: &Vec<T>, keyer: F) -> HashMap<K, Vec<T>>
where
    T: Clone,
    K: Eq + Hash,
    F: Fn(&T) -> K,
{
    let mut grouped: HashMap<K, Vec<T>> = HashMap::new();
    for item in items {
        let key = keyer(&item);
        grouped.entry(key).or_insert_with(Vec::new).push(item.clone());
    }
    grouped
}

pub fn make_lookup<T, K, F>(items: &Vec<T>, keyer: F) -> HashMap<K, T>
where
    T: Clone,
    K: Eq + Hash,
    F: Fn(&T) -> K,
{
    let mut lookup: HashMap<K, T> = HashMap::new();
    for item in items {
        let key = keyer(&item);
        lookup.insert(key, item.clone());
    }
    lookup
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_by_first_char() {
        let items = vec!["apple", "banana", "apricot", "blueberry"];
        let keyer = |&x: &&str| x.chars().next().unwrap(); 
        let grouped = group_by(&items, keyer);
        
        assert_eq!(grouped.get(&'a'), Some(&vec!["apple", "apricot"]));
        assert_eq!(grouped.get(&'b'), Some(&vec!["banana", "blueberry"]));
    }

    #[test]
    fn test_group_by_length() {
        let items = vec!["a", "bb", "ccc", "dddd", "ee", "fff"];
        let keyer = |&x: &&str| x.len(); 
        let grouped = group_by(&items, keyer);
        
        assert_eq!(grouped.get(&1), Some(&vec!["a"]));
        assert_eq!(grouped.get(&2), Some(&vec!["bb", "ee"]));
        assert_eq!(grouped.get(&3), Some(&vec!["ccc", "fff"]));
        assert_eq!(grouped.get(&4), Some(&vec!["dddd"]));
    }
}