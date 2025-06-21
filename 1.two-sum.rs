/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */
// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut sums: HashMap<i32,i32> = HashMap::new();
        for i in 0..nums.len() {
            let n: i32 = nums[i];
            let i = i as i32;
            if sums.contains_key(&n) {
                return vec![sums[&n],i];
            }
            sums.insert((target - n), i );
        }
        vec![0,0]
    }
}
// @lc code=end

