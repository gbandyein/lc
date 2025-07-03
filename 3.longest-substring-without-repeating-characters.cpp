/*
 * @lc app=leetcode id=3 lang=cpp
 *
 * [3] Longest Substring Without Repeating Characters
 */
#include <array>
#include <string>
#include <map>
#include <set>
#include <unordered_map>
#include <unordered_set>
#include <vector>
using std::string;
using std::map;
using std::set;
using std::unordered_map;
using std::unordered_set;
using std::vector;
// @lc code=start
class Solution {
public:
    int lengthOfLongestSubstring(string s) {
        int i=0,j=0,mLen=0;
        // unordered_map<char,int> map;
        std::array<int,sizeof(char)<<8> map;
        map.fill(-1);
        for(i=0;i<s.length();i++){
            char ch = s.at(i);
            // if(map.count(ch) && map[ch]>=j) j=map[ch]+1;
            if(map[ch]>=j) j=map[ch]+1;
            mLen = std::max(mLen, i-j+1);
            map[ch] = i;
        }
        return mLen;
    }
};
// @lc code=end

