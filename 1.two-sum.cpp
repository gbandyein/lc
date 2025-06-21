/*
 * @lc app=leetcode id=1 lang=cpp
 *
 * [1] Two Sum
 */
#include <vector>
#include <map>
#include <unordered_map>
using std::vector;
using std::map;
using std::unordered_map;
// @lc code=start
class Solution {
public:
    vector<int> twoSum(vector<int>& nums, int target) {
        unordered_map<int,int> sums;
        for(int i=0;i<nums.size();i++){
            int n = nums[i];
            if(sums.contains(n)){
                return {sums[n],i};
            }
            sums[target-n]=i;
        }
        return {0,0};
    }
};
// @lc code=end

