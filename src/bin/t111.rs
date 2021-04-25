// 111. 二叉树的最小深度
// 给定一个二叉树，找出其最小深度。
// 
// 最小深度是从根节点到最近叶子节点的最短路径上的节点数量。
// 
// 说明：叶子节点是指没有子节点的节点。
// 
//  
// 
// 示例 1：
// 
// 
// 输入：root = [3,9,20,null,null,15,7]
// 输出：2
// 示例 2：
// 
// 输入：root = [2,null,3,null,4,null,5,null,6]
// 输出：5
//  
// 
// 提示：
// 
// 树中节点数的范围在 [0, 105] 内
// -1000 <= Node.val <= 1000
// 
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/minimum-depth-of-binary-tree
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

use std::rc::Rc;
use std::cell::RefCell;
use rustleetcode::TreeNode;
use rustleetcode::options_vec;
use std::collections::VecDeque;
use std::option::Option::Some;

struct Solution;
// 递归，深度遍历
// impl Solution {
//     pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
//         match root {
//             None => 0,
//             Some(root) => {
//                 match (root.borrow().left.clone(), root.borrow().right.clone()) {
//                     (None, None) => 1,
//                     (left, None) => {
//                         Self::min_depth(left) + 1
//                     },
//                     (None, right) => {
//                         Self::min_depth(right) + 1
//                     },
//                     (left, right) => {
//                         Self::min_depth(left).min(Self::min_depth(right)) + 1
//                     }
//                 }
//             }
//         }
//     }
// }

// impl Solution {
//     pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
//         match root {
//             None => 0,
//             Some(root) => {
//                 let leftHeight = Self::min_depth(root.borrow().left.clone());
//                 let rightHeight = Self::min_depth(root.borrow().right.clone());
//                 if leftHeight == 0 || rightHeight == 0 {
//                     return leftHeight.max(rightHeight) + 1;
//                 } else {
//                     return leftHeight.min(rightHeight) + 1;
//                 }
//             }
//         }
//     }
// }

// 迭代，广度遍历，更快
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() { return 0; }

        let mut deque = VecDeque::new();
        deque.push_back(root);
        let mut count = 0;
        while !deque.is_empty() {
            count += 1;
            let size = deque.len();
            for _ in 0..size {
                let node = deque.pop_front().unwrap();
                if let Some(node) = node {
                    if node.borrow().left.is_none() && node.borrow().right.is_none() {
                        return count;
                    } else {
                        deque.push_back(node.borrow().left.clone());
                        deque.push_back(node.borrow().right.clone());
                    }
                }
            }
        }
        count
    }
}

fn main() {
    assert_eq!(Solution::min_depth(TreeNode::from_vec(options_vec![3,9,20,None,None,15,7])), 2);
    assert_eq!(Solution::min_depth(TreeNode::from_vec(options_vec![2,None,3,None,4,None,5,None,6])), 5);
}