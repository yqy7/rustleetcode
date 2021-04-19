// 35. 搜索插入位置
// 给定一个排序数组和一个目标值，在数组中找到目标值，并返回其索引。如果目标值不存在于数组中，返回它将会被按顺序插入的位置。
//
// 你可以假设数组中无重复元素。
//
// 示例 1:
//
// 输入: [1,3,5,6], 5
// 输出: 2
// 示例 2:
//
// 输入: [1,3,5,6], 2
// 输出: 1
// 示例 3:
//
// 输入: [1,3,5,6], 7
// 输出: 4
// 示例 4:
//
// 输入: [1,3,5,6], 0
// 输出: 0
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/search-insert-position
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn main() {
    assert_eq!(search_insert(vec![1,3,5,6], 5), 2);
    assert_eq!(search_insert(vec![1,3,5,6], 2), 1);
    assert_eq!(search_insert(vec![1,3,5,6], 7), 4);
    assert_eq!(search_insert(vec![1,3,5,6], 0), 0);
}

// 实现二分查找
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut l = 0;
    let mut r = nums.len(); // 不包含r，如果用nums.len() - 1后面r会减到-1溢出不好处理
    while l < r {
        let mid = (l + r) >> 1;
        if nums[mid] == target {
            return mid as i32;
        } else if nums[mid] > target { // 不包含mid
            r = mid;
        } else {
            l = mid + 1;
        }
    }

    l as i32
}

// 自带的二分查找 1
// pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
//     match nums.binary_search(&target) {
//         Ok(i) | Err(i) => i as i32,
//     }
// }

// 自带的二分查找 2
// pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
//     nums.binary_search(&target).unwrap_or_else(|x| x) as i32 // unwrap_or_else可以传入一个函数处理Err
// }

