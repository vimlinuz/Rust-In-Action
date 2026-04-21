// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1.as_ref();
        let mut l2 = l2.as_ref();
        let mut carry = 0;

        let mut return_node: Option<Box<ListNode>> =
            Some(Box::new(ListNode { val: 0, next: None }));
        let mut current_node = return_node.as_mut();

        while l1.is_some() || l2.is_some() {
            let mut sum = carry;

            if let Some(node) = l1 {
                sum = sum + node.val;
                l1 = node.next.as_ref();
            }
            if let Some(node) = l2 {
                sum = sum + node.val;
                l2 = node.next.as_ref();
            }
            carry = sum / 10;
            let temp = Some(Box::new(ListNode::new(sum % 10)));
            current_node.as_mut().unwrap().next = temp;
            current_node = current_node.unwrap().next.as_mut();
        }

        if carry > 0 {
            let temp = Some(Box::new(ListNode::new(carry)));
            current_node.as_mut().unwrap().next = temp;
            current_node = current_node.unwrap().next.as_mut();
        }
        return return_node.unwrap().next;
    }
}
