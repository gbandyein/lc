/*
 * @lc app=leetcode id=2 lang=java
 *
 * [2] Add Two Numbers
 */
 public class ListNode {
    int val;
    ListNode next;
    ListNode() {}
    ListNode(int val) { this.val = val; }
    ListNode(int val, ListNode next) { this.val = val; this.next = next; }
 }

// @lc code=start
/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode() {}
 *     ListNode(int val) { this.val = val; }
 *     ListNode(int val, ListNode next) { this.val = val; this.next = next; }
 * }
 */
class Solution {
    public ListNode addTwoNumbers(ListNode l1, ListNode l2) {
        ListNode res = new ListNode();
        ListNode ptr = res;
        int sum=0,carry=0;
        while(l1!=null || l2!=null || carry>0){
            sum = carry;
            if(l1!=null){
                sum+=l1.val;
                l1 = l1.next;
            }
            if(l2!=null){
                sum+=l2.val;
                l2 = l2.next;
            }
            carry = sum / 10;
            ptr.next = new ListNode(sum%10);
            ptr=ptr.next;
        }
        return res.next;
    }
}
// @lc code=end

