fn main() {
    println!("Hello, world!");
}

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

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (Some(mut node1), Some(mut node2)) => {
            if node1.val < node2.val {
                let next = node1.next.take(); // Take ownership of `node1.next`
                node1.next = merge_two_lists(next, Some(node2));
                Some(node1)
            } else {
                let next = node2.next.take(); // Take ownership of `node2.next`
                node2.next = merge_two_lists(Some(node1), next);
                Some(node2)
            }
        }
        (Some(node1), None) => Some(node1),
        (None, Some(node2)) => Some(node2),
        (None, None) => None,
    }
}
