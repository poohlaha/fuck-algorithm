use crate::leet::link::ring::CircularLinkedList;
use crate::leet::link::sum::NumLink;

/// 测试 `环形链表`
fn test_link_ring() {
    println!("----- ring start ------");
    let head = vec![3, 2, 0, -4];
    let pos = 1;
    let has_cycle = CircularLinkedList::has_cycle(head.clone(), pos);
    println!("has cycle: {}", has_cycle);

    let cycle = CircularLinkedList::detect_cycle(head.clone(), pos);
    if let Some(cycle) = cycle {
        println!("cycle entrance: {:#?}", cycle.borrow().val);
    }

    println!("----- ring end ------");
}

/// 测试 `两数之和`
fn test_add_two_numbers() {
    println!("----- sum start ------");
    let l1 = vec![2, 4, 3];
    let l2 = vec![5, 6, 4];
    let l1 = NumLink::create(l1);
    let l2 = NumLink::create(l2);
    let l = NumLink::add_two_numbers(l1, l2);
    println!("[2, 4, 3] 和 [5, 6, 4]: {:?}", l);

    let l1 = vec![9, 9, 9, 9, 9, 9, 9];
    let l2 = vec![9, 9, 9, 9];
    let l1 = NumLink::create(l1);
    let l2 = NumLink::create(l2);
    let l = NumLink::add_two_numbers(l1, l2);
    println!("[9, 9, 9, 9, 9, 9, 9] 和 [9, 9, 9, 9]: {:?}", l);

    println!("----- sum end ------");
}

pub fn test() {
    println!("----- leet code link start ------");
    test_link_ring();
    test_add_two_numbers();
    println!("----- leet code link end ------");
}
