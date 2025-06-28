/*
 * @lc app=leetcode id=2 lang=cpp
 *
 * [2] Add Two Numbers
 */

 struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
 };
// @lc code=start
/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
class Solution {
public:
    ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
        ListNode res;
        ListNode* tmp = &res;
        
        int sum=0;
        int carry = 0;

        while(l1!=nullptr || l2!=nullptr || carry>0){
            sum = carry;

            if(l1!=nullptr) {
                sum+=l1->val;
                l1 = l1->next;
            }

            if(l2!=nullptr) {
                sum+=l2->val;
                l2 = l2->next;
            }

            carry = sum / 10;
            sum = sum % 10;

            tmp->next = new ListNode(sum);
            tmp = tmp->next;
        }

        return res.next;
    }
};
// @lc code=end

