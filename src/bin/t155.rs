// 155. 最小栈
// 设计一个支持 push ，pop ，top 操作，并能在常数时间内检索到最小元素的栈。
// 
// push(x) —— 将元素 x 推入栈中。
// pop() —— 删除栈顶的元素。
// top() —— 获取栈顶元素。
// getMin() —— 检索栈中的最小元素。
//  
// 
// 示例:
// 
// 输入：
// ["MinStack","push","push","push","getMin","pop","top","getMin"]
// [[],[-2],[0],[-3],[],[],[],[]]
// 
// 输出：
// [null,null,null,null,-3,null,0,-2]
// 
// 解释：
// MinStack minStack = new MinStack();
// minStack.push(-2);
// minStack.push(0);
// minStack.push(-3);
// minStack.getMin();   --> 返回 -3.
// minStack.pop();
// minStack.top();      --> 返回 0.
// minStack.getMin();   --> 返回 -2.
//  
// 
// 提示：
// 
// pop、top 和 getMin 操作总是在 非空栈 上调用。
// 
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/min-stack
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

fn main() {
    // let mut minStack = MinStack::new();
    // minStack.push(-2);
    // minStack.push(0);
    // minStack.push(-3);
    // println!("{}", minStack.get_min());   // -3.
    // minStack.pop();
    // println!("{}", minStack.top());      // 0.
    // println!("{}", minStack.get_min());   // -2.

    // [null,null,null,null,2147483647,null,2147483646,null,2147483646,null,null,2147483647,2147483647,null,-2147483648,2147483647,null,2147483647]
    let mut minStack = MinStack::new();
    minStack.push(2147483647);
    minStack.push(-2147483648);
    // minStack.push(-3);
    println!("{}", minStack.get_min());   // -3.
    minStack.pop();
    println!("{}", minStack.top());      // 0.
    println!("{}", minStack.get_min());   // -2.
}

// 方法一：辅助栈
// struct MinStack {
//     stack: Vec<i32>,
//     minStack: Vec<i32>
// }
//
// impl MinStack {
//
//     /** initialize your data structure here. */
//     fn new() -> Self {
//         MinStack {
//             stack: vec![],
//             minStack: vec![],
//         }
//     }
//
//     fn push(&mut self, val: i32) {
//         self.stack.push(val);
//         if self.minStack.is_empty() || *self.minStack.last().unwrap() > val {
//             self.minStack.push(val);
//         } else {
//             self.minStack.push(*self.minStack.last().unwrap());
//         }
//     }
//
//     fn pop(&mut self) {
//         self.stack.pop();
//         self.minStack.pop();
//     }
//
//     fn top(&self) -> i32 {
//         let top = *self.stack.last().unwrap();
//         return top;
//     }
//
//     fn get_min(&self) -> i32 {
//         let min = *self.minStack.last().unwrap();
//         return min;
//     }
// }

// 方法二：栈保存差值
// struct MinStack {
//     stack: Vec<i64>, // 需要注意溢出的情况，因此用i64保存
//     min: i64,
// }
//
// impl MinStack {
//
//     /** initialize your data structure here. */
//     fn new() -> Self {
//         MinStack {
//             stack: vec![],
//             min: 0,
//         }
//     }
//     // 如果val >= min，则val-min>=0，如果val < min，则val - min < 0，这种情况要修改min为val。
//     // 通过val-min可知val和min的关系，可以还原val或min。
//     fn push(&mut self, val: i32) {
//         if self.stack.is_empty() {
//             self.min = val as i64;
//             self.stack.push(0);
//         } else {
//             let diff = val as i64 - self.min;
//             self.stack.push(diff);
//             if diff < 0 {
//                 self.min = val as i64;
//             }
//         }
//     }
//
//     fn pop(&mut self) {
//         let diff = self.stack.pop().unwrap();
//         if diff < 0 {
//             self.min = self.min - diff;
//         }
//     }
//
//     fn top(&self) -> i32 {
//         let diff = *self.stack.last().unwrap();
//         if diff < 0 {
//             return self.min as i32;
//         } else {
//             return (diff + self.min) as i32;
//         }
//     }
//
//     fn get_min(&self) -> i32 {
//         self.min as i32
//     }
// }

// 方法三：stack多存放一个min，如果val<=min，押入min0、val，min=val；如果val > min，押入val。
// 弹出时，如果min == top，则说明之前多压了一个min，需要pop出来
struct MinStack {
    stack: Vec<i32>,
    min: i32,
}

impl MinStack {

    /** initialize your data structure here. */
    fn new() -> Self {
        MinStack {
            stack: vec![],
            min: i32::MAX, // 为了在第一次比较的时候能正确
        }
    }

    fn push(&mut self, val: i32) {
        if val <= self.min {
            self.stack.push(self.min);
            self.min = val;
        }
        self.stack.push(val);
    }

    fn pop(&mut self) {
        let top = self.stack.pop().unwrap();
        if top == self.min {
            self.min = self.stack.pop().unwrap();
        }
    }

    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        self.min
    }
}