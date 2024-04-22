//! 单向链表

use std::cmp::Ordering;
use std::fmt::Debug;
use crate::tree::binary_heap::{BinaryHeap, BinaryMinHeap};

#[derive(Clone, Debug)]
pub struct ListNode<E> {
    pub(crate) element: E,
    pub(crate) next: Option<Box<ListNode<E>>>,
}

impl<E> ListNode<E> {
    pub fn new(element: E) -> Self {
        Self {
            element,
            next: None,
        }
    }
}

impl<E: PartialOrd> PartialOrd for ListNode<E> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.element.partial_cmp(&other.element)
    }
}

impl<T: PartialEq> PartialEq for ListNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.element == other.element
    }
}

/// 根据数组创建链表
pub(crate) fn create<E>(arr: Vec<E>) -> Option<Box<ListNode<E>>>
where
    E: Clone,
{
    if arr.is_empty() {
        return None;
    }

    let first = arr.get(0);
    if let Some(first) = first {
        let mut head = Some(Box::new(ListNode::<E>::new(first.clone())));
        let mut cur = &mut head;
        for x in arr.iter().skip(1) {
            let new_node = Some(Box::new(ListNode::new(x.clone())));
            if let Some(ref mut node) = cur {
                node.next = new_node;
                cur = &mut node.next;
            }
        }

        return head;
    }

    return None;
}

/// 打印链表
pub(crate) fn print<E>(head: Option<Box<ListNode<E>>>)
where
    E: Debug,
{
    let mut current = head;
    while let Some(node) = current {
        print!("{:#?} ", node.element);
        current = node.next
    }
}

/// 分隔链表
pub(crate) fn partition(v: Vec<u32>, x: u32) -> Option<Box<ListNode<u32>>> {
    let mut dummy_head1 = ListNode::<u32>::new(0); // 存放小于 x 的链表的虚拟头结点
    let mut dummy_head2 = ListNode::<u32>::new(0); // 存放大于等于 x 的链表的虚拟头结点
    let mut p1 = &mut dummy_head1;
    let mut p2 = &mut dummy_head2;
    let mut p = create(v); // p 负责遍历原链表，类似合并两个有序链表的逻辑

    while p.is_some() {
        let element = p.as_ref().unwrap().element;
        if element < x {
            p1.next = p.clone();
            p1 = p1.next.as_mut().unwrap().as_mut();
        } else {
            p2.next = p.clone();
            p2 = p2.next.as_mut().unwrap().as_mut();
        }

        // 不能直接让 p 指针前进
        // p = p.next
        // 断开原链表中的每个节点的 next 指针
        let next = p.as_mut().unwrap().next.take();
        p = next
    }

    // 连接两个链表
    // 将原链表的尾部节点的 next 指针置为 None
    p1.next = None;
    p2.next = None;

    p1.next = dummy_head2.next;
    dummy_head1.next
}

/// 合并两个有序链表
pub(crate) fn merge_two_list(v1: Vec<u32>, v2: Vec<u32>) -> Option<Box<ListNode<u32>>> {
    // 创建 `虚拟头节点`
    let mut dummy_head = ListNode::<u32>::new(0);
    let mut current = &mut dummy_head;
    let mut p1 = create(v1);
    let mut p2 = create(v2);

    while p1.is_some() && p2.is_some() {
        let element1 = p1.as_ref().unwrap().element;
        let element2 = p2.as_ref().unwrap().element;
        if element1 > element2 {
            current.next = p2.clone();
            p2 = p2.unwrap().next
        } else {
            current.next = p1.clone();
            p1 = p1.unwrap().next
        }

        // 移动当前指针, 不断前进
        current = current.next.as_mut().unwrap()
    }

    if p1.is_some() {
        current.next = p1
    }

    if p2.is_some() {
        current.next = p2
    }

    dummy_head.next
}

/// 合并 K 个有序链表
/**
- 创建一个最小堆，用于存储每个链表的头节点。
- 将每个链表的头节点插入到最小堆中。
- 从最小堆中弹出堆顶节点（即当前最小的节点），将其添加到结果链表中。
- 如果弹出的节点有下一个节点，则将下一个节点插入到最小堆中，以保持堆的性质。
- 重复步骤 3 和步骤 4，直到最小堆为空。
 **/
pub(crate) fn merge_k_list(v: Vec<Vec<u32>>) -> Option<Box<ListNode<u32>>> {
    if v.is_empty() {
        return None;
    }

    let mut dummy_head = ListNode::<u32>::new(0); // 虚拟头节点
    let mut p = &mut dummy_head;

    // 1. 创建一个最小堆，用于存储每个链表的头节点
    let mut heap = BinaryMinHeap::new();

    // 2. 将 K 个链表的头结点加入最小堆, 时间复杂度: O(K * m), K: 链表长度, m 是链表的平均长度
    for v in v.iter() {
        // 创建节点
        let head = create(v.clone()); // O(m)，其中 m 是链表的平均长度
        if let Some(head) = head {
            heap.push(head)
        }
    }

    // 时间复杂度为: O(K * log n), 假设弹出堆顶节点的时间复杂度为 O(log n)，其中 n 是堆中的元素数量, K: 链表长度
    while !heap.is_empty() {
        // 3. 从最小堆中弹出堆顶节点（即当前最小的节点），将其添加到结果链表中
        let node = heap.delete(); // 时间复杂度为 O(K * log n)
        p.next = node.clone();

        // 4. 如果弹出的节点有下一个节点，则将下一个节点插入到最小堆中，以保持堆的性质。时间复杂度为 O(K * log n)
        if let Some(mut node) = node {
            let next = node.as_mut().next.take();
            if let Some(next) = next {
                heap.push(next.clone());
            }
        }

        // p 指针不断前进
        p = p.next.as_mut().unwrap().as_mut();
    }

    // 5. 将结果链表的最后一个节点的 next 字段设置为 None，以避免环形链表
    p.next = None;

    dummy_head.next
}

/// 单链表的倒数第 k 个节点
/**
  使用双指针
  1. p1 指针先走 k 步
  2. p2 指针从头开始走
  3. 当 p1 指针走到结尾, 取 p2 指针对应的节点, 即为 k 节点
  p1 指针和 p2指针中间相关 k 步
*/
pub(crate) fn find_from_end(head: &Option<Box<ListNode<u32>>>, k: u32) -> Option<Box<ListNode<u32>>> {
    if head.is_none() {
        return None;
    }

    let mut p1 = head;
    let mut p2 = head;  // 2. p2 指针从头开始走

    for _ in 0 .. k {
        p1 = &p1.as_ref().unwrap().next;
    }

    // 3. 当 p1 指针走到结尾, 取 p2 指针对应的节点, 即为 k 节点
    while p1.as_ref().is_some()  {
        p1 = &p1.as_ref().unwrap().next;
        p2 = &p2.as_ref().unwrap().next;
    }

    // 4. 结束后, 取 p2 指针对应的节点
    return p2.clone();
}

/// 删除链表的倒数第 N 个结点
/// 查找到 N 的前一个节点 N + 1, 然后把 N + 1 的下一个节点指向 N - 1个节点
pub(crate) fn remove_n_from_end(v1: Vec<u32>, k: u32) -> Option<Box<ListNode<u32>>> {
    if v1.is_empty() {
        return None;
    }

    let len = (v1.len() - 1) as u32;
    if k > len - 1 {
        return None
    }

    let mut dummy = ListNode::<u32>::new(0);
    dummy.next = create(v1.clone());

    // 查找 N + 1 的节点
    let mut node = find_from_end(&dummy.next, k + 1);
    if let Some(mut head) = node.as_mut() {
        if let Some(mut next_node) = head.next.take() {
            head.next = next_node.next
        }
    }

    return dummy.next;
}

