/*!
 2. 两数相加
 地址: https://leetcode.cn/problems/add-two-numbers/
 问题: 给你两个 `非空` 的链表，表示两个非负的整数。它们每位数字都是按照 `逆序` 的方式存储的，并且每个节点只能存储 `一位` 数字。
      请你将两个数相加，并以相同形式返回一个表示和的链表
      你可以假设除了数字 0 之外，这两个数都不会以 0 开头。

 提示:
    1. 每个链表中的节点数在范围 `[1, 100]` 内
    2. 0 <= Node.val <= 9
    3. 题目数据保证列表表示的数字不含前导零

 解释:
   逆序: 链表的头结点存储的是最低位，越往后是更高的位
   如:
     [2, 4, 3] 表示的其实是数字：342（即 3×100 + 4×10 + 2×1）
     [5, 6, 4] 表示的是数字：465

  相加
      342
    + 465
    -----
      807
    2 + 5 = 7         --> 进位 carry = 0
    4 + 6 = 10        --> 存 0，carry = 1
    3 + 4 + 1 = 8     --> 存 8，carry = 0
   -> [7, 0, 8]

  步骤:
   1. 初始化：carry = 0
   2. 遍历两个链表:
      - 取出当前位 a, b，如果链表结束则当作 0
      - 计算：sum = a + b + carry
      - 新值：val = sum % 10
      - 新进位：carry = sum / 10
      - 将 val 放入结果链表
   3. 最后检查：如果 carry > 0，加一个新节点

  时间复杂度: O(max(m, n))
  空间复杂度: O(max(m, n))
*/
#[derive(Debug)]
pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

pub struct NumLink;

impl NumLink {
    pub fn create(list: Vec<i32>) -> Option<Box<ListNode>> {
        if list.len() == 0 {
            return None;
        }

        let mut head = None;
        for &val in list.iter().rev() {
            let mut node = Box::new(ListNode::new(val));
            node.next = head;
            head = Some(node);
        }

        head
    }

    // 两数相加
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() && l2.is_none() {
            return None;
        }

        if l1.is_none() {
            return l2;
        }

        if l2.is_none() {
            return l1;
        }

        let mut cur1 = l1.as_ref();
        let mut cur2 = l2.as_ref();

        let mut dummy = Box::new(ListNode::new(0)); // 哨兵节点，方便返回头部
        let mut cur_res = &mut dummy;

        let mut carry = 0; // 新进的值
        while cur1.is_some() || cur2.is_some() || carry > 0 {
            let mut a = 0;
            let mut b = 0;
            if let Some(ref c1) = cur1 {
                a = c1.val;
            }

            if let Some(ref c2) = cur2 {
                b = c2.val;
            }

            // 计算
            let sum = a + b + carry;

            // 新值
            let val = sum % 10;

            // 进位
            carry = sum / 10;

            cur_res.next = Some(Box::new(ListNode::new(val)));
            cur_res = cur_res.next.as_mut().unwrap();

            cur1 = if let Some(node) = cur1 {
                node.next.as_ref()
            } else {
                None
            };
            cur2 = if let Some(node) = cur2 {
                node.next.as_ref()
            } else {
                None
            };
        }

        // 如果 carry > 0，加一个新节点, 可以直接使用 while 循环
        /*
        if carry > 0 {
            cur_res.next = Some(Box::new(ListNode::new(carry)));
        }
         */

        dummy.next
    }
}
