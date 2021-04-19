// 7. 整数反转
// 给你一个 32 位的有符号整数 x ，返回将 x 中的数字部分反转后的结果。
//
// 如果反转后整数超过 32 位的有符号整数的范围[−231, 231− 1] ，就返回 0。
//
// 假设环境不允许存储 64 位整数（有符号或无符号）。
// 
//
// 示例 1：
//
// 输入：x = 123
// 输出：321
// 示例 2：
//
// 输入：x = -123
// 输出：-321
// 示例 3：
//
// 输入：x = 120
// 输出：21
// 示例 4：
//
// 输入：x = 0
// 输出：0
// 
//
// 提示：
//
// -231 <= x <= 231 - 1
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/reverse-integer
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
fn main() {
    assert_eq!(reverse(123), 321);
    assert_eq!(reverse(-123), -321);
    assert_eq!(reverse(120), 21);
    assert_eq!(reverse(0), 0);

    // println!("{}", reverse(-123));
    println!("{}", reverse(i32::MIN));
    println!("{}", reverse(i32::MAX));
    println!("{}", reverse(-2147483412));
}

// 1).一个数会反转溢出，则位数必须与最值一样。对于一个32位整数，最开头(反转的最后一位)只能是1或2
// 2).如果(位数-1)个数字反转之后：
//  a.小于 MAX/10，则最后一位是任何数也小于MAX，即不会溢出
//  b.大于 MAX/10，则最后一位是任何数也大于MAX，即会溢出
//  c.等于 MAX/10，则最后一位必须小于MAX%10(MAX是7，MIN是-8)，而根据1)这是必然的，即不会溢出
// 综上，只需要判断 大于 MAX/10 的情况，同理负数判断小于 MIN/10的情况
pub fn reverse(x: i32) -> i32 {
    let (MAX_LIMIT, MIN_LIMIT) = (i32::MAX / 10, i32::MIN / 10);
    let (mut n ,mut r) = (x, 0);
    while n != 0 {
        if r > MAX_LIMIT || r < MIN_LIMIT { return 0; }
        r = r * 10 + n % 10;
        n /= 10;
    }
    r
}

// fn reverse(x: i32) -> i32 {
//     fn helper(mut n: i32) -> Option<i32> {
//         let mut res = 0i32;
//         while n.abs() != 0 {
//             res = res.checked_mul(10)?.checked_add(n % 10)?; // checked_mul如果溢出会返回None
//             n /= 10;
//         }
//         Some(res)
//     }
//     helper(x).unwrap_or_default()
// }