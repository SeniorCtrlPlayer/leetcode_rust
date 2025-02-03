/*
 * @lc app=leetcode id=26 lang=rust
 *
 * [26] Remove Duplicates from Sorted Array
 */

// @lc code=start
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut max_ = nums[0];
        let mut n = 0;
        match nums.is_empty() {
            true => 0,
            false => {
                for i in 1..nums.len() {
                    if nums[i] > nums[n] {
                        n += 1;
                        nums[n] = nums[i];
                    }
                }
                (n+1) as i32
            }
        }
    }
}
// @lc code=end

