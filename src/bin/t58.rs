// 58. 最后一个单词的长度
// 给你一个字符串 s，由若干单词组成，单词之间用空格隔开。返回字符串中最后一个单词的长度。如果不存在最后一个单词，请返回 0 。
//
// 单词 是指仅由字母组成、不包含任何空格字符的最大子字符串。
//
//  
//
// 示例 1：
//
// 输入：s = "Hello World"
// 输出：5
// 示例 2：
//
// 输入：s = " "
// 输出：0
//  
//
// 提示：
//
// 1 <= s.length <= 104
// s 仅有英文字母和空格 ' ' 组成
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/length-of-last-word
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn main() {
    assert_eq!(length_of_last_word("Hello World".to_string()), 5);
    assert_eq!(length_of_last_word("Hello World  ".to_string()), 5);
    assert_eq!(length_of_last_word(" ".to_string()), 0);
}

// 从尾部开始遍历
pub fn length_of_last_word(s: String) -> i32 {
    s.chars()
        .rev()
        .skip_while(|&c| c == ' ')
        .take_while(|&c| c != ' ')
        .count() as i32
}

// pub fn length_of_last_word(s: String) -> i32 {
//     let mut len = 0;
//     for c in s.chars().rev(){
//         if len == 0 && c == ' ' {
//             continue;
//         } else if len != 0 && c == ' '{
//             break;
//         } else {
//             len += 1;
//         }
//     }
//     len
// }

// pub fn length_of_last_word(s: String) -> i32 {
//     let mut len = 0;
//     for c in s.chars().rev(){
//         if c != ' ' {
//             len += 1;
//         } else if len > 0 { // 隐含条件：c == ' '
//             break;
//         }
//     }
//     len
// }

// pub fn length_of_last_word(s: String) -> i32 {
//     let mut len = 0;
//     let chars = s.chars().collect::<Vec<char>>();
//     for i in (0..chars.len()).rev() {
//         if len == 0 && chars[i] == ' ' {
//             continue;
//         } else if len != 0 && chars[i] == ' '{
//             break;
//         } else {
//             len += 1;
//         }
//     }
//     len
// }

// 从头开始遍历
// pub fn length_of_last_word(s: String) -> i32 {
//     let mut res = 0;
//     let mut prev = 0;
//     for c in s.chars() {
//         if c == ' ' {
//             if prev != 0 { res = prev; }
//             prev = 0;
//         } else {
//             prev += 1;
//         }
//     }
//     if prev != 0 { res = prev; }
//     res
// }
