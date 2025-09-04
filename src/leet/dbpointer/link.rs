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

use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub struct Link;

#[derive(Eq, Ord, PartialEq, PartialOrd, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

// 包装节点, 手动实现 Ord
#[derive(Eq, PartialEq)]
struct HeapNode(Box<ListNode>);

// 完全可比较, 每一对值都必须能比较出大小
impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // BinaryHeap 默认是最大堆，所以这里反转顺序
        other.0.val.cmp(&self.0.val)
    }
}

// 部分可比较, 可能有些值没法比较（例如 f64::NAN 和别的数）
impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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

/**
  23. 合并 K 个升序链表
  力扣: https://leetcode.cn/problems/merge-k-sorted-lists/description/
  题目: 给你一个链表数组，每个链表都已经按升序排列。
       请你将所有链表合并到一个升序链表中，返回合并后的链表。

  解: 使用最小堆把头节点全放进去
  时间复杂度: O(n log k)
  空间复杂度: O(k)
*/
pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    if lists.is_empty() {
        return None;
    }

    let mut heap = BinaryHeap::new();

    // 把每个链表的头节点加入最小堆
    for list in lists {
        if let Some(node) = list {
            // 需要包装 Reverse，使其变成最小堆
            // std::cmp::Reverse，它会反转比较顺序
            // heap.push(Reverse(node));
            heap.push(HeapNode(node));
        }
    }

    let mut dummy = Box::new(ListNode::new(0));
    let mut tail = &mut dummy;

    // while let Some(Reverse(mut node)) = heap.pop() {
    while let Some(HeapNode(mut node)) = heap.pop() {
        // 把最小的节点接到结果链表
        tail.next = Some(Box::new(ListNode::new(node.val)));

        if let Some(next) = node.next.take() {
            // heap.push(Reverse(next));
            heap.push(HeapNode(next));
        }

        // tail 指针不断前进
        tail = tail.next.as_mut().unwrap();
    }

    dummy.next
}

/**
   24. 两两交换链表中的节点
   题目: 给你一个链表，两两交换其中相邻的节点，并返回交换后链表的头节点。你必须在不修改节点内部的值的情况下完成本题（即，只能进行节点交换）。
   力扣: https://leetcode.cn/problems/swap-nodes-in-pairs/description/

   解:
   维护三个指针 prev, a, b, prev -> a -> b, 每次交换 a 和 b, 交换完成后移动 prev 到 a(此时 a 和 b 已交换, prev 直接放到原来 b 的位置), 继续下一轮

   例:
     设链表: 1 -> 2 -> 3 -> 4 -> 5
     变量: dummy(虚拟头节点)、prev(prev = dummy)
     临时变量: a、b

     1. 第一轮
        prev -> 1 -> 2
        a = prev.next = 1
        b = a.next = 2

       交换(a, b), 即(1, 2)
       把 prev 的 next 把向尾部节点 b(2), 这样第一次 dummy.next = b = 2
       prev.next = b = 2
       a.next = b.next = 3
       b.next = a = 1
       交换完成后把 prev 指向 a: prev = a
       此时链表为: dummy -> 2 -> 1 -> prev -> 3 -> 4 -> 5

    2. 第二轮
       prev -> 3 -> 4
       a = prev.next = 3
       b = a.next = 4

       交换(a, b), 即(3, 4)
       把 prev 的 next 把向尾部节点 b(4)
       prev.next = b = 4
       a.next = b.next = 5
       b.next = a = 3
       交换完成后把 prev 指向 a: prev = a
       此时链表为: dummy -> 2 -> 1 -> 4 -> 3 -> prev -> 5

     3. 第三轮
        prev -> 5
        a = prev.next = 5
        b = a.next = null
        b 结束为空, 循环结束

     最终取 dummy.next, 链表为: dummy -> 2 -> 1 -> 4 -> 3 -> 5

   时间复杂度: O(n)
   空间复杂度: O(1)
*/
pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }

    // 创建虚拟头节点
    let mut dummy = Box::new(ListNode { val: 0, next: head });

    // prev 指针，用于处理每一对的前驱节点
    let mut prev = &mut dummy;

    while prev.next.is_some() && prev.next.is_some() && prev.next.as_ref().unwrap().next.is_some() {
        // a = prev.next
        let mut a = prev.next.take().unwrap();
        // b = a.next
        let mut b = a.next.take().unwrap();

        // 交换(a, b)
        // a.next = b.next
        a.next = b.next.take();

        // b.next = a
        b.next = Some(a);

        // prev.next = b
        prev.next = Some(b);

        // prev 前进到 a (即交换后的第二个节点)
        prev = prev.next.as_mut().unwrap().next.as_mut().unwrap();
    }

    dummy.next
}

/**
   25. K 个一组翻转链表
   题目: 给你链表的头节点 head ，每 k 个节点一组进行翻转，请你返回修改后的链表。
        k 是一个正整数，它的值小于或等于链表的长度。如果节点总数不是 k 的整数倍，那么请将最后剩余的节点保持原有顺序。
        你不能只是单纯的改变节点内部的值，而是需要实际进行节点交换。
   解:
       1. 两两翻转: 每次处理 2 个节点, 改 3 条边
       2. k 组翻转: 每次处理 k 个节点, 要翻转整个区间 [a .. b)(左闭右开)

   例:
     1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 7, k = 3(逐步)
     变量: dummy(虚拟头节点)、prev(prev = dummy)
     临时变量: tail, a, b, cur, prev_local

    1. 第一轮
       检查并取出 3 个节点
       从 prev 向前走 3 步找到 tail
       tail = 3(组尾)
       a = prev.next = 1(组头)
       b = tail.next = 4(组尾, 右边界, 不翻转)

       翻转[1, 4), 即: 1, 2, 3: 3 -> 2 -> 1
       过程:
            1. 把 3 放到最前面
            2. 尾节点 1 指向 b = 4

            prev.next = tail // 把前面部分连接到翻转后的组头
            a.next = b // 翻转后的尾连接到组外
            prev = a // 移动 prev 到翻转后的尾，为下一轮翻转做准备

       此时链表为: dummy -> 3 -> 2 -> 1(prev) -> 4 -> 5 -> 6 -> 7

     2. 第二轮
        检查并取出 3 个节点
        从 prev 向前走 3 步找到 tail
        tail = 6(组尾)
        a = prev.next = 4(组头)
        b = tail.next = 7(组尾, 右边界, 不翻转)

        翻转[4, 7), 即: 4, 5, 6: 6 -> 5 -> 4
        过程:
            1. 把 6 放到最前面
            2. 尾节点 4 指向 b = 7

        prev.next = tail // 把前面部分连接到翻转后的组头
        a.next = b // 翻转后的尾连接到组外
        prev = a // 移动 prev 到翻转后的尾，为下一轮翻转做准备

        此时链表为: dummy -> 3 -> 2 -> 1 -> 6 -> 5 -> 4(prev) -> 7

     3. 第三轮
        检查并取出 3 个节点
        从 prev 向前走 3 步未找到 tail
        循环结束

      最终取 dummy.next, 链表为: dummy -> 3 -> 2 -> 1 -> 6 -> 5 -> 4 -> 7

      反转过程:
              1 -> 2 -> 3 反转, 右边界 end = 4
              变量: prev = 4(end), head = 1
              临时变量: node, cur

              第一步:
              1. 取出 node = 1
              2. 把 node.next = prev = 4, 得到 1 -> 4
              3. 把前一个节点指向 node, prev = node = 1
              4. 最终: prev -> 1 -> 4
              5. cur 向前走一步, cur = node.next = 2

              第二步:
              1. 取出 node = 2
              2. 把 node.next = prev = 1
              3. 把前一个节点指向 node, prev = node = 2
              4. 最终: prev -> 2 -> 1 -> 4
              5. cur 向前走一步, cur = node.next = 3

              第三步:
              1. 取出 node = 3
              2. 把 node.next = prev = 2
              3. 把前一个节点指向 node, prev = node = 3
              4. 最终: prev -> 3 -> 2 -> 1 -> 4
              5. cur 向前走一步, cur = node.next = 4

              第四步:
              cur = end, 结束循环, cur 置为空

      时间复杂度: O(n)
      空间复杂度: O(1)
*/
pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    if head.is_none() || k <= 0 {
        return None;
    }

    if k == 1 {
        return head;
    }

    let mut dummy = Box::new(ListNode { val: 0, next: head });
    let mut prev = &mut dummy;
    loop {
        // 1. 检查是否有 k 个节点
        let mut tail = prev.as_mut();
        for _ in 0..k {
            if let Some(ref mut next) = tail.next {
                tail = next;
            } else {
                return dummy.next; // 剩余不足 k 个，直接结束
            }
        }


        // 2. 定义区间[a, b), a = prev.next, b = tail.next
        let b = tail.next.take(); // 区间的右边界，不参与翻转
        let a = prev.next.take();

        // 3. 翻转 [a, b)
        let new_head = reverse_segment(a, b);

        // 4. 接回链表
        prev.next = new_head;

        // 移动 prev 到本组的尾(翻转前的 head)
        for _ in 0..k {
            if let Some(ref mut next) = prev.next {
                prev = next;
            }
        }
    }
}

/// 翻转区间 [head, end)，返回 (新的头, 新的尾)
fn reverse_segment(
    mut head: Option<Box<ListNode>>,
    end: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    // end: 翻转区间的右边界(不包含在翻转范围内)
    // head: 翻转区间的起点
    let mut prev = end;

    // 反转区间 [head .. enc)
    while let Some(mut node) = head {
        head = node.next.take();
        node.next = prev;
        prev = Some(node);
    }

    prev
}
