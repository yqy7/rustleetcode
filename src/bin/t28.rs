// 28. 实现 strStr()
// 实现 strStr() 函数。
// 
// 给你两个字符串 haystack 和 needle ，请你在 haystack 字符串中找出 needle 字符串出现的第一个位置（下标从 0 开始）。如果不存在，则返回  -1 。
// 
//  
// 
// 说明：
// 
// 当 needle 是空字符串时，我们应当返回什么值呢？这是一个在面试中很好的问题。
// 
// 对于本题而言，当 needle 是空字符串时我们应当返回 0 。这与 C 语言的 strstr() 以及 Java 的 indexOf() 定义相符。
// 
//  
// 
// 示例 1：
// 
// 输入：haystack = "hello", needle = "ll"
// 输出：2
// 示例 2：
// 
// 输入：haystack = "aaaaa", needle = "bba"
// 输出：-1
// 示例 3：
// 
// 输入：haystack = "", needle = ""
// 输出：0
//  
// 
// 提示：
// 
// 0 <= haystack.length, needle.length <= 5 * 104
// haystack 和 needle 仅由小写英文字符组成
// 
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/implement-strstr
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn main() {
    let haystack = "hello".to_string();
    let needle = "ll".to_string();
    assert_eq!(str_str(haystack, needle), 2);

    let haystack = "aaaaa".to_string();
    let needle = "bba".to_string();
    assert_eq!(str_str(haystack, needle), -1);

    let haystack = "".to_string();
    let needle = "".to_string();
    assert_eq!(str_str(haystack, needle), 0);
}

// pub fn str_str(haystack: String, needle: String) -> i32 {
//     haystack.find(&needle).map(|i| i as i32).unwrap_or(-1)
// }

// 暴力法：非常慢
// pub fn str_str(haystack: String, needle: String) -> i32 {
//     if needle.len() == 0 { return 0; }
//     if needle.len() > haystack.len() { return -1; }
//
//     let haystack: Vec<char> = haystack.chars().collect();
//     let needle: Vec<char> = needle.chars().collect();
//
//     for i in 0..haystack.len() {
//         if haystack[i] != needle[0] {
//             continue;
//         }
//
//         let mut j = 0;
//         while j < needle.len() && i + j < haystack.len() && haystack[i + j] == needle[j] {
//             j += 1;
//         }
//         if j == needle.len() {
//             return i as i32;
//         }
//     }
//
//     -1
// }

pub fn str_str(haystack: String, needle: String) -> i32 {
    if needle.len() == 0 { return 0; }
    if needle.len() > haystack.len() { return -1; }

    let haystack: Vec<char> = haystack.chars().collect();
    let needle: Vec<char> = needle.chars().collect();

    for i in 0..haystack.len() - needle.len() + 1 { // 这一步优化很明显
        if haystack[i] != needle[0] {
            continue;
        }

        let mut j = 0;
        while j < needle.len() && i + j < haystack.len() && haystack[i + j] == needle[j] {
            j += 1;
        }
        if j == needle.len() {
            return i as i32;
        }
    }

    -1
}

// TODO kmp
