use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut index_map = HashMap::<i32, u32>::with_capacity(nums.len());
        for (index, num) in nums.into_iter().enumerate() {
            let other = target - num;
            if index_map.contains_key(&other) {
                return vec![*index_map.get(&other).unwrap() as i32, index as i32];
            }
            index_map.insert(num, index as u32);
        }
        vec![]
    }
}

#[test]
fn test_two_sum() {
    let nums = vec![3,2,4];
    let target = 6;
    let result = Solution::two_sum(nums, target);
    assert_eq!(result, [1, 2])
}