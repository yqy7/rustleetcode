// 110. 平衡二叉树
// 给定一个二叉树，判断它是否是高度平衡的二叉树。
// 
// 本题中，一棵高度平衡二叉树定义为：
// 
// 一个二叉树每个节点 的左右两个子树的高度差的绝对值不超过 1 。
// 
//  
// 
// 示例 1：
// 
// 
// 输入：root = [3,9,20,null,null,15,7]
// 输出：true
// 示例 2：
// 
// 
// 输入：root = [1,2,2,3,3,null,null,4,4]
// 输出：false
// 示例 3：
// 
// 输入：root = []
// 输出：true
//  
// 
// 提示：
// 
// 树中的节点数在范围 [0, 5000] 内
// -104 <= Node.val <= 104
// 
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/balanced-binary-tree
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

use rustleetcode::TreeNode;
use rustleetcode::options_vec;

fn main() {
    assert!(Solution::is_balanced(TreeNode::from_vec(options_vec![3,9,20,None,None,15,7])));
    assert!(!Solution::is_balanced(TreeNode::from_vec(options_vec![1,2,2,3,3,None,None,4,4])));
    assert!(Solution::is_balanced(TreeNode::from_vec(options_vec![])));
}

struct Solution;
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn height(root: Option<Rc<RefCell<TreeNode>>>) -> i32 { // -1表示已经存在高度差大于1的情况
            match root {
                None => 0,
                Some(node) => {
                    let left = height(node.borrow().left.clone());
                    let right = height(node.borrow().right.clone());
                    if left == -1 || right == -1 || (left - right).abs() > 1 { return -1; }
                    left.max(right) + 1
                }
            }
        }
        height(root) != -1
    }
}