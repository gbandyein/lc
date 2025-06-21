/*
 * @lc app=leetcode id=1 lang=java
 *
 * [1] Two Sum
 */

// @lc code=start
class Solution {
    public int[] twoSum(int[] nums, int target) {
        Map<Integer, Integer> sums = new HashMap<Integer, Integer>();
        int[] res = new int[2];
        for(int i=0;i<nums.length; i++){
            int n = nums[i];
            int diff = target - n;
            if(sums.containsKey(n)){
                res[0] = sums.get(n);
                res[1] = i;
                break;
            } else {
                sums.put(diff,i);
            }
        }
        return res;
    }
}
// @lc code=end

