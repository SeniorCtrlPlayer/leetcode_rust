/*
 * @lc app=leetcode id=9 lang=rust
 *
 * [9] Palindrome Number
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let num_str: String = x.to_string();
        let len = num_str.len();
        let max_i = len / 2;
        for i in 0..max_i {
            if num_str.chars().nth(i).unwrap() != num_str.chars().nth(len-i-1).unwrap() {
                return false;
            }
        }
        // println!("{}", num_str);
        return true;
    }
}
// @lc code=end

