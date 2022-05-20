use std::collections::HashMap;
use std::cmp;


struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        let mut index_map = HashMap::<char, usize>::with_capacity(s.len());
        let mut start_index = 0;
        let mut result = 1;
        for (index, ch) in s.chars().enumerate() {
            if index_map.contains_key(&ch) {
                let old_index = index_map.get(&ch).unwrap().to_owned();
                start_index = old_index + 1;
                let keys_to_remove: Vec<char> = index_map.iter()
                    .filter(|&(&k, &v)| v < old_index)
                    .map(|o| *o.0).collect();
                for x in keys_to_remove {
                    index_map.remove(&x);
                }
                index_map.insert(ch, index);
            }
            result = cmp::max(result, index - start_index + 1);
            index_map.insert(ch, index);
        }
        result as i32
    }

    pub fn length_of_longest_substring_2(s: String) -> i32 {
        let mut index_map = HashMap::<char, usize>::with_capacity(s.len());
        let mut start_index = 0;
        let mut result = 0;
        for (index, ch) in s.chars().enumerate() {
            if index_map.contains_key(&ch) {
                let old_index = index_map.get(&ch).unwrap().to_owned();
                start_index = cmp::max(old_index + 1, start_index);
            }
            result = cmp::max(result, index - start_index + 1);
            index_map.insert(ch, index);
        }
        result as i32
    }
}

#[test]
fn test() {
    let s = "abba".to_string();
    let result = Solution::length_of_longest_substring(s);
    println!("{}", result);
}