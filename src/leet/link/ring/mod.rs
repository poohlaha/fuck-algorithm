/*!
  环形链表:
  地址: https://leetcode.cn/problems/linked-list-cycle/description/
  问题: 给你一个链表的头节点 `head`, 判断链表中是否有环
       如果链表中有某个节点，可以通过连续跟踪 `next` 指针再次到达，则链表中存在环。
       为了表示给定链表中的环，评测系统内部使用整数 `pos` 来表示链表尾连接到链表中的位置（索引从 `0` 开始）。
       注意：`pos` 不作为参数进行传递 。仅仅是为了标识链表的实际情况
       如果链表中存在环 ，则返回 true 。 否则，返回 false
  解题思路:
    从链表头开始不断往下走 `next` 指针, 是否会无限循环?
    正常的链表最终会指向 `null`, 但如果存在某个节点的 `next` 指向前面的节点, 就会形成一个 `圈`

  解:
    1. 快最指针(Floyd 判圈算法)
       使用 `快`、`慢` 两指针:
         慢指针(slow): 每次走一步(slow = slow.next)
         快指针(fast): 每次走两步(fast = fast.next.next)
       如果链表没有环, 那第快指针会走到末尾(null) 退出;
       如果链表有环:
         快指针会有某一时刻从后面追上慢指针
       时间复杂度: O(n)(最多遍历两遍链表)
       空间复杂度: O(1)(只用了两个指针, 常数空间)
       注:
         空链表(head == null) -> 一定没有环
         只有一个节点, 且 head.next = null -> 没有环
         只有一个节点, 且 head.next = head -> 有环(自己指向自己)

    2. 哈希表记录访问节点
       用一个 `Set(集合)` 记录访问过的每一个节点
       每访问一个节点, 判断它是否已经在集合中
        是: 有环
        否: 加入集合继续
       时间复杂度: O(n)(最多遍历一遍链表)
       空间复杂度: O(n)(最坏情况下每个节点都要存入集合)
       适用场景:
         如果不能修改链表、也不在乎空间, 用此法也可以
         不如快慢指针优雅、节省空间
*/
use std::cell::RefCell;
use std::rc::Rc;

// 使用 Rc<RefCell>>
// Rc<T>: 允许多个指针指向同一个节点（引用计数）
// RefCell<T>：允许在运行时可变（内部可变性）

// 使用 Box 是独占所有权
type Link = Option<Rc<RefCell<ListNode>>>;

pub struct ListNode {
    val: i32,
    next: Link,
}

impl ListNode {
    pub fn new(val: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { val, next: None }))
    }
}

pub struct CircularLinkedList {}

impl CircularLinkedList {
    // 构建列表
    fn build(values: Vec<i32>, pos: i32) -> Link {
        let mut nodes = Vec::new();
        let mut head: Link = None;
        let mut prev: Option<Rc<RefCell<ListNode>>> = None;

        for val in values.iter() {
            let node = ListNode::new(val.clone());

            // 判断前一个节点是否有值
            // 每次对 prev.next 的修改，都会间接地影响整个链表结构
            if let Some(p) = prev {
                p.borrow_mut().next = Some(node.clone());
            } else {
                head = Some(Rc::clone(&node));
            }

            nodes.push(Rc::clone(&node));
            prev = Some(node);
        }

        // 创建环: 最后一个节点指向 pos 位置节点(力扣上最后一个节点使用箭头指向了 pos 位置节点)
        if pos > 0 && nodes.len() > 0 {
            let last = nodes.last();
            let target = &nodes[pos as usize - 1];
            if let Some(last) = last {
                last.borrow_mut().next = Some(Rc::clone(&target));
            }
        }

        head
    }

    // 判断链表是否有环（快慢指针法）
    pub fn has_cycle(values: Vec<i32>, pos: i32) -> bool {
        let head = Self::build(values, pos);

        // 1. 判断边界
        // 1.1 空链表(head == null) -> 一定没有环
        if head.is_none() {
            return false;
        }

        if let Some(head) = head.as_ref() {
            let head_ref = head.borrow();
            let next = head_ref.next.as_ref();

            // 1.2 只有一个节点, 且 head.next = null -> 没有环
            if next.is_none() {
                return false;
            }

            // 1.3 只有一个节点, 且 head.next = head -> 有环(自己指向自己)
            if let Some(next) = next {
                if Rc::ptr_eq(next, head) {
                    return true;
                }
            }
        }

        // 2. 定义 `快`、`慢` 两指针
        let mut slow = head.clone();
        let mut fast = head.clone();

        while let (Some(s), Some(f)) = (slow.clone(), fast.clone()) {
            // 2.1
            // 慢指针(slow): 每次走一步(slow = slow.next)
            slow = s.borrow().next.clone();

            // 快指针(fast): 每次走两步(fast = fast.next.next)
            let f_ref = f.borrow();
            let f_next = f_ref.next.as_ref();
            if let Some(f_next) = f_next {
                fast = f_next.borrow().next.clone();
            } else {
                return false;
            }

            // 如果有环, 快指针会有某一时刻从后面追上慢指针
            if let (Some(s), Some(f)) = (&slow, &fast) {
                if Rc::ptr_eq(s, f) {
                    return true;
                }
            }
        }

        false
    }
}
