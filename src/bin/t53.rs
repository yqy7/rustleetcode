// 53. 最大子序和
// 给定一个整数数组 nums ，找到一个具有最大和的连续子数组（子数组最少包含一个元素），返回其最大和。
//
// 示例 1：
//
// 输入：nums = [-2,1,-3,4,-1,2,1,-5,4]
// 输出：6
// 解释：连续子数组 [4,-1,2,1] 的和最大，为 6 。
// 示例 2：
//
// 输入：nums = [1]
// 输出：1
// 示例 3：
//
// 输入：nums = [0]
// 输出：0
// 示例 4：
//
// 输入：nums = [-1]
// 输出：-1
// 示例 5：
//
// 输入：nums = [-100000]
// 输出：-100000
//  
//
// 提示：
//
// 1 <= nums.length <= 3 * 104
// -105 <= nums[i] <= 105
//  
//
// 进阶：如果你已经实现复杂度为 O(n) 的解法，尝试使用更为精妙的 分治法 求解。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/maximum-subarray
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn main() {
    let vec = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    assert_eq!(max_sub_array(vec), 6);
    let vec = vec![1];
    assert_eq!(max_sub_array(vec), 1);
    let vec = vec![0];
    assert_eq!(max_sub_array(vec), 0);
    let vec = vec![-1];
    assert_eq!(max_sub_array(vec), -1);
    let vec = vec![-100000];
    assert_eq!(max_sub_array(vec), -100000);
}

// 动态规划2：dp[n] 只与 dp[n-1] 相关，用一个变量保存即可
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut prev = nums[0];
    let mut max = nums[0];
    for i in 1..nums.len() {
        prev = if prev < 0 { nums[i] } else { prev + nums[i] };
        max = max.max(prev);
    }
    max
}

// 动态规划1
// [-2, 1, -3, 4, -1, 2, 1, -5, 4] f(n)表示到n为止最大连续的和，如果f(n-1)>0，f(n)=f(n-1)+nums[n]，否则f(n)=nums[n]
// pub fn max_sub_array(nums: Vec<i32>) -> i32 {
//     let mut dp = vec![0; nums.len()];
//     let mut max = nums[0];
//     for i in 0..nums.len() {
//         if i == 0 || dp[i - 1] < 0 {
//             dp[i] = nums[i];
//         } else {
//             dp[i] = dp[i - 1] + nums[i];
//         }
//         max = std::cmp::max(max, dp[i]);
//     }
//     max
// }