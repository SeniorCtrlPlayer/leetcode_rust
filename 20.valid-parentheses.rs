/*
 * @lc app=leetcode id=20 lang=rust
 *
 * [20] Valid Parentheses
 */

// @lc code=start
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = String::new();
        for c in s.chars() {
            if (c == ')') {
                // if (stack.chars().nth(stack.len()-1).unwrap() != '(') {
                //     return false;
                // } else {
                //     stack.pop();
                // }
                if stack.pop() != Some('(') {
                    return false;
                }
            } else if (c == '}') {
                // if (stack.chars().nth(stack.len()-1).unwrap() != '{') {
                    // return false;
                // } else {
                    // stack.pop();
                // }
                if stack.pop() != Some('{') {
                    return false;
                }
            } else if (c == ']') {
                // if (stack.chars().nth(stack.len()-1).unwrap() != '[') {
                    // return false;
                // } else {
                    // stack.pop();
                // }
                if stack.pop() != Some('[') {
                    return false;
                }
            } else {
                stack.push(c);
            }
        }

        if (stack.len() > 0) {
            // println!("here");
            return false;
        }
        return true;
    }
}
// @lc code=end

