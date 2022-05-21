use std::cmp;
use std::cmp::Ordering;
use std::ops::Range;

struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let tot = nums1.len() + nums2.len();
        return match tot & 1 {
            1 => {
                let k = tot / 2 + 1;
                Self::find_kth(&nums1, &nums2, k) as f64
            },
            _ => {
                let k = tot / 2;
                let kth = Self::find_kth(&nums1, &nums2, k);
                let k_plus_th = Self::find_kth(&nums1, &nums2, k + 1);
                (kth + k_plus_th) as f64 / 2.0f64
            }
        }
    }

    fn find_kth(nums1: &[i32], nums2: &[i32], k: usize) -> i32{
        if nums1.is_empty() {
            return nums2[k - 1];
        }

        if nums2.is_empty() {
            return nums1[k - 1];
        }

        let mid1 = (nums1.len() - 1) / 2;
        let upper_bound2 = Self::upper_bound(nums2, nums1[mid1]);
        let left = mid1 + upper_bound2 + 1;

        return match k.cmp(&left) {
            Ordering::Less => {
                Self::find_kth(&nums1[..mid1], &nums2[..upper_bound2], k)
            },
            Ordering::Equal => {
                if upper_bound2 == 0 || nums1[mid1] >= nums2[upper_bound2 - 1] {
                    nums1[mid1]
                } else {
                    nums2[upper_bound2 - 1]
                }
            },
            Ordering::Greater => {
                Self::find_kth(&nums1[mid1 + 1..], &nums2[upper_bound2..], k - left)
            }
        }
    }

    fn upper_bound(nums: &[i32], num: i32) -> usize {
        let mut low: i32 = 0;
        let mut high: i32 = (nums.len() - 1) as i32;
        while low <= high {
            let mut mid = low + (high - low) / 2;
            match nums[mid as usize].cmp(&num) {
                Ordering::Greater => {
                    high = mid - 1;
                },
                _ => {
                    low = mid + 1;
                }
            }
        }
        low as usize
    }
}

#[test]
fn test() {
    let nums1 = vec![1,1,1];
    let nums2 = vec![1,1,1];
    let result = Solution::find_median_sorted_arrays(nums1, nums2);
    println!("{:?}", result);
}