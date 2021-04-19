use std::borrow::Borrow;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl Drop for ListNode {
    fn drop(&mut self) {
        println!("drop: {}", self.val);
    }
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }

    pub fn fromVec(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut p = &mut None;
        for i in 0..vec.len() {
            match p {
                None => {
                    head = Some(Box::new(ListNode::new(vec[i])));
                    p = &mut head;
                }
                Some(node) => {
                    node.next = Some(Box::new(ListNode::new(vec[i])));
                    p = &mut node.next;
                }
            }
        }

        head
    }
}