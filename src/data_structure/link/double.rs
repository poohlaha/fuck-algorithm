/*!
  双向链表
    双向链表使用 `RC<T>` 来共享所有权, 而 `Box<T>` 是独占所有权。
  - Box<T> 是 堆分配的智能指针，它的大小是 固定的(通常是一个指针的大小，即 8 字节在 64 位系统上), 独占所有权，而 prev 需要一个指向前一个节点的弱引用
  - RC<T>
    - Rc<T> 允许多个变量共享同一个 T 的所有权，并通过引用计数来管理 T 的生命周期
      - Rc<T> 是 强引用，它会增加 strong_count，导致 Rc 无法释放，形成循环引用
      - Weak<T> 是 弱引用，它不会增加 strong_count，Rust 允许 Weak 引用的对象被正确回收
      - 当 next 指向 Rc，prev 只用 Weak，这样 prev 就不会导致对象无法释放
*/

use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::{Rc, Weak};

#[derive(Clone, Debug)]
pub struct Node<T> {
    element: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Weak<RefCell<Node<T>>>>, // Rc<T> 会 增加强引用计数，但 Weak<T> 不会增加计数，它只是一个 弱引用, 避免形成循环引用
}

impl<T: Clone> Node<T> {
    fn new(element: T) -> Self {
        Self {
            element,
            next: None,
            prev: None,
        }
    }

    fn crete(element: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Self {
            element,
            next: None,
            prev: None,
        }))
    }
}

pub struct Link<T> {
    head: Option<Rc<RefCell<Node<T>>>>, // 头节点
    size: usize,
}

impl<T: Clone + Debug> Link<T> {
    pub(crate) fn new() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }

    // 在头部添加
    pub fn add_first(&mut self, element: T) {
        let node = Node::crete(element.clone());

        // 1. 头节点为空
        if self.head.is_none() {
            self.head = Some(node);
            self.size += 1;
            return;
        }

        // 2. 在头部插入
        if let Some(cur) = self.head.take() {
            node.borrow_mut().next = Some(Rc::clone(&cur));
            // 旧头节点的 prev 指向新节点
            cur.borrow_mut().prev = Some(Rc::downgrade(&node));
            // 新头节点的 next 指向旧头
            self.head = Some(node);
        }

        self.size += 1;
    }

    // 在尾部添加
    pub fn add_last(&mut self, element: T) {
        if self.head.is_none() {
            self.add_first(element);
            return;
        }

        let node = Node::crete(element.clone());
        let mut current = self.head.clone();
        while let Some(cur) = current {
            let mut current_borrow = cur.borrow_mut();
            if current_borrow.next.is_none() {
                current_borrow.next = Some(Rc::clone(&node));
                node.borrow_mut().prev = Some(Rc::downgrade(&cur));
                self.size += 1;
                return;
            }
            current = current_borrow.next.clone()
        }
    }

    // 在索引位置插入
    pub fn add(&mut self, element: T, index: usize) {
        // 1. 在头节点处插入
        if index == 0 {
            self.add_first(element);
            return;
        }

        // 2. 在尾节点处插入
        if index >= self.size {
            self.add_last(element);
            return;
        }

        // 3. 在索引位置处插入
        let node = Node::crete(element.clone());
        let current = self.head.clone();
        if let Some(mut cur) = current {
            for _ in 0..index - 1 {
                let next = cur.borrow_mut().next.clone();
                if let Some(next) = next {
                    cur = next.clone()
                }
            }

            // 下一个节点
            let next = cur.borrow_mut().next.take();
            // 下一个节点的前一个节点为新节点
            if let Some(next) = next.clone() {
                next.borrow_mut().prev = Some(Rc::downgrade(&node));
            }

            // 新节点的前一个节点指向当前索引节点
            node.borrow_mut().prev = Some(Rc::downgrade(&cur));
            // 新节点的下一个节点指向当前索引位的下一个节点
            node.borrow_mut().next = next.clone();

            // 当前节点的下一个节点指向新节点
            cur.borrow_mut().next = Some(Rc::clone(&node));
            self.size += 1;
        }
    }

    // 在头节点处删除
    pub fn remove_first(&mut self) {
        if self.head.is_none() {
            return;
        }

        let current = self.head.clone();
        if let Some(cur) = current {
            let next = cur.borrow_mut().next.take();
            if let Some(cur) = next {
                cur.borrow_mut().prev = None;
                self.head = Some(Rc::clone(&cur));
                self.size -= 1;
            }
        }
    }

    // 在尾节点处删除
    pub fn remove_last(&mut self) {
        if self.head.is_none() {
            return;
        }

        if self.size == 1 {
            self.remove_first();
            return;
        }

        let mut current = self.head.clone();
        if let Some(mut cur) = current {
            for i in 0..self.size - 2 {
                let next = cur.borrow_mut().next.clone();
                if let Some(next) = next {
                    cur = next.clone()
                }
            }

            // 当前节点的下一个节点为空
            cur.borrow_mut().next = None;
            self.size -= 1;
        }
    }

    // 在索引位置删除
    pub fn remove(&mut self, index: usize) {
        if index >= self.size {
            return;
        }

        if index == 0 {
            self.remove_first();
            return;
        }

        if index == self.size {
            self.remove_last();
            return;
        }

        let mut current = self.head.clone();
        if let Some(mut cur) = current {
            for _ in 0..index - 1 {
                let next = cur.borrow_mut().next.clone();
                if let Some(next) = next {
                    cur = next.clone()
                }
            }

            // 下一个节点
            let next = cur.borrow_mut().next.take();
            if let Some(node) = next {
                // 下下个节点
                let next = node.borrow_mut().next.clone();
                if let Some(next) = next {
                    // 当前节点的下一个节点等于下下个节点
                    cur.borrow_mut().next = Some(Rc::clone(&next));
                    // 下下个节点的前一个节点等于当前节点
                    next.borrow_mut().prev = Some(Rc::downgrade(&cur));
                    self.size -= 1;
                }
            }
        }
    }
}

// 其他功能
impl<T: Clone + Debug> Link<T> {
    // 输入一个数组, 转换成一条双向链表
    pub fn create(&mut self, arr: Vec<T>) {
        if arr.is_empty() {
            return;
        }

        let mut iter = arr.into_iter();
        let mut head = Some(Node::crete(iter.next().unwrap()));
        let mut current = head.clone();

        for t in iter {
            if let Some(mut cur) = current {
                let node = Node::crete(t);

                // 新节点的前一个节点 = 当前节点
                node.borrow_mut().prev = Some(Rc::downgrade(&cur));

                // 当前节点的下一个节点 = 新节点
                cur.borrow_mut().next = Some(node.clone());
                self.size += 1;

                current = cur.borrow_mut().next.clone();
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

        println!("{:#?}", self.head);
    }
}
