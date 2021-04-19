// 9. 回文数
// 给你一个整数 x ，如果 x 是一个回文整数，返回 true ；否则，返回 false 。
//
// 回文数是指正序（从左向右）和倒序（从右向左）读都是一样的整数。例如，121 是回文，而 123 不是。
//
//  
//
// 示例 1：
//
// 输入：x = 121
// 输出：true
// 示例 2：
//
// 输入：x = -121
// 输出：false
// 解释：从左向右读, 为 -121 。 从右向左读, 为 121- 。因此它不是一个回文数。
// 示例 3：
//
// 输入：x = 10
// 输出：false
// 解释：从右向左读, 为 01 。因此它不是一个回文数。
// 示例 4：
//
// 输入：x = -101
// 输出：false
//  
//
// 提示：
//
// -231 <= x <= 231 - 1
//  
//
// 进阶：你能不将整数转为字符串来解决这个问题吗？
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/palindrome-number
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn main() {
    assert!(is_palindrome(121));
    assert!(!is_palindrome(-121));
    assert!(!is_palindrome(10));
    assert!(!is_palindrome(-101));
    assert!(!is_palindrome(1234567899));
}

// 整数反转一半
pub fn is_palindrome(x: i32) -> bool {
    if x < 0 || (x != 0 && x % 10 == 0) { return false; }

    let mut n = x;
    let mut r = 0;
    while n > r { // 对于回文数：奇数个数字停止时 r > n，偶数个数字停止时 r == n，非回文数也可以使用r > n的停止条件
        r = r * 10 + n % 10;
        n /= 10;
    }
    n == r || r / 10 == n
}

// 字符串反转
// pub fn is_palindrome(x: i32) -> bool {
//     if x < 0 { return false; }
//
//     let xs0 = x.to_string();
//     let mut xs1 = xs0.clone();
//     unsafe { xs1.as_bytes_mut().reverse(); }
//     xs0 == xs1
// }

// 整数反转，见t7
// pub fn is_palindrome(x: i32) -> bool {
//     const MAX_BOUND: i32 = i32::MAX / 10;
//     const MIN_BOUND: i32 = i32::MIN / 10;
//     if x < 0 { return false; }
//
//     let mut n = x;
//     let mut r = 0;
//     while n != 0 {
//         if r > MAX_BOUND || r < MIN_BOUND { return false; } // 回文数不可能反转之后溢出
//         r = r * 10 + n % 10;
//         n /= 10;
//     }
//     r == x
// }

