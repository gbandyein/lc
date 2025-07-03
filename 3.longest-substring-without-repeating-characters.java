/*
 * @lc app=leetcode id=3 lang=java
 *
 * [3] Longest Substring Without Repeating Characters
 */
// @lc code=start
import java.util.Map;
import java.util.HashMap;
class Solution {
    public int lengthOfLongestSubstring(String s) {
        // Map<Character,Integer> map = new HashMap<Character,Integer>(256);
        int[] map = new int[256];
        Arrays.fill(map,-1);
        int l = 0, r = 0, mLen = 0;
        for(r = 0; r<s.length();r++){
            char ch = s.charAt(r);
            // int mVal = map.getOrDefault(ch,-1);
            int mVal = map[ch];
            if(mVal>=l){
                l = mVal+1;
            }
            mLen = Math.max(mLen,r-l+1);
            // map.put(ch,r);
            map[ch]=r;
        }
        return mLen;
    }
}
// @lc code=end

