// 104. 二叉树的最大深度
// 给定一个二叉树，找出其最大深度。
//
// 二叉树的深度为根节点到最远叶子节点的最长路径上的节点数。
//
// 说明: 叶子节点是指没有子节点的节点。
//
// 示例：
// 给定二叉树 [3,9,20,null,null,15,7]，
//
//     3
//    / \
//   9  20
//     /  \
//    15   7
// 返回它的最大深度 3 。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/maximum-depth-of-binary-tree
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
use rustleetcode::TreeNode;
use rustleetcode::options_vec;

fn main() {
    assert_eq!(Solution::max_depth(TreeNode::from_vec(options_vec![3,9,20,None,None,15,7])), 3);
}

struct Solution;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

// impl Solution {
//     // 迭代法
//     pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
//         let root = match root {
//             None => return 0,
//             Some(root) => root
//         };
//         let mut deque = VecDeque::new();
//         deque.push_back(root);
//         let mut count = 0;
//         while !deque.is_empty() {
//             let size = deque.len();
//             for _ in 0..size {
//                 let node = deque.pop_front().unwrap();
//                 let nodeBorrow = node.borrow();
//                 if let Some(left) = nodeBorrow.left.clone() {
//                     deque.push_back(left);
//                 }
//                 if let Some(right) = nodeBorrow.right.clone() {
//                     deque.push_back(right);
//                 }
//             }
//             count += 1;
//         }
//         count
//     }
// }

impl Solution {
    // 递归法
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => return 0,
            Some(node) => {
                i32::max(Solution::max_depth(node.borrow().left.clone()),
                         Solution::max_depth(node.borrow().right.clone())) + 1
            }
        }
    }
}