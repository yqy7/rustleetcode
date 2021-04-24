// 83. 删除排序链表中的重复元素
// 存在一个按升序排列的链表，给你这个链表的头节点 head ，请你删除所有重复的元素，使每个元素 只出现一次 。
//
// 返回同样按升序排列的结果链表。
//
//  
//
// 示例 1：
//
//
// 输入：head = [1,1,2]
// 输出：[1,2]
// 示例 2：
//
//
// 输入：head = [1,1,2,3,3]
// 输出：[1,2,3]
//  
//
// 提示：
//
// 链表中节点数目在范围 [0, 300] 内
// -100 <= Node.val <= 100
// 题目数据保证链表已经按升序排列
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/remove-duplicates-from-sorted-list
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

use rustleetcode::ListNode;
use std::option::Option::Some;
use std::borrow::{BorrowMut, Borrow};

fn main() {
    println!("{:?}", delete_duplicates(ListNode::fromVec(vec![1, 1, 1])));
    println!("{:?}", delete_duplicates(ListNode::fromVec(vec![1, 1, 2])));
    println!("{:?}", delete_duplicates(ListNode::fromVec(vec![1,1,2,3,3])));
}

pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut p = &mut head;
    while let Some(node) = p {
        while node.next.is_some()
            && node.val == node.next.as_ref().unwrap().val {
            let next = node.next.take();
            node.next = next.unwrap().next.take();
        }
        p = &mut node.next;
    }

    head
}

// pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//     let mut res = ListNode {val: 0, next: head };
//
//     let mut p = &mut res.next;
//     while let Some(node) = p {
//         while node.next.is_some()
//             && node.val == node.next.as_ref().unwrap().val {
//             let next = node.next.take();
//             node.next = next.unwrap().next.take();
//         }
//         p = &mut node.next;
//     }
//
//     res.next.take()
// }
