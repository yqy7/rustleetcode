// 118. 杨辉三角
// 给定一个非负整数 numRows，生成杨辉三角的前 numRows 行。
//
//
//
// 在杨辉三角中，每个数是它左上方和右上方的数的和。
//
// 示例:
//
// 输入: 5
// 输出:
// [
//      [1],
//     [1,1],
//    [1,2,1],
//   [1,3,3,1],
//  [1,4,6,4,1]
// ]
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/pascals-triangle
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

struct Solution;
// impl Solution {
//     pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
//         let mut res = Vec::new();
//         res.push(vec![1]);
//         for i in 1..num_rows as usize {
//             let mut row = vec![1];
//             for j in 1..res[i - 1].len() {
//                 row.push(res[i - 1][j] + res[i - 1][j - 1]);
//             }
//             row.push(1);
//             res.push(row);
//         }
//
//         res
//     }
// }

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        for i in 1..=num_rows as usize {
            let mut row = vec![1;i]; // 初始化为1
            for j in 1..row.len() - 1 {
                row[j] = res[i - 2][j] + res[i - 2][j - 1];
            }
            res.push(row);
        }

        res
    }
}

fn main() {
    let res = Solution::generate(5);
    for v in res {
        println!("{:?}", v);
    }
}