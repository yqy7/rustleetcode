// 70. 爬楼梯
// 假设你正在爬楼梯。需要 n 阶你才能到达楼顶。
//
// 每次你可以爬 1 或 2 个台阶。你有多少种不同的方法可以爬到楼顶呢？
//
// 注意：给定 n 是一个正整数。
//
// 示例 1：
//
// 输入： 2
// 输出： 2
// 解释： 有两种方法可以爬到楼顶。
// 1.  1 阶 + 1 阶
// 2.  2 阶
// 示例 2：
//
// 输入： 3
// 输出： 3
// 解释： 有三种方法可以爬到楼顶。
// 1.  1 阶 + 1 阶 + 1 阶
// 2.  1 阶 + 2 阶
// 3.  2 阶 + 1 阶
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/climbing-stairs
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn main() {
    assert_eq!(climb_stairs(2), 2);
    assert_eq!(climb_stairs(3), 3);
}

// 动态规划1
// pub fn climb_stairs(n: i32) -> i32 { // f(n)表示到n为止的方法数，f(n) = f(n - 1) + f(n - 2)
//     let n = n as usize;
//     let mut dp = vec![0; n + 1];
//     dp[0] = 1;
//     dp[1] = 1;
//     for i in 2..=n {
//         dp[i] = dp[i - 1] + dp[i - 2];
//     }
//
//     dp[n]
// }

// 动态规划2
pub fn climb_stairs(n: i32) -> i32 { // 因为f(n) 只与 f(n-1) 和 f(n -2) 有关，因此可以只用两个变量保存状态
    let mut dp0 = 1;
    let mut dp1 = 1;
    for _ in 2..=n {
        let t = dp1 + dp0;
        dp0 = dp1;
        dp1 = t;
    }
    dp1
}
