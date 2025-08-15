/*!
  19. 删除链表的倒数第 N 个节点
  力扣: https://leetcode.cn/problems/remove-nth-node-from-end-of-list/
  题目: 给你一个链表, 删除链表的倒数第 n 个结点, 并且返回链表的头结点
  示例:
      1 -> 2 -> 3 -> 4 -> 5
                |
      1 -> 2 -> 3 - ----> 5

  解: 使用双指针(快慢指针), 让快指针先走 n 步, 再同时移动快慢指针, 当快指针指向末尾 null 时, 慢指针为要删除的节点
*/
pub struct Link;

#[derive(Debug, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

impl Link {
    pub fn create_node(arr: Vec<i32>) -> Option<Box<ListNode>> {
        if arr.len() == 0 {
            return None;
        }

        let mut dummy = Box::new(ListNode::new(0));
        let mut head = &mut dummy;

        for &i in arr.iter() {
            head.next = Some(Box::new(ListNode::new(i)));
            head = head.next.as_mut().unwrap();
        }

        dummy.next
    }

    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let mut dummy = Box::new(ListNode {
            val: 0,
            next: head.clone(),
        });
        let mut remove = dummy.as_mut();

        let mut fast = &head;
        let mut slow = &head;

        // 快指针先走 n 步
        for _ in 0..n {
            if let Some(f) = fast.as_ref() {
                fast = &f.next;
            }
        }

        // 快慢指针一起走，直到 fast 到末尾
        while fast.is_some() {
            remove = remove.next.as_mut().unwrap();

            if let Some(f) = fast.as_ref() {
                fast = &f.next;
            }

            if let Some(s) = slow.as_ref() {
                slow = &s.next;
            }
        }

        // 此时 remove.next 是要删除的节点
        remove.next = remove.next.as_mut().unwrap().next.take();

        dummy.next
    }

    /**
      21. 合并两个有序链表
      力扣: https://leetcode.cn/problems/merge-two-sorted-lists/description/
      题目: 将两个升序链表合并为一个新的 升序 链表并返回。新链表是通过拼接给定的两个链表的所有节点组成的。
      示例:
           1 ---> 2 ---> 3

           1 ---> 3 ----> 4

           1 ---> 1 ---> 2 ---> 3 ---> 4 ---> 4
      解:
        使用双指针

        时间复杂度: O(m + n)
        空间复杂度: O(m + n)
    */
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if list1.is_none() && list2.is_none() {
            return None;
        }

        if list1.is_none() {
            return list2;
        }

        if list2.is_none() {
            return list1;
        }

        let mut dummy = Box::new(ListNode::new(0));
        let mut p = &mut dummy;
        let mut p1 = list1.as_ref();
        let mut p2 = list2.as_ref();

        while let (Some(n1), Some(n2)) = (p1, p2) {
            if n1.val < n2.val {
                p.next = Some(Box::new(ListNode::new(n1.val)));
                p = p.next.as_mut().unwrap();
                p1 = n1.next.as_ref();
            } else {
                p.next = Some(Box::new(ListNode::new(n2.val)));
                p = p.next.as_mut().unwrap();
                p2 = n2.next.as_ref();
            }
        }

        // 如果 list1 还有节点
        while let Some(n1) = p1 {
            p.next = Some(Box::new(ListNode::new(n1.val)));
            p = p.next.as_mut().unwrap();
            p1 = n1.next.as_ref();
        }

        // 如果 list2 还有节点
        while let Some(n2) = p2 {
            p.next = Some(Box::new(ListNode::new(n2.val)));
            p = p.next.as_mut().unwrap();
            p2 = n2.next.as_ref();
        }

        dummy.next
    }
}
