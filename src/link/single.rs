//! 单链表

use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct ListNode<T> {
    pub element: T,
    pub next: Option<Box<ListNode<T>>>,
}

#[derive(Debug, Clone)]
pub struct LinkedList<T> {
    head: Option<Box<ListNode<T>>>,
    pub(crate) len: usize,
}

impl<T> ListNode<T> {
    pub fn new(element: T) -> Self {
        Self {
            element,
            next: None,
        }
    }
}

impl<T: Clone + Debug> LinkedList<T> {
    pub(crate) fn new() -> Self {
        Self { head: None, len: 0 }
    }

    /// 创建链表
    pub(crate) fn create(&mut self, arr: Vec<T>) {
        if arr.is_empty() {
            return;
        }

        let first = arr.get(0);
        if let Some(first) = first {
            let mut head = Some(Box::new(ListNode::<T>::new(first.clone())));
            let mut cur = &mut head;
            for x in arr.iter().skip(1) {
                let new_node = Some(Box::new(ListNode::new(x.clone())));
                if let Some(ref mut node) = cur {
                    node.next = new_node.clone();
                    cur = &mut node.next;
                }
            }

            self.head = head.clone();
            self.len = arr.len();
        }
    }

    /// 打印链表
    pub(crate) fn print(&self) {
        let head = self.head.clone();
        if head.is_none() {
            return;
        }

        let mut current = head;
        while let Some(node) = current {
            print!("{:#?} ", node.element);
            current = node.next
        }
    }

    /// 插入
    pub(crate) fn insert(&mut self, element: T, index: usize) {
        if index > self.len {
            return;
        }

        let mut new_node: Option<Box<ListNode<T>>> = Some(Box::new(ListNode::new(element)));
        if self.head.is_none() {
            self.head = new_node;
            self.len += 1;
            return;
        }

        // 在第一个插入
        if index == 0 {
            if let Some(node) = new_node.as_mut() {
                node.next = self.head.take();
                self.head = new_node;
            }
            self.len += 1;
            return;
        }

        // 在尾部插入, 需要指向最后一个元素
        if index == self.len {
            let mut current = self.head.as_mut();
            while let Some(node) = current {
                if node.next.is_none() {
                    node.next = new_node;
                    self.len += 1;
                    return;
                }

                current = node.next.as_mut();
            }
            return;
        }

        // 在中间插入, 需要先找到索引位置
        if let Some(mut node) = self.head.as_mut() {
            for _ in 0..index - 1 {
                if let Some(next) = node.next.as_mut() {
                    node = next;
                } else {
                    return;
                }
            }

            if let Some(mut new_node) = new_node {
                new_node.next = node.next.take();
                node.next = Some(new_node)
            }

            self.len += 1;
        }
    }

    /// 删除
    pub(crate) fn delete(&mut self, index: usize) {
        if self.head.is_none() || index > self.len {
            return;
        }

        // 删除第一个
        if index == 0 {
            let head = self.head.take();
            if let Some(head) = head {
                self.head = head.next;
            }
            self.len -= 1;
            return;
        }

        // 删除最后一个
        if index == self.len {
            if let Some(mut current) = self.head.as_mut() {
                while let Some(node) = current.next.take() {
                    if node.next.is_none() {
                        current.next = None;
                        self.len -= 1;
                        return;
                    }

                    current.next = Some(node); // 重新放回节点
                    current = current.next.as_mut().unwrap(); // 移动到下一个节点
                }

                // 只有一个节点
                self.head = None;
                self.len -= 1;
            }
        }

        // 删除中间, 需要先找到索引位置
        if let Some(mut node) = self.head.as_mut() {
            let i = index - 1;
            if i > 0 {
                for _ in 0..i - 1 {
                    if let Some(next) = node.next.as_mut() {
                        node = next;
                    } else {
                        return;
                    }
                }
            }

            let next_node = node.next.take();
            if let Some(next_node) = next_node {
                if let Some(next_node) = next_node.next {
                    node.next = Some(next_node);
                }
            }

            self.len -= 1;
        }
    }
}




