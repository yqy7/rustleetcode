// 67. 二进制求和
// 给你两个二进制字符串，返回它们的和（用二进制表示）。
// 
// 输入为 非空 字符串且只包含数字 1 和 0。
// 
//  
// 
// 示例 1:
// 
// 输入: a = "11", b = "1"
// 输出: "100"
// 示例 2:
// 
// 输入: a = "1010", b = "1011"
// 输出: "10101"
//  
// 
// 提示：
// 
// 每个字符串仅由字符 '0' 或 '1' 组成。
// 1 <= a.length, b.length <= 10^4
// 字符串如果不是 "0" ，就都不含前导零。
// 
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/add-binary
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn main() {
    assert_eq!(add_binary("11".to_string(), "1".to_string()), "100");
    assert_eq!(add_binary("1010".to_string(), "1011".to_string()), "10101");

    println!("{}", add_binary("10100000100100110110010000010101111011011001101110111111111101000000101111001110001111100001101".to_string(),
"110101001011101110001111100110001010100001101011101010000011011011001011101111001100000011011110011".to_string()));
}

pub fn add_binary(a: String, b: String) -> String {
    fn c2i(c: char) -> i32 {
        match c { '0' => 0, _ => 1 }
    }
    fn i2c(i: i32) -> char {
        match i { 0 => '0', _ => '1'}
    }

    let (a, b): (Vec<char>, Vec<char>) = (a.chars().rev().collect(), b.chars().rev().collect());
    let n = a.len().max(b.len());

    let mut res = vec![];
    let mut carry  = 0;
    for i in 0..n {
        carry += if i < a.len() { c2i(a[i]) } else { 0 };
        carry += if i < b.len() { c2i(b[i]) } else { 0 };
        res.push(i2c(carry % 2));
        carry /= 2;
    }
    if carry != 0 { res.push('1'); }

    res.reverse();
    res.into_iter().collect()
}

// pub fn add_binary(a: String, b: String) -> String {
//     let mut res = vec![];
//     let (l_str, s_str): (Vec<u8>, Vec<u8>) = if a.len() > b.len() {
//         (a.bytes().rev().collect(), b.bytes().rev().collect())
//     } else {
//         (b.bytes().rev().collect(), a.bytes().rev().collect())
//     };
//
//     let mut carry = 0;
//     for i in 0..l_str.len() {
//         let n = if i < s_str.len() {
//             l_str[i] as u8 + s_str[i] as u8 + carry - 2 * b'0'
//         } else {
//             l_str[i] as u8 - b'0' + carry
//         };
//         carry = n / 2;
//         res.push((n % 2) + b'0');
//     }
//     if carry != 0 {
//         res.push(b'1');
//     }
//
//     res.reverse();
//     String::from_utf8(res).unwrap()
// }

// 此方法会溢出，这里记录一下整数和二进制串之间的转换
// pub fn add_binary(a: String, b: String) -> String {
//     format!("{:b}", i32::from_str_radix(&a, 2).unwrap() + i32::from_str_radix(&b, 2).unwrap())
// }