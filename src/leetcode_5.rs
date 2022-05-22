use std::cmp;
use std::ops::Range;
use std::iter::FromIterator;

struct Solution;
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let sv = s.chars().collect::<Vec<char>>();
        Self::do_find(&sv)
    }

    fn do_find(s: &[char]) -> String {
        if s.is_empty() {
            return "".to_string();
        }

        let len = s.len() as i32;
        let mut range: Range<i32> = 0..1;
        let mut i = 0;
        while i < len {
            // 枚举长度
            // 奇数
            let mut j = 0;
            while i - j - 1 >= 0 && i + j + 1 < len && s[(i - j - 1) as usize] == s[(i + j + 1) as usize] {
                j += 1;
            }
            if 2 * j + 1 > range.len() as i32 {
                range = (i - j)..(i + j + 1);
            }

            //偶数
            if i + 1 < len && s[i as usize] == s[i as usize + 1] {
                j = 0;
                while i - j - 1 >= 0 && i + j + 2 < len && s[(i - j - 1) as usize] == s[(i + j + 2) as usize] {
                    j += 1;
                }

                if j * 2 + 2 > range.len() as i32 {
                    range = (i - j)..(i + j + 2);
                }
            }

            i += 1;
        }
        let result = s[range.start as usize..range.end as usize].iter();
        String::from_iter(result)
    }
}

#[test]
fn test() {
    let s = "cbbd".to_string();
    let result = Solution::longest_palindrome(s);
    println!("{}", result);
}