use std::char;

/*
 * @lc app=leetcode id=3 lang=rust
 *
 * [3] Longest Substring Without Repeating Characters
 */
// #![crate_name="lc3"]
pub struct Solution{}
fn main(){
    let res = Solution::length_of_longest_substring(String::from("abcabc"));
    println!("{}",res);
    let res = Solution::length_of_longest_substring(String::from("pwwkew"));
    println!("{}",res);
    let res = Solution::length_of_longest_substring(String::from("bbbb"));
    println!("{}",res);
}
// @lc code=start
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut mLen: i32 = 0;
        let mut j: i32 = 0;
        let mut map: [i32; 256] = [-1; 256];
        for (i, ch) in s.char_indices() {
            let ch = ch as usize;
            let i = i as i32;
            if map[ch]>=j {
                j = map[ch]+1;
            }
            mLen = std::cmp::max(mLen, i-j+1);
            map[ch]=i;
        }
        mLen
    }
}
// @lc code=end

