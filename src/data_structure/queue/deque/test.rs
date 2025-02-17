use crate::data_structure::queue::deque::array::Array;
use crate::data_structure::queue::deque::link::Deque;

/// 测试 `链表实现`
pub fn test_link() {
    let mut link = Deque::new();

    println!("add front:");
    link.push_front(3);
    link.push_front(2);
    link.push_front(1);
    link.print();
    println!("get front: {:?}", link.front());

    println!("add back:");
    link.push_back(4);
    link.push_back(5);
    link.push_back(6);
    link.print();
    println!("get back: {:?}", link.back());

    println!("pop front:");
    link.pop_front();
    link.pop_front();
    link.print();

    println!("pop back:");
    link.pop_back();
    link.pop_back();
    link.print();

    println!("get front: {:?}", link.front());
    println!("get back: {:?}", link.back());
    println!("link is empty: {:?}", link.is_empty());
    println!("link size: {:?}", link.size());
    println!();
}

/// 测试 `环形数组实现`
pub fn test_array() {
    let mut array = Array::new();

    println!("add front:");
    array.push_front(3);
    array.push_front(2);
    array.push_front(1);
    array.print();
    println!("get front: {:?}", array.front());

    println!("add back:");
    array.push_back(4);
    array.push_back(5);
    array.push_back(6);
    array.print();
    println!("get back: {:?}", array.back());

    println!("pop front:");
    array.pop_front();
    array.pop_front();
    array.print();

    println!("pop back:");
    array.pop_back();
    array.pop_back();
    array.print();

    println!("get front: {:?}", array.front());
    println!("get back: {:?}", array.back());
    println!("link is empty: {:?}", array.is_empty());
    println!("link size: {:?}", array.size());
}

pub fn test() {
    println!("----- deque start ------");
    test_link();
    test_array();
    println!("----- deque end ------");
}
