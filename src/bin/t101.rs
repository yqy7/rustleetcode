// 101. 对称二叉树
// 给定一个二叉树，检查它是否是镜像对称的。
//
//  
//
// 例如，二叉树 [1,2,2,3,4,4,3] 是对称的。
//
//     1
//    / \
//   2   2
//  / \ / \
// 3  4 4  3
//  
//
// 但是下面这个 [1,2,2,null,3,null,3] 则不是镜像对称的:
//
//     1
//    / \
//   2   2
//    \   \
//    3    3
//  
//
// 进阶：
//
// 你可以运用递归和迭代两种方法解决这个问题吗？
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/symmetric-tree
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

use rustleetcode::options_vec;
use rustleetcode::TreeNode;
#[macro_export]
macro_rules! foo {
    (@as_expr $e:expr) => {$e};

    ($($tts:tt)*) => {
        foo!(@as_expr $($tts)*)
    };
}

fn main() {
    assert!(Solution::is_symmetric(TreeNode::from_vec(options_vec![1,2,2,3,4,4,3])));
    assert!(!Solution::is_symmetric(TreeNode::from_vec(options_vec![1,2,2,None,3,None,3])));
    assert!(!Solution::is_symmetric(TreeNode::from_vec(options_vec![9,(-42),(-42),None,76,76,None,None,13,None,13])));
}

struct Solution;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

// 递归
// impl Solution {
//     fn helper(left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> bool {
//         match (left, right) {
//             (None, None) => true,
//             (Some(left), Some(right)) => {
//                 left.borrow().val == right.borrow().val
//                     && Solution::helper(left.borrow().left.clone(), right.borrow().right.clone())
//                     && Solution::helper(left.borrow().right.clone(), right.borrow().left.clone())
//             },
//             _ => false,
//         }
//     }
//
//     pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
//         match root {
//             None => true,
//             Some(node) => Solution::helper(node.borrow().left.clone(), node.borrow().right.clone())
//         }
//     }
// }

// 迭代
// impl Solution {
//     pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
//         match root {
//             None => true,
//             Some(node) => {
//                 let mut deque = VecDeque::new();
//                 deque.push_back(node.clone()); // 这种方式多比较了一倍
//                 deque.push_back(node.clone());
//
//                 while !deque.is_empty() {
//                     let left = deque.pop_front().unwrap();
//                     let right = deque.pop_front().unwrap();
//                     let left = left.borrow();
//                     let right = right.borrow();
//
//                     if left.val != right.val{
//                         return false;
//                     }
//
//                     match (left.left.clone(), right.right.clone()) {
//                         (None, None) => {}
//                         (Some(left), Some(right)) => {
//                             deque.push_back(left.clone());
//                             deque.push_back(right.clone());
//                         }
//                         _ => return false,
//                     }
//
//                     match (left.right.clone(), right.left.clone()) {
//                         (None, None) => {}
//                         (Some(left), Some(right)) => {
//                             deque.push_back(left.clone());
//                             deque.push_back(right.clone());
//                         }
//                         _ => return false,
//                     }
//                 }
//                 true
//             }
//         }
//     }
// }

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let node = match root { // 用这种方式避免嵌套过深
            None => return true,
            Some(node) => node
        };

        let (left, right) = match (node.borrow().left.clone(), node.borrow().right.clone()) {
            (None, None) => return true,
            (Some(left), Some(right)) => (left, right),
            _ => return false
        };

        let mut deque = VecDeque::new();
        deque.push_back(left);
        deque.push_back(right);

        while !deque.is_empty() {
            let left = deque.pop_front().unwrap();
            let right = deque.pop_front().unwrap();
            let left = left.borrow();
            let right = right.borrow();

            if left.val != right.val{
                return false;
            }

            match (left.left.clone(), right.right.clone()) {
                (None, None) => {}
                (Some(left), Some(right)) => {
                    deque.push_back(left.clone());
                    deque.push_back(right.clone());
                }
                _ => return false,
            }

            match (left.right.clone(), right.left.clone()) {
                (None, None) => {}
                (Some(left), Some(right)) => {
                    deque.push_back(left.clone());
                    deque.push_back(right.clone());
                }
                _ => return false,
            }
        }
        true
    }
}