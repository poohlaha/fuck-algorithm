//! 单向链表

use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct ListNode<E> {
    pub(crate) element: E,
    pub(crate) next: Option<Box<ListNode<E>>>
}

impl<E> ListNode<E> {

    pub fn new(element: E) -> Self {
        Self {
            element,
            next: None
        }
    }
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

/// 根据数组创建链表
pub(crate) fn create<E>(arr: Vec<E>) -> Option<Box<ListNode<E>>>
where E: Clone
{
    if arr.is_empty() {
        return None
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

        return head
    }

    return None
}

/// 打印链表
pub(crate) fn print<E>(head: Option<Box<ListNode<E>>>)
where
    E: Debug
{
    let mut current = head;
    while let Some(node) = current {
        print!("{:#?} ", node.element);
        current = node.next
    }
}