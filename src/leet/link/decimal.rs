/*!
   1290. 二进制链表转整数
   力扣: https://leetcode.cn/problems/convert-binary-number-in-a-linked-list-to-integer/description
   题目: 给你一个单链表的引用结点 head。链表中每个结点的值不是 0 就是 1。已知此链表是一个整数数字的二进制表示形式。
        请你返回该链表所表示数字的 十进制值 。
        最高位 在链表的头部。

   思路: 每来一个新节点, 当前数字左移一位(×2), 然后加上当前位(0或1), 即: res = (res << 1) | 当前位

   举例:
       链表: 1 → 0 → 1
       res = 0
       res = res * 2 + 1 = 1
       res = res * 2 + 0 = 2
       res = res * 2 + 1 = 5

      如: 385
         3 * 100 + 8 * 10 + 5 *1
         读到 3 → res = 3
         读到 8 → res = 3 * 10 + 8 = 38
         讲到 5 → res = 38 * 10 + 5 = 385

         `乘以 10` 是为了 `给下一位腾位置`
         加上 `当前位`，是把 `新数字放到最后一位`

      同理:
         十进制:  res = res * 10 + 当前数字
         二进制:  res = res * 2 + 当前 bit
         八进制:  res = res * 8 + 当前数字
         十六进制: res = res * 16 + 当前数字

     二进制:
          (x << 1) | 0 = x << 1
          (x << 1) + 0 = x << 1
          (x << 1) | 1 = (x << 1) + 1
           二进制 `|` 和 `+` 结果相同, `|` 是 CPU 原生运算符

    时间复杂度: O(n)
    空间复杂度: O(1)
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

pub struct DecimalLink;

impl DecimalLink {
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

    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        if head.is_none() {
            return 0;
        }

        let mut res = 0;
        let mut current = head;

        while let Some(node) = current {
            res = (res << 1) | node.val; // 左移一位，加上当前 bit
            current = node.next;
        }

        res
    }
}
