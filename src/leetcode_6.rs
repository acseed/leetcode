use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let sv = s.chars().collect::<Vec<char>>();
        let mut index_map = Solution::index_calculate(&sv, num_rows);
        let mut result_vec = vec![];
        for i in 1..=num_rows {
            match index_map.get_mut(&i) {
                Some(s) => {
                    result_vec.append(s);
                },
                _ => continue
            }
        }
        result_vec.iter().collect()
    }

    fn index_calculate(s: &[char], num_rows: i32) -> HashMap<i32, Vec<char>> {
        let mut idx = 1;
        let mut zig = true;
        let mut result = HashMap::<i32, Vec<char>>::new();
        for i in 0..s.len() {
            match result.get_mut(&idx) {
                Some(v) => {
                    v.push(s[i].clone());
                }
                _ => {
                    let v = vec![s[i].clone()];
                    result.insert(idx, v);
                }
            }

            if zig && idx == num_rows {
                zig = false;
            } else if !zig && idx == 1 {
                zig = true;
            }

            idx = match zig {
                true if idx + 1 <= num_rows => idx + 1,
                _ => {
                    if idx - 1 > 0 {
                        idx - 1
                    } else {
                        idx
                    }
                },
            }
        }
        result
    }
}

#[test]
fn test() {
    let s = "AB".to_string();
    let num_rows = 1;
    let result = Solution::convert(s, num_rows);
    println!("{}", result);
}