// 100. 相同的树
// 给你两棵二叉树的根节点 p 和 q ，编写一个函数来检验这两棵树是否相同。
// 
// 如果两个树在结构上相同，并且节点具有相同的值，则认为它们是相同的。
//
// 示例 1：
//
// 输入：p = [1,2,3], q = [1,2,3]
// 输出：true
// 示例 2：
//
// 输入：p = [1,2], q = [1,null,2]
// 输出：false
// 示例 3：
// 
// 
// 输入：p = [1,2,1], q = [1,1,2]
// 输出：false
//  
// 
// 提示：
// 
// 两棵树上的节点数目都在范围 [0, 100] 内
// -104 <= Node.val <= 104
// 
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/same-tree
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

use rustleetcode::TreeNode;
use rustleetcode::options_vec;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let p = TreeNode::from_vec(options_vec![1, 2, 3]);
    let q = TreeNode::from_vec(options_vec![1, 2, 3]);
    assert!(is_same_tree(p, q));

    let p = TreeNode::from_vec(options_vec![1, 2]);
    let q = TreeNode::from_vec(options_vec![1, None ,2]);
    assert!(!is_same_tree(p, q));

    let p = TreeNode::from_vec(options_vec![1,2,1]);
    let q = TreeNode::from_vec(options_vec![1,1,2]);
    assert!(!is_same_tree(p, q));
}

pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (p, q) {
        (None, None) => true,
        (Some(p), Some(q)) => {
            p.borrow().val == q.borrow().val &&
                is_same_tree(p.borrow().left.clone(), q.borrow().left.clone()) && // 提交的代码要加 Solution::
                is_same_tree(p.borrow().right.clone(), q.borrow().right.clone())
        },
        _ => false
    }
}

// pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
//     p == q
// }

// pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
//     if p.is_none() || q.is_none() {
//         return p.is_none() && q.is_none();
//     }
//
//     let p = p.unwrap();
//     let q = q.unwrap();
//     if p.borrow().val != q.borrow().val {
//         return false;
//     }
//
//     let mut pMut = p.borrow_mut();
//     let mut qMut = q.borrow_mut();
//     // let res = Solution::is_same_tree(pMut.left.take(), qMut.left.take()) && // 需要加Solution::
//     //     Solution::is_same_tree(pMut.right.take(), qMut.right.take());
//     let res = is_same_tree(pMut.left.take(), qMut.left.take()) &&
//     is_same_tree(pMut.right.take(), qMut.right.take());
//
//     res
// }
