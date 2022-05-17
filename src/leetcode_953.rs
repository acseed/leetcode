use std::cmp::Ordering;
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut dict_index_map = HashMap::with_capacity(order.len());
        for ch in order.chars().enumerate() {
            dict_index_map.insert(ch.1, ch.0);
        }
        let (head, tail) = words.split_at(1);
        let mut pre = &head[0];
        for word in tail {
            match Self::cmp(pre, word, &dict_index_map) {
                Ordering::Greater => return false,
                _ => {
                    pre = word;
                    continue;
                }
            }
        }
        true
    }

    fn cmp(a: &String, b: &String, dict: &HashMap<char, usize>) -> Ordering {
        for (a_char, b_char) in a.chars().zip(b.chars()) {
            match dict.get(&a_char).unwrap().cmp(dict.get(&b_char).unwrap()) {
                Ordering::Equal => continue,
                ret => {
                    println!("a_index:{:?}, b_index:{:?}", dict.get(&a_char), dict.get(&b_char));
                    return ret
                },
            }
        }
        return a.len().cmp(&b.len());
    }

}

#[test]
fn test() {
    let words = vec!["".to_string(), "apple".to_string()];
    let order = "abcdefghijklmnopqrstuvwxyz".to_string();
    let result = Solution::is_alien_sorted(words, order);
    assert_eq!(result, true);
}
