// 125. 验证回文串
// 给定一个字符串，验证它是否是回文串，只考虑字母和数字字符，可以忽略字母的大小写。
//
// 说明：本题中，我们将空字符串定义为有效的回文串。
//
// 示例 1:
//
// 输入: "A man, a plan, a canal: Panama"
// 输出: true
// 示例 2:
//
// 输入: "race a car"
// 输出: false
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/valid-palindrome
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

struct Solution;
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars: Vec<char> = s.chars().collect();

        let mut i = 0;
        let mut j = chars.len() - 1;
        while i < j {
            if !chars[i].is_ascii_alphanumeric() {
                i += 1;
                continue;
            }
            if !chars[j].is_ascii_alphanumeric() {
                j -= 1;
                continue;
            }
            if chars[i].to_ascii_lowercase() != chars[j].to_ascii_lowercase() {
                return false;
            }
            i += 1;
            j -= 1;
        }
        true
    }
}

// impl Solution {
//     pub fn is_palindrome(s: String) -> bool { // 先处理掉不参与比较的字符
//         let mut iter = s.chars().filter(|c| c.is_alphanumeric()).map(|c| c.to_ascii_lowercase());
//         iter.clone().eq(iter.rev())
//
//         // let mut chars = s.chars().filter(|c| c.is_alphanumeric());
//         // while let (Some(c1), Some(c2)) = (chars.next(), chars.next_back()) {
//         //     if !c1.eq_ignore_ascii_case(&c2) { return false; }
//         // }
//         // true
//     }
// }

fn main() {
    assert!(!Solution::is_palindrome("race a car".to_string()));
    assert!(Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()));
}