// 119. 杨辉三角 II
// 给定一个非负索引 k，其中 k ≤ 33，返回杨辉三角的第 k 行。
//
//
//
// 在杨辉三角中，每个数是它左上方和右上方的数的和。
//
// 示例:
//
// 输入: 3
// 输出: [1,3,3,1]
// 进阶：
//
// 你可以优化你的算法到 O(k) 空间复杂度吗？
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/pascals-triangle-ii
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn main() {
    println!("{:?}", Solution::get_row(0));
    println!("{:?}", Solution::get_row(1));
    println!("{:?}", Solution::get_row(2));
    println!("{:?}", Solution::get_row(3));
    println!("{:?}", Solution::get_row(4));
}

struct Solution;
impl Solution {
    // pub fn get_row(row_index: i32) -> Vec<i32> {
    //     let mut row = vec![1; (row_index + 1) as usize];
    //     for i in 1..row_index as usize {
    //         let mut pre = row[0];
    //         for j in 1..i + 1 {
    //             row[j] += pre;
    //             pre = row[j] - pre;
    //         }
    //     }
    //     row
    // }
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut row = vec![];
        for i in 0..=row_index as usize {
            row.push(1);
            for j in (1..i).rev() { // 逆序修改
                row[j] = row[j] + row[j - 1];
            }
        }
        row
    }
}
