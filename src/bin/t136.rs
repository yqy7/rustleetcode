// 136. 只出现一次的数字
// 给定一个非空整数数组，除了某个元素只出现一次以外，其余每个元素均出现两次。找出那个只出现了一次的元素。
// 
// 说明：
// 
// 你的算法应该具有线性时间复杂度。 你可以不使用额外空间来实现吗？
// 
// 示例 1:
// 
// 输入: [2,2,1]
// 输出: 1
// 示例 2:
// 
// 输入: [4,1,2,1,2]
// 输出: 4
// 
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/single-number
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn main() {
    assert_eq!(1, Solution::single_number(vec![2,2,1]));
    assert_eq!(4, Solution::single_number(vec![4,1,2,1,2]));
    // println!("{:?}", vec![4, 1, 2, 1, 2].windows(2).map(|w| w[0]+w[1]).collect::<Vec<i32>>());
}

struct Solution;
// impl Solution {
//     pub fn single_number(nums: Vec<i32>) -> i32 {
//         nums.into_iter().reduce(|x0,x1| x0^x1).unwrap()
//     }
// }

// impl Solution {
//     pub fn single_number(nums: Vec<i32>) -> i32 {
//         let mut res = 0;
//         for n in nums {
//             res ^= n;
//         }
//         res
//     }
// }

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |x0,x1| x0^x1)
    }
}
