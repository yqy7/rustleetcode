// 21. 合并两个有序链表
// 将两个升序链表合并为一个新的 升序 链表并返回。新链表是通过拼接给定的两个链表的所有节点组成的。 
//
// 示例 1：
//
// 输入：l1 = [1,2,4], l2 = [1,3,4]
// 输出：[1,1,2,3,4,4]
// 示例 2：
// 
// 输入：l1 = [], l2 = []
// 输出：[]
// 示例 3：
// 
// 输入：l1 = [], l2 = [0]
// 输出：[0]
//
// 提示：
// 
// 两个链表的节点数目范围是 [0, 50]
// -100 <= Node.val <= 100
// l1 和 l2 均按 非递减顺序 排列
// 
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/merge-two-sorted-lists
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

use rustleetcode::ListNode;

fn main() {
    let l1 = ListNode::fromVec(vec![1, 2, 4]);
    let l2 = ListNode::fromVec(vec![1, 3, 4]);
    println!("{:?}", merge_two_lists(l1, l2));
}

pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut p1 = l1;
    let mut p2 = l2;

    let mut head = ListNode::new(-1); // 虚拟头节点
    let mut p = &mut head;
    while p1.is_some() && p2.is_some() {
        let mut tp;
        if p1.as_ref().unwrap().val < p2.as_ref().unwrap().val {
            tp = p1;
            p1 = tp.as_mut().unwrap().next.take();
        } else {
            tp = p2;
            p2 = tp.as_mut().unwrap().next.take();
        }

        p.next = tp;
        p = p.next.as_deref_mut().unwrap();
    }

    p.next = p1.or(p2);
    head.next.take()
}

// pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//     let mut p1 = l1;
//     let mut p2 = l2;
//
//     let mut head = Some(Box::new(ListNode::new(-1))); // 虚拟头节点
//     let mut p = &mut head;
//     while p1.is_some() && p2.is_some() {
//         let mut tp;
//         if p1.as_ref().unwrap().val < p2.as_ref().unwrap().val {
//             tp = p1;
//             p1 = tp.as_mut().unwrap().next.take();
//         } else {
//             tp = p2;
//             p2 = tp.as_mut().unwrap().next.take();
//         }
//
//         p.as_mut().unwrap().next = tp;
//         p = &mut p.as_mut().unwrap().next;
//     }
//
//     p.as_mut().unwrap().next = p1.or(p2);
//     head.as_mut().unwrap().next.take()
// }


