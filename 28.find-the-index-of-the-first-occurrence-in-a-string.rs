/*
 * @lc app=leetcode id=28 lang=rust
 *
 * [28] Find the Index of the First Occurrence in a String
 */

// @lc code=start
impl Solution {
    pub fn str_str1(haystack: String, needle: String) -> i32 {
        let mut m = haystack.len();
        let mut n = needle.len();
        if m < n {
            return -1
        }
        if m == 1 && n==1 && haystack.chars().nth(m) == needle.chars().nth(n) {
            return 0;
        }
        for i in 0..(m-n+1) {
            let mut j = i;
            let mut flag = true;
            for y in needle.chars() {
                if y != haystack.chars().nth(j).unwrap() {
                    flag = false;
                    break;
                }
                j += 1;
            }
            if flag {
                return i as i32;
            }
        }
        -1
    }
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let mut m = haystack.len();
        let mut n = needle.len();
        if m < n {
            return -1;
        }
        for i in 0..(m-n+1) {
            if haystack[i..i+n] == needle {
                return i as i32;
            }
        }
        return -1 as i32;
    }
}
// @lc code=end

