// 66. 加一
// 给定一个由 整数 组成的 非空 数组所表示的非负整数，在该数的基础上加一。
// 
// 最高位数字存放在数组的首位， 数组中每个元素只存储单个数字。
// 
// 你可以假设除了整数 0 之外，这个整数不会以零开头。
// 
//  
// 
// 示例 1：
// 
// 输入：digits = [1,2,3]
// 输出：[1,2,4]
// 解释：输入数组表示数字 123。
// 示例 2：
// 
// 输入：digits = [4,3,2,1]
// 输出：[4,3,2,2]
// 解释：输入数组表示数字 4321。
// 示例 3：
// 
// 输入：digits = [0]
// 输出：[1]
//  
// 
// 提示：
// 
// 1 <= digits.length <= 100
// 0 <= digits[i] <= 9
// 
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/plus-one
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn main() {
    let digits = vec![1, 2, 3];
    assert_eq!(plus_one(digits), vec![1, 2, 4]);

    let digits = vec![4,3,2,1];
    assert_eq!(plus_one(digits), vec![4,3,2,2]);

    let digits = vec![0];
    assert_eq!(plus_one(digits), vec![1]);

    let digits = vec![9];
    assert_eq!(plus_one(digits), vec![1,0]);
}

pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut res = vec![];
    let mut carry = 1;
    for n in digits.iter().rev() {
        res.push((n + carry) % 10);
        carry = (n + carry) / 10;
    }
    if carry != 0 { res.push(1); }

    res.reverse();
    res
}

// pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
//     let mut res = digits.clone();
//     for n in res.iter_mut().rev() {
//         *n += 1;
//         *n %= 10;
//         if *n != 0 { return res; } // *n == 0 说明有进位了
//     }
//
//     // 只有在全是9的情况才可能走到这里
//     let mut res = vec![0i32; res.len() + 1];
//     res[0] = 1;
//     res
// }
