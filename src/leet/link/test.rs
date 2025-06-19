use crate::leet::link::ring::CircularLinkedList;

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

pub fn test() {
    println!("----- leet code link start ------");
    test_link_ring();
    println!("----- leet code link end ------");
}
