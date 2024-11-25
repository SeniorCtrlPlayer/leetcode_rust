/*
 * @lc app=leetcode id=14 lang=rust
 *
 * [14] Longest Common Prefix
 */

// @lc code=start
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut output = String::new();
        // let len = strs[0].len();
        let mut len = strs[0].len();
        for s in strs.iter() {
            if len > s.len() {
                len = s.len();
            }
        }
        // println!("the min len of string is {len}");
        for i in 0..len {
            // let b = strs[0].chars().nth(i).unwrap();
            for s in strs.iter() {
                if (s.chars().nth(i).unwrap() != strs[0].chars().nth(i).unwrap()) {
                    return output;
                }
            }
            // output.push(b);
            output.push(strs[0].chars().nth(i).unwrap());
        }
        return output;
    }
}
// @lc code=end

