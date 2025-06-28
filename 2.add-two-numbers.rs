/*
 * @lc app=leetcode id=2 lang=rust
 *
 * [2] Add Two Numbers
 */
pub struct Solution {}
fn main(){
  let mut res = & Solution::add_two_numbers(Some(Box::new(ListNode::new(9))), Some(Box::new(ListNode::new(2))));
  // print!("{}",res.unwrap().val);
  while let Some(ref node) = res {
    print!("{}",node.val);
    res = &node.next;
  }
  println!();
}
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
      let mut res: Box<ListNode> = Box::new(ListNode::new(0));
      let mut curr = &mut res;
      let mut sum = 0;
      let mut carry = 0;

      let mut l1 = &l1;
      let mut l2 = &l2;

      while l1.is_some() || l2.is_some() || carry!=0 {
        sum = carry;
        if let Some(node) = l1 {
          sum = sum + node.val;
          l1 = &node.next;
        }
        if let Some(node) = l2 {
          sum = sum + node.val;
          l2 = &node.next;
        }
        carry = sum / 10;
        sum = sum % 10;
        curr.next = Some(Box::new(ListNode::new(sum)));
        curr = curr.next.as_mut().unwrap();
      }

      res.next
    }
}
// @lc code=end

