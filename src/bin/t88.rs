// 88. 合并两个有序数组
// 给你两个有序整数数组 nums1 和 nums2，请你将 nums2 合并到 nums1 中，使 nums1 成为一个有序数组。
// 
// 初始化 nums1 和 nums2 的元素数量分别为 m 和 n 。你可以假设 nums1 的空间大小等于 m + n，这样它就有足够的空间保存来自 nums2 的元素。
// 
//  
// 
// 示例 1：
// 
// 输入：nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
// 输出：[1,2,2,3,5,6]
// 示例 2：
// 
// 输入：nums1 = [1], m = 1, nums2 = [], n = 0
// 输出：[1]
//  
// 
// 提示：
// 
// nums1.length == m + n
// nums2.length == n
// 0 <= m, n <= 200
// 1 <= m + n <= 200
// -109 <= nums1[i], nums2[i] <= 109
// 
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/merge-sorted-array
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn main() {
    let mut nums1 = vec![1,3,8,0,0,0];
    let mut nums2 = vec![2, 5, 6];
    merge(&mut nums1, 3, &mut nums2, 3);
    println!("{:?}", nums1);

    let mut nums1 = vec![1];
    let mut nums2 = vec![];
    merge(&mut nums1, 1, &mut nums2, 0);
    println!("{:?}", nums1);

    let mut nums1 = vec![0];
    let mut nums2 = vec![1];
    merge(&mut nums1, 0, &mut nums2, 1);
    println!("{:?}", nums1);
}

// 倒序遍历：O(n)
pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let (mut p1, mut p2) = (m as usize, n as usize);
    while p2 > 0 {
        if p1 == 0 || nums1[p1 - 1] < nums2[p2 - 1] {
            nums1[p1 + p2 - 1] = nums2[p2 - 1];
            p2 -= 1;
        } else {
            nums1[p1 + p2 - 1] = nums1[p1 - 1];
            p1 -= 1;
        }
    }
}

// pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
//     let mut p1 = m - 1;
//     let mut p2 = n - 1;
//     let mut p3 = n + m - 1;
//     while p1 >= 0 && p2 >= 0 {
//         if nums1[p1 as usize] > nums2[p2 as usize] {
//             nums1[p3 as usize] = nums1[p1 as usize];
//             p1 -= 1;
//         } else {
//             nums1[p3 as usize] = nums2[p2 as usize];
//             p2 -= 1;
//         }
//         p3 -= 1;
//     }
//     while p2 >= 0 {
//         nums1[p3 as usize] = nums2[p2 as usize];
//         p2 -= 1;
//         p3 -= 1;
//     }
// }

// 排序法：O(nlogn)
// pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
//     nums1[m as usize..].copy_from_slice(nums2);
//     nums1.sort();
// }
