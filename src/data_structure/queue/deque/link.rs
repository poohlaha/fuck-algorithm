/*!
  双端队列, 通过 `链表` 实现
*/

use std::cell::RefCell;
use std::fmt::{Debug, Display};
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Node<T> {
    element: T,
    prev: Option<Weak<RefCell<Node<T>>>>, // 指向前一个节点的指针
    next: Option<Rc<RefCell<Node<T>>>>,   // 指向后一个节点的指针
}

impl<T> Node<T> {
    pub fn new(element: T) -> Self {
        Self {
            element,
            next: None,
            prev: None,
        }
    }

    pub fn create(element: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Self {
            element,
            next: None,
            prev: None,
        }))
    }
}

pub(crate) struct Deque<T> {
    front: Option<Rc<RefCell<Node<T>>>>, // 队列头指针
    back: Option<Rc<RefCell<Node<T>>>>,  // 队列尾指针
    size: usize,                         // 队列大小
}

impl<T: Clone + Display + Debug> Deque<T> {
    pub fn new() -> Self {
        Self {
            front: None,
            back: None,
            size: 0,
        }
    }

    // 入队操作: 添加元素到队列的尾部
    pub fn push_back(&mut self, element: T) {
        let node = Node::create(element);

        // 为空时, 头节点和尾节点指针指向同一元素
        if let Some(back) = self.back.take() {
            back.borrow_mut().next = Some(Rc::clone(&node));
            node.borrow_mut().prev = Some(Rc::downgrade(&back));
        } else {
            self.front = Some(Rc::clone(&node));
        }

        self.back = Some(node);
        self.size += 1;
    }

    // 入队操作: 添加元素到队列的头部
    pub fn push_front(&mut self, element: T) {
        let node = Node::create(element);

        // 为空时, 头节点和尾节点指针指向同一元素
        if let Some(front) = self.front.take() {
            front.borrow_mut().prev = Some(Rc::downgrade(&node));
            node.borrow_mut().next = Some(Rc::clone(&front));
        } else {
            self.back = Some(Rc::clone(&node));
        }

        self.front = Some(node);
        self.size += 1;
    }

    // 出队操作：从队列头部删除元素
    pub fn pop_front(&mut self) -> Option<T> {
        self.front.take().map(|front| {
            // 只有一个元素时, 删除后头节点和尾节点指针都为 None
            if let Some(next) = front.borrow().next.as_ref() {
                next.borrow_mut().prev = None;
                self.front = Some(Rc::clone(&next));
            } else {
                self.back = None;
            }

            self.size -= 1;
            front.borrow().element.clone()
        })
    }

    // 出队操作: 从队列尾部删除元素
    pub fn pop_back(&mut self) -> Option<T> {
        self.back.take().map(|back| {
            // 只有一个元素时, 删除后头节点和尾节点指针都为 None
            if let Some(prev) = back.borrow().prev.as_ref() {
                prev.upgrade().unwrap().borrow_mut().next = None;
                if let Some(prev) = Some(prev).and_then(|weak| weak.upgrade()) {
                    self.back = Some(prev);
                }
            } else {
                self.front = None;
            }

            self.size -= 1;
            back.borrow().element.clone()
        })
    }

    // 获取队列的头部元素
    pub fn front(&self) -> Option<T> {
        self.front
            .as_ref()
            .map(|front| front.borrow().element.clone())
    }

    // 获取队列的尾部元素
    pub fn back(&self) -> Option<T> {
        self.back.as_ref().map(|back| back.borrow().element.clone())
    }

    // 检查队列是否为空
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    // 获取队列的大小
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn print(&self) {
        let mut current = self.front.clone();
        while let Some(node) = current {
            print!("{} ", node.borrow().element.clone());
            current = node.borrow().next.clone(); // 移动到下一个节点
        }

        println!();
    }
}
