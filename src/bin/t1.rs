// 1. 两数之和
// 给定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出 和为目标值 的那 两个 整数，并返回它们的数组下标。
//
// 你可以假设每种输入只会对应一个答案。但是，数组中同一个元素在答案里不能重复出现。
//
// 你可以按任意顺序返回答案。
//
//  
//
// 示例 1：
//
// 输入：nums = [2,7,11,15], target = 9
// 输出：[0,1]
// 解释：因为 nums[0] + nums[1] == 9 ，返回 [0, 1] 。
// 示例 2：
//
// 输入：nums = [3,2,4], target = 6
// 输出：[1,2]
// 示例 3：
//
// 输入：nums = [3,3], target = 6
// 输出：[0,1]
//  
//
// 提示：
//
// 2 <= nums.length <= 103
// -109 <= nums[i] <= 109
// -109 <= target <= 109
// 只会存在一个有效答案
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/two-sum
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn main() {
    let num = vec![2,7,11,15];
    let target = 9;
    println!("{:?}", two_sum(num, target));

    let num = vec![3,2,4];
    let target = 6;
    println!("{:?}", two_sum(num, target));

    let num = vec![3, 3];
    let target = 6;
    println!("{:?}", two_sum(num, target));
}

// 要求返回数组下标，因此不能用排序
// 暴力法O(n2)
// hashmap
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    for i in 0..nums.len() {
        if let Some(&v) = map.get(&(target - nums[i])) {
            return vec![v, i as i32];
        }
        map.insert(nums[i], i as i32);
    }
    vec![]
}

// pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//     use std::collections::HashMap;
//     let mut map = HashMap::new();
//     for i in 0..nums.len() {
//         match map.get(&(target - nums[i])) {
//             Some(&v) => { return vec![v, i as i32]; }
//             _ => { map.insert(nums[i], i as i32); }
//         }
//     }
//     vec![]
// }

// pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//     use std::collections::HashMap;
//
//     let mut map = HashMap::new();
//     for i in 0..nums.len() {
//         map.insert(nums[i], i as i32);
//     }
//
//     for i in 0..nums.len() {
//         if let Some(&v) = map.get(&(target - nums[i])) {
//             if v != i as i32 {
//                 return vec![i as i32, v];
//             }
//         }
//     }
//
//     Vec::new()
// }

// pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//     use std::collections::HashMap;
//
//     let mut map = HashMap::new();
//     for (i,n) in nums.iter().enumerate() {
//         map.insert(*n, i as i32);
//     }
//
//     for (i, n) in nums.iter().enumerate() {
//         let i = i as i32;
//         match map.get(&(target - n)) {
//             Some(v) if *v != i => {
//                 return vec![i as i32, *v];
//             }
//             _ => {}
//         }
//     }
//
//     Vec::new()
// }

// pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//     let mut map = HashMap::new();
//     for (i,n) in nums.iter().enumerate() {
//         map.insert(*n, i as i32);
//     }
//
//     for (i, n) in nums.iter().enumerate() {
//         let i = i as i32;
//         let v = map.get(&(target - n));
//         match v {
//             Some(value) => {
//                 if *value != i {
//                     return vec![i, *value];
//                 }
//             },
//             None => {}
//         }
//     }
//
//     Vec::new()
// }