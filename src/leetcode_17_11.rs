use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn find_closest(words: Vec<String>, word1: String, word2: String) -> i32 {
        let mut index_map = HashMap::<String, Vec<i32>>::new();
        for (index, word) in words.iter().enumerate() {
            let v = match index_map.get_mut(word) {
                Some(vv) => {
                    vv.push(index as i32);
                    vv.to_owned()
                },
                _ => {
                    vec![index as i32]
                }
            };
            index_map.insert(word.clone(), v);
        }

        match (index_map.get(&word1), index_map.get(&word2)) {
            (Some(v1), Some(v2)) => {
                let mut min_dis = i32::MAX;
                for p1 in v1 {
                    for p2 in v2 {
                        min_dis = i32::min(min_dis, i32::abs(p1 - p2))
                    }
                }
               min_dis
            },
            _ => 0
        }
    }
}

#[test]
fn test() {
    let words = vec!["I".to_string(), "am".to_string(), "a".to_string(),
                     "student".to_string(), "from".to_string(), "a".to_string(),
                     "university".to_string(), "in".to_string(), "a".to_string(),
                     "city".to_string()];
    let word1 = "a".to_string();
    let word2 = "student".to_string();
    let result = Solution::find_closest(words, word1, word2);
    println!("{}", result)
}