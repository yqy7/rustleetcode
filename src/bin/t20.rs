// 20. 有效的括号
// 给定一个只包括 '('，')'，'{'，'}'，'['，']' 的字符串 s ，判断字符串是否有效。
//
// 有效字符串需满足：
//
// 左括号必须用相同类型的右括号闭合。
// 左括号必须以正确的顺序闭合。
//  
//
// 示例 1：
//
// 输入：s = "()"
// 输出：true
// 示例 2：
//
// 输入：s = "()[]{}"
// 输出：true
// 示例 3：
//
// 输入：s = "(]"
// 输出：false
// 示例 4：
//
// 输入：s = "([)]"
// 输出：false
// 示例 5：
//
// 输入：s = "{[]}"
// 输出：true
//  
//
// 提示：
//
// 1 <= s.length <= 104
// s 仅由括号 '()[]{}' 组成
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/valid-parentheses
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn main() {
    let s = "()";
    assert!(is_valid(s.to_string()));
    let s = "()[]{}";
    assert!(is_valid(s.to_string()));
    let s = "(]";
    assert!(!is_valid(s.to_string()));
    let s = "([)]";
    assert!(!is_valid(s.to_string()));
    let s = "{[]}";
    assert!(is_valid(s.to_string()));
}

// pub fn is_valid(s: String) -> bool {
//     fn is_pair(c_r: char, c_l: char) -> bool {
//         match c_r {
//             ')' => c_l == '(',
//             ']' => c_l == '[',
//             '}' => c_l == '{',
//             _ => false
//         }
//     }
//     let mut v: Vec<char> = Vec::new();
//     for c in s.chars() {
//         match c {
//             ')' | ']' | '}' => {
//                 match v.pop() {
//                     None => return false,
//                     Some(pop) if !is_pair(c, pop) => return false,
//                     _ => {}
//                 }
//
//             }
//             _ => v.push(c)
//         }
//     }
//     v.is_empty()
// }

// pub fn is_valid(s: String) -> bool {
//     let mut v: Vec<char> = Vec::new();
//     for c in s.chars() {
//         match c {
//             ')' | ']' | '}' => {
//                 match v.pop() {
//                     None => return false,
//                     Some(pop_value) => {
//                         if (c == ')' && pop_value != '(') || (c == ']' && pop_value != '[') || (c == '}' && pop_value != '{') {
//                             return false;
//                         }
//                     }
//                 }
//             }
//             _ => v.push(c)
//         }
//     }
//     v.is_empty()
// }

pub fn is_valid(s: String) -> bool {
    let mut v: Vec<char> = Vec::new();
    for c in s.chars() {
        match c {
            '(' | '[' | '{' => v.push(c),
            ')' if v.pop().eq(&Some('(')) =>  {}
            ']' if v.pop().eq(&Some('[')) =>  {}
            '}' if v.pop().eq(&Some('{')) =>  {}
            _ => return false,
        }
    }
    v.is_empty()
}