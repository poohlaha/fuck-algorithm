/*!
  单向链表
*/

use std::fmt::Debug;

// Box<T> 是 堆分配的智能指针，它的大小是 固定的（通常是一个指针的大小，即 8 字节在 64 位系统上）
#[derive(Clone, Debug)]
pub struct Node<T> {
    element: T,
    next: Option<Box<Node<T>>>,
}

impl<T: Clone> Node<T> {
    fn new(element: T) -> Self {
        Self {
            element,
            next: None,
        }
    }

    fn crete(element: T) -> Box<Node<T>> {
        Box::new(Node::new(element))
    }
}

// 单向链表
pub struct Link<T> {
    head: Option<Box<Node<T>>>, // 头节点
    size: usize,
}

impl<T: Clone> Link<T> {
    pub(crate) fn new() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }

    // 在头部添加
    pub fn add_first(&mut self, element: T) {
        let mut node = Node::crete(element.clone());

        // 1. 头节点为空
        if self.head.is_none() {
            self.head = Some(node);
            self.size += 1;
            return;
        }

        // 2. 在头部插入
        node.next = self.head.take();
        self.head = Some(node);
        self.size += 1;
    }

    // 在尾部插入
    pub fn add_last(&mut self, element: T) {
        // 1. 节点为空, 在头部插入
        if self.head.is_none() {
            self.add_first(element);
            return;
        }

        // 2. 在尾部插入
        let node = Node::crete(element.clone());
        let mut current = self.head.as_mut();
        while let Some(cur) = current {
            if cur.next.is_none() {
                // 3. 让当前的 current.next 指向新节点
                cur.next = Some(node);
                self.size += 1;
                return;
            }

            current = cur.next.as_mut();
        }
    }

    // 在索引位置插入
    pub fn add(&mut self, element: T, index: usize) {
        // 1. 在头节点插入
        if index == 0 {
            self.add_first(element);
            return;
        }

        // 2. 在尾节点插入
        if index >= self.size {
            self.add_last(element);
            return;
        }

        // 3. 在索引位置插入
        let mut node = Node::crete(element.clone());
        if let Some(mut current) = self.head.as_mut() {
            // head 一直指向 index 索引位置前一个
            for _ in 0..index - 1 {
                if let Some(next) = current.next.as_mut() {
                    current = next;
                } else {
                    return;
                }
            }

            // 找到 index 索引位置前一个节点
            // 1. 把新节点的一下节点指向前一个节点的下一个节点
            // 2. 把前一个节点的下一个节点指向新节点
            node.next = current.next.take(); // node 取下一个节点
            current.next = Some(node); // head 节点的下一个节点指向 node
            self.size += 1;
        }
    }

    // 在头节点处删除
    pub fn remove_first(&mut self) {
        // 1. 链表为空
        if self.head.is_none() {
            return;
        }

        // 2. 当链表只有一个值
        if self.size == 1 {
            self.head = None;
            self.size -= 1;
            return;
        }

        // 2. 把头节点指向头节点的下一个节点
        if let Some(head) = self.head.as_mut() {
            self.head = head.next.take();
            self.size -= 1;
        }
    }

    // 在尾节点处删除
    pub fn remove_last(&mut self) {
        // 1. 链表为空
        if self.head.is_none() {
            return;
        }

        // 2. 当大小为 1 时, 头节点等于尾节点, 清空节点
        if self.size == 1 {
            self.head = None;
            self.size -= 1;
            return;
        }

        // 3. 把尾节点指向尾节点前一个节点
        if let Some(mut current) = self.head.as_mut() {
            while let Some(next) = current.next.as_mut() {
                // 此时 current 为最后第二个位置的节点, 把 current 设为尾节点
                if next.next.is_none() {
                    current.next = None;
                    self.size -= 1;
                    return;
                }

                current = current.next.as_mut().unwrap(); // 移动到下一个节点
            }
        }
    }

    // 删除索引位置的节点
    pub fn remove(&mut self, index: usize) {
        if index >= self.size {
            return;
        }

        // 1. 删除头节点
        if index == 0 {
            self.remove_first();
            return;
        }

        // 2. 删除尾节点
        if index == self.size {
            self.remove_first();
            return;
        }

        // 获取索引位置节点
        if let Some(mut current) = self.head.as_mut() {
            for _ in 0..index - 1 {
                if let Some(next) = current.next.as_mut() {
                    current = next
                } else {
                    return;
                }
            }

            // 找到索引位置为 index 前一个节点
            // 把 current 的下一个节点指向下下个节点
            let mut remove_node = current.next.take();
            if let Some(node) = remove_node.as_mut() {
                current.next = node.next.take();
            }
            self.size -= 1;
        }
    }

    // 获取头节点
    pub fn get_first(&self) -> Option<T> {
        if self.size == 0 {
            return None;
        }

        if let Some(node) = self.head.as_ref() {
            return Some(node.element.clone());
        }

        None
    }

    // 获取尾节点
    pub fn get_last(&self) -> Option<T> {
        if self.size == 0 {
            return None;
        }

        if let Some(mut current) = self.head.as_ref() {
            for _ in 0..self.size - 1 {
                if let Some(next) = current.next.as_ref() {
                    current = next
                } else {
                    return None;
                }
            }

            return Some(current.element.clone());
        }

        None
    }

    // 通过索引查找
    pub fn get(&mut self, index: usize) -> Option<T> {
        if index >= self.size {
            return None;
        }

        if let Some(mut current) = self.head.as_mut() {
            for _ in 0..index - 1 {
                if let Some(next) = current.next.as_mut() {
                    current = next
                } else {
                    return None;
                }
            }

            if let Some(next) = current.next.as_ref() {
                return Some(next.element.clone());
            }
        }

        None
    }
}

// 其他功能
impl<T: Clone + Debug> Link<T> {
    // 输入一个数组, 转换成一条单链表
    pub fn create(&mut self, arr: Vec<T>) {
        if arr.is_empty() {
            return;
        }

        let mut iter = arr.into_iter();
        let mut head = Some(Node::crete(iter.next().unwrap()));
        let mut cur = &mut head;

        for t in iter {
            if let Some(current) = cur {
                current.next = Some(Node::crete(t));
                cur = &mut current.next;
            }
        }

        self.head = head;
    }

    // 打印
    pub fn print(&mut self) {
        if self.head.is_none() {
            println!("None");
            return;
        }

        let mut current = self.head.clone();
        while let Some(node) = current {
            print!("{:#?} ", node.element);
            current = node.next
        }
    }
}
