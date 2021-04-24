// 69. x 的平方根
// 实现 int sqrt(int x) 函数。
//
// 计算并返回 x 的平方根，其中 x 是非负整数。
//
// 由于返回类型是整数，结果只保留整数的部分，小数部分将被舍去。
//
// 示例 1:
//
// 输入: 4
// 输出: 2
// 示例 2:
//
// 输入: 8
// 输出: 2
// 说明: 8 的平方根是 2.82842...,
//      由于返回类型是整数，小数部分将被舍去。
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/sqrtx
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn main() {
    assert_eq!(my_sqrt(4), 2);
    assert_eq!(my_sqrt(8), 2);
    assert_eq!(my_sqrt(1), 1);
    assert_eq!(my_sqrt(2), 1);
    assert_eq!(my_sqrt(3), 1);
    assert_eq!(my_sqrt(5), 2);
    println!("{}", my_sqrt(2147395599)); // 46339
    println!("{}", my_sqrt(2147395600)); // 46340
    println!("{}", my_sqrt(2147483647)); // 46340
}

// pub fn my_sqrt(x: i32) -> i32 {
//     f64::sqrt(x as f64) as i32
// }

// 二分查找
pub fn my_sqrt(x: i32) -> i32 {
    if x ==0 || x == 1 { return x; }

    let mut l = 1;
    let mut r = x;
    while l < r {
        let mid = l + (r - l) / 2; // 先减后加防止溢出

        match x / mid { // 用除法防止溢出
            q if q > mid => l = mid + 1,
            q if q < mid => r = mid,
            _ => return mid
        }
    }
    l - 1
}

// pub fn my_sqrt(x: i32) -> i32 {
//     if x ==0 || x == 1 { return x; }
//
//     let mut l = 1;
//     let mut r = x;
//     while l < r {
//         let mid = (l.wrapping_add(r) as u32 >> 1) as i32; // i32类型会溢出，因此用wrapping_add。又因为没有无符号右移，需要先转u32再转回来
//         if i32::MAX / mid < mid { // 溢出
//             r = mid;
//             continue;
//         }
//
//         let sqrt = mid * mid;
//         if sqrt == x {
//             return mid;
//         } else if sqrt > x {
//             r = mid;
//         } else {
//             l = mid + 1;
//         }
//     }
//     l - 1
// }

// pub fn my_sqrt(x: i32) -> i32 {
//     assert!(x >= 0); // 排除掉负数
//     if x ==0 || x == 1 { return x; }
//
//     let x = x as u32;
//     let mut l = 1;
//     let mut r = x;
//     while l < r {
//         let mid = (l + r) >> 1;
//         if i32::MAX as u32 / mid < mid { // 溢出
//             r = mid;
//             continue;
//         }
//
//         let sqrt = mid * mid;
//         if sqrt == x {
//             return mid as i32;
//         } else if sqrt > x {
//             r = mid;
//         } else {
//             l = mid + 1;
//         }
//     }
//     (l - 1) as i32
// }

// 算数法：「袖珍计算器算法」是一种用指数函数 exp 和对数函数 ln 代替平方根函数的方法。
// x^0.5 = (e^ln(x))^0.5 = e^(0.5*ln(x))
// pub fn my_sqrt(x: i32) -> i32 {
//     let xf = x as f64;
//     let a = (0.5 * xf.ln()).exp() as i64;
//     let res =  if (a + 1) * (a + 1) <= x as i64 { a + 1 } else { a }; // a + 1处理误差
//     res as i32
// }


// 牛顿迭代法
// x1 = x0 - (f(x0) / f`(x0))，f(x0) = x^2 - C => x1 = 1/2(x0 + C/x0)，当x1和x0很接近的时候，< 1e-6/1e-7，可以认为就是x0
// pub fn my_sqrt(x: i32) -> i32 {
//     if x == 0 {
//         return 0;
//     }
//
//     let mut x0 = x as f64;
//     let mut C = x as f64;
//     while true {
//         let x1 = 0.5 * (x0 + C / x0);
//         if (x0-x1).abs() < 1e-7 {
//             break;
//         }
//         x0 = x1;
//     }
//     x0 as i32
// }
