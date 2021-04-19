// 27. 移除元素
// 给你一个数组 nums 和一个值 val，你需要 原地 移除所有数值等于 val 的元素，并返回移除后数组的新长度。
// 
// 不要使用额外的数组空间，你必须仅使用 O(1) 额外空间并 原地 修改输入数组。
// 
// 元素的顺序可以改变。你不需要考虑数组中超出新长度后面的元素。
// 
//  
// 
// 说明:
// 
// 为什么返回数值是整数，但输出的答案是数组呢?
// 
// 请注意，输入数组是以「引用」方式传递的，这意味着在函数里修改输入数组对于调用者是可见的。
// 
// 你可以想象内部操作如下:
// 
// // nums 是以“引用”方式传递的。也就是说，不对实参作任何拷贝
// int len = removeElement(nums, val);
// 
// // 在函数里修改输入数组对于调用者是可见的。
// // 根据你的函数返回的长度, 它会打印出数组中 该长度范围内 的所有元素。
// for (int i = 0; i < len; i++) {
//     print(nums[i]);
// }
//  
// 
// 示例 1：
// 
// 输入：nums = [3,2,2,3], val = 3
// 输出：2, nums = [2,2]
// 解释：函数应该返回新的长度 2, 并且 nums 中的前两个元素均为 2。你不需要考虑数组中超出新长度后面的元素。例如，函数返回的新长度为 2 ，而 nums = [2,2,3,3] 或 nums = [2,2,0,0]，也会被视作正确答案。
// 示例 2：
// 
// 输入：nums = [0,1,2,2,3,0,4,2], val = 2
// 输出：5, nums = [0,1,4,0,3]
// 解释：函数应该返回新的长度 5, 并且 nums 中的前五个元素为 0, 1, 3, 0, 4。注意这五个元素可为任意顺序。你不需要考虑数组中超出新长度后面的元素。
//  
// 
// 提示：
// 
// 0 <= nums.length <= 100
// 0 <= nums[i] <= 50
// 0 <= val <= 100
// 
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/remove-element
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn main() {
    let mut vec = vec![3, 2, 2, 3];
    println!("{}", remove_element(&mut vec, 3)); // 2
    println!("{:?}", vec); // [2,2]

    let mut vec = vec![0,1,2,2,3,0,4,2];
    println!("{}", remove_element(&mut vec, 2)); // 5
    println!("{:?}", vec); // [0,1,4,0,3]

    let mut vec = vec![];
    println!("{}", remove_element(&mut vec, 2));
    println!("{:?}", vec);

    let mut vec = vec![2];
    println!("{}", remove_element(&mut vec, 3)); // 1
    println!("{:?}", vec);

    let mut vec = vec![1];
    println!("{}", remove_element(&mut vec, 1)); // 0
    println!("{:?}", vec);
}

// 双指针
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut j = 0;
    for i in 0..nums.len() {
        if val != nums[i] {
            nums[j] = nums[i]; // 在没有遇到第一个等于val值前，j都等于i
            j += 1;
        }
    }
    j as i32
}

// 先排序，后处理
// pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
//     if nums.len() == 0 { return 0; }
//
//     nums.sort();
//
//     let mut j = nums.len() - 1; // usize不能小于0
//     let mut i = 0;
//     while i <= j {
//         if nums[i] == val { // 需要交换
//             if nums[i] == nums[j] { // 已经没有可以交换的元素了
//                 break;
//             } else {
//                 nums[i] = nums[j]; // 把后面的放过来
//                 j -= 1;
//             }
//         }
//         i += 1;
//     }
//
//     i as i32
// }
