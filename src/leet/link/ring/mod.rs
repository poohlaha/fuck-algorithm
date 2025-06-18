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

/*!
环形链表II:
地址: https://leetcode.cn/problems/linked-list-cycle-ii/description/
问题: 给你一个链表的头节点 `head`, 判断链表中是否有环
     给定一个链表的头节点  head ，返回链表开始入环的第一个节点。 如果链表无环，则返回 null
     如果链表中有某个节点，可以通过连续跟踪 next 指针再次到达，则链表中存在环。
     为了表示给定链表中的环，评测系统内部使用整数 pos 来表示链表尾连接到链表中的位置（索引从 0 开始）
     如果 pos 是 -1，则在该链表中没有环
     注意：pos 不作为参数进行传递，仅仅是为了标识链表的实际情况。
     不允许修改 链表。
解题思路:
    1. 判断是否有环
       快指针走两步, 慢指针走一步
       如果他们相遇, 说明有环;否则走到 None 就没环
    2. 找到环的起点
       当快慢指针相遇后:
       再用一个指针从头开始走, 走 a 步
       另一个指针从相遇点继续走, 走 nr - b 步(也等于绕回环起点
       两个指针每次走一步, 他们将在入环点相遇

 解:
   链表结构如下：
      head → [非环区域] a 步 →  ⭘环区域（长度 r）⭕
                       ↑           ↓
                        ←←←←←←←←←←←←
   假设:
     a: 从链表头部 head 到环的起点的步数(非环部分)
     b: 从环起点到快慢指针第一次 `相遇点` 的步数(在环内)
     r: 环的长度(一个完整的环有 r  个节点)
     n: 快指针在环内多绕了几圈
     快慢指针第一次在环中相遇的位置，一定是快指针绕了至少一整圈之后才追上慢指针的

   1. 慢指针走的路程
      慢指针从 head 出发
      经过非环部分 a 步
      然后进入环, 走了 b 步到达相遇点
   所以慢指针走的总步数是:
      慢 = a + b

   2. 快指针走的路程
      快指针从 head 出发
      经过非环部分 a 步
      然后进入环, 它在环里可能绕了好几圈

      快指针比慢指针快, 每轮多走 2 步(快 2 步, 慢 1 步)
      所以它追上慢指针时, 总共走了 2(a + b) 步
   最终:
      走完非环 a 步 + 在环里走 b 步 + n圈环路(n * r 步)
   所以快指针的总则步数是:
      快 = a + b + n * r

   3. 列出等式
      快 = 2 * 慢
      a + b + n * r = 2(a + b)
   -> n * r = a + b
   -> a = n * r - b

   最终:
      慢指针走了: a + b
      快指针走了: a + b + n * r

   根据公式推导:
      a = n * r - b
   -> a + b = n * r
   -> a = r - b (mod r)
   链表头从 `a` 步到入环点, 然后再走 `b` 步(从头到相遇点的路径), 刚好等于绕环 `n` 圈
   所以:
     从相遇点走 `a` 步就正好回到了环起点
   这时:
     从链表头出发的指针走了 `a` 步
     从相遇点的指针走了 `r - b`  步
*/

use std::cell::RefCell;
use std::rc::Rc;

// 使用 Rc<RefCell>>
// Rc<T>: 允许多个指针指向同一个节点（引用计数）
// RefCell<T>：允许在运行时可变（内部可变性）

// 使用 Box 是独占所有权

type Link = Option<Rc<RefCell<ListNode>>>;

#[derive(Debug)]
pub struct ListNode {
    val: i32,
    next: Option<Rc<RefCell<ListNode>>>,
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

    // 获取环的起点
    pub fn detect_cycle(values: Vec<i32>, pos: i32) -> Link {
        let head = Self::build(values, pos);

        // 1. 判断边界
        // 1.1 空链表(head == null) -> 一定没有环
        if head.is_none() {
            return None;
        }

        if let Some(head) = head.as_ref() {
            let head_ref = head.borrow();
            let next = head_ref.next.as_ref();

            // 1.2 只有一个节点, 且 head.next = null -> 没有环
            if next.is_none() {
                return None;
            }

            // 1.3 只有一个节点, 且 head.next = head -> 有环(自己指向自己)
            if let Some(next) = next {
                if Rc::ptr_eq(next, head) {
                    return Some(next.clone());
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
                return None;
            }

            // 如果有环, 快指针会有某一时刻从后面追上慢指针
            // 再用一个指针从头开始走
            // 另一个指针从相遇点继续走
            // 两个指针每次走一步, 他们将在入环点相遇
            if let (Some(s), Some(f)) = (&slow, &fast) {
                // 再用一个指针从头开始走, 走 a 步
                let mut p1 = head.clone();
                // 另一个指针从相遇点继续走, 走 nr - b 步
                let mut p2 = slow.clone(); // 或 fast.clone()

                // 两个指针每次走一步, 他们将在入环点相遇
                while let (Some(a), Some(b)) = (&p1, &p2) {
                    if Rc::ptr_eq(a, b) {
                        return Some(a.clone());
                    }

                    p1 = a.borrow().next.clone();
                    p2 = b.borrow().next.clone();
                }
            }
        }

        None
    }
}
