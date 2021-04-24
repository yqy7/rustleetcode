use std::cell::RefCell;
use std::rc::Rc;
use std::option::Option::Some;
use std::collections::VecDeque;
use std::ops::Deref;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
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
            val,
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

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
    pub fn newTreeNode(val: i32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode::new(val)))
    }
    pub fn from_vec(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>>{
        if vec.len() == 0 { return None; }

        let mut root = TreeNode::newTreeNode(vec[0].unwrap());

        let mut deque = VecDeque::new();
        deque.push_back(root.clone());

        let mut i = 1;
        while i < vec.len() {
            let node = deque.pop_front().unwrap();
            if i < vec.len() {
                (*node.borrow_mut()).left = vec[i].map(|v| {
                    let left = TreeNode::newTreeNode(v);
                    deque.push_back(left.clone());
                    left
                });
                i += 1;
            }
            if i < vec.len() {
                (*node.borrow_mut()).right = vec[i].map(|v| {
                    let right = TreeNode::newTreeNode(v);
                    deque.push_back(right.clone());
                    right
                });
                i += 1;
            }
        }

        Some(root)
    }
}

#[macro_export]
macro_rules! options_vec {
    (None) => { None };
    ($e:expr) => { Some($e)};

    ($($e:tt),*) => {
        vec![$(
            options_vec!($e), // 负数会被拆成- 和 数字，导致出现错误，需要用 (-45)
        )*]
    };
}