// 14. 最长公共前缀
// 编写一个函数来查找字符串数组中的最长公共前缀。
// 
// 如果不存在公共前缀，返回空字符串 ""。
//
// 示例 1：
// 
// 输入：strs = ["flower","flow","flight"]
// 输出："fl"
// 示例 2：
// 
// 输入：strs = ["dog","racecar","car"]
// 输出：""
// 解释：输入不存在公共前缀。
//  
// 
// 提示：
// 
// 0 <= strs.length <= 200
// 0 <= strs[i].length <= 200
// strs[i] 仅由小写英文字母组成
// 
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/longest-common-prefix
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn main() {
    let strs = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
    println!("{}", longest_common_prefix(strs));
    let strs = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
    println!("{}", longest_common_prefix(strs));
    let strs = vec!["dog".to_string()];
    println!("{}", longest_common_prefix(strs));
}

// 纵向扫描
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut res = String::new();
    if strs.len() == 0 { return res; }

    let strList:Vec<Vec<char>> = strs.iter().map(|s| s.chars().collect() ).collect();

    for i in 0..strList[0].len() {
        let c = strList[0][i];
        for j in 1..strList.len() {
            if i >= strList[j].len() || strList[j][i] != c {
                return res;
            }
        }
        res.push(c);
    }

    res
}






