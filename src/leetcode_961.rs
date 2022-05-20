extern crate rand;

use rand::random;

struct Solution;

impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        for i in 1..nums.len() {
            if nums[i] == nums[i - 1] {
                return nums[i];
            }

            if i > 1 && nums[i] == nums[i - 2] {
                return nums[i];
            }

            if i > 2 && nums[i] == nums[i - 3] {
                return nums[i];
            }
        }
        0
    }

    pub fn repeated_n_times_2(nums: Vec<i32>) -> i32 {
        loop {
            let x: usize = random::<usize>() % nums.len();
            let y: usize = random::<usize>() % nums.len();
            if x != y && nums[x] == nums[y] {
                return nums[x];
            }
        }
    }
}

#[test]
fn test() {
    let nums = vec![2, 6, 2, 1];
    let result = Solution::repeated_n_times_2(nums);
    println!("{}", result);
}