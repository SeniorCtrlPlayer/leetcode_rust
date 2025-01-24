/*
 * @lc app=leetcode id=21 lang=rust
 *
 * [21] Merge Two Sorted Lists
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // println!("test");
        // let mut list1_ = list1; // 所有权转移，为什么要转移
        // let mut list2_ = list2; // 同上
        let mut head = None;   // 创建一个None为list3，为什么创建的不是Some(None)
        let mut next = &mut head; // 创建了一个list3的引用，由此可看出，仅通等号左边的变量声明是无法判断该变量是否为引用的。
        loop {
            // // 创建一个新的node1，Box::new一般代表在栈上申请，返回的是一个Box<ListNode>
            // let mut node1 = Box::new(ListNode {
            //     val: 0,
            //     next: None,
            // });

            // // 其中list1或者list1_，完整的类型应该写为 Option<Some(Box<ListNode>)>
            // match list1_ {
            //     // 匹配时，解开Some后，l1 应该为 Box<ListNode>, 将node1指向list1_的表头。
            //     Some(l1) => {
            //         node1 = l1;
            //     },
            //     _ => { // 如果不为Some，即None
            //         *next = list2_; // 将list4所引用的变量，即list3，修改为list2_ (list2_的所有权应该会转移到list3上)
            //         break; // 因为break打破了loop，所以不会走到下面的 match list2_，所以不会有value moved的问题
            //     },
            // };

            // let mut node2 = Box::new(ListNode{
            //     val: 0,
            //     next: None,
            // });

            // match list2_ {
            //     Some(l2) => {
            //         node2 = l2;
            //     },
            //     _ => {
            //         *next = Some(node1);
            //         break;
            //     },
            // }

            let Some(mut node1) = list1 else {
                *next = list2;
                break;
            };

            let Some(mut node2) = list2 else {
                *next = Some(node1);
                break;
            };

            if node1.val < node2.val {
                list1 = node1.next.take();
                list2 = Some(node2);
                *next = Some(node1);
            } else {
                list2 = node2.next.take();
                list1 = Some(node1);
                *next = Some(node2);
            }
            next = &mut next.as_mut().unwrap().next;
            assert!(next.is_none());
        }
        head
    }
}
// @lc code=end

