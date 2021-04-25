// 108. 将有序数组转换为二叉搜索树
// 给你一个整数数组 nums ，其中元素已经按 升序 排列，请你将其转换为一棵 高度平衡 二叉搜索树。
// 
// 高度平衡 二叉树是一棵满足「每个节点的左右两个子树的高度差的绝对值不超过 1 」的二叉树。
//
// 示例 1：
//
// 输入：nums = [-10,-3,0,5,9]
// 输出：[0,-3,9,-10,null,5]
// 解释：[0,-10,5,null,-3,null,9] 也将被视为正确答案：
// 
// 示例 2：
//
// 输入：nums = [1,3]
// 输出：[3,1]
// 解释：[1,3] 和 [3,1] 都是高度平衡二叉搜索树。
//
// 提示：
// 
// 1 <= nums.length <= 104
// -104 <= nums[i] <= 104
// nums 按 严格递增 顺序排列
// 
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/convert-sorted-array-to-binary-search-tree
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

use rustleetcode::TreeNode;
use rustleetcode::options_vec;

fn main() {
    // let v = vec![1,2,3];
    // println!("{:?}", &v[3..3]); // 3..4或4..3都会报错
    // println!("{:?}", &v[0..0]);
    let res = Solution::sorted_array_to_bst(vec![-10,-3,0,5,9]);
    println!("{:?}", res);
}

struct Solution;

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(nums: &Vec<i32>, l: usize, r: usize) -> Option<Rc<RefCell<TreeNode>>> {
            if l == r { return None; } // 范围不包括r，则取的是中间 或 靠右。范围包括r，则取的是中间 或 靠左。

            let mid = (l + r) / 2;
            let mut node = TreeNode::new(nums[mid]);
            node.left = helper(nums, l, mid); // 如果范围包括r，则这里是l..=mid-1，需要判断l<mid或mid>0来防止usize类型的mid溢出
            node.right = helper(nums, mid + 1, r);
            Some(Rc::new(RefCell::new(node)))
        }

        helper(&nums,0,nums.len())
    }

    // 不用修改nums的可以考虑用切片
    // pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    //     fn helper(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    //         if nums.is_empty() { return None; }
    //
    //         let mid = (0 + nums.len()) / 2;
    //         Some(Rc::new(RefCell::new(TreeNode {
    //             val: nums[mid],
    //             left: helper(&nums[0..mid]),
    //             right: helper(&nums[mid + 1..nums.len()])
    //         })))
    //     }
    //
    //     helper(&nums[..])
    // }


}

