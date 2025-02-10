use crate::data_structure::link::{double, single};

/// 测试 `单向链表`
fn test_single_link() {
    println!("single link:");
    let mut link = single::Link::new();

    // 1. add first
    link.add_first(1);
    link.add_first(2);
    link.add_first(3);
    link.add_first(4);
    link.add_first(5);

    // 2. add last
    link.add_last(6);
    link.add_last(7);
    link.add_last(8);
    link.add_last(9);
    link.add_last(10);

    // 3. add
    link.add(11, 6);
    link.add(12, 8);
    link.add(13, 10);
    link.print();

    // 4. get first
    println!("first: {:?}", link.get_first());

    // 5. get last
    println!("last: {:?}", link.get_last());

    // 6. get
    println!("get index 1: {:?}", link.get(1));
    println!("get index 6: {:?}", link.get(6));
    println!("get index 10: {:?}", link.get(10));

    println!("origin:");
    link.print();
    println!();

    // 7. remove first
    println!("remove first: ");
    link.remove_first();
    link.remove_first();
    link.remove_first();
    link.print();
    println!();

    // 8. remove last
    println!("remove first: ");
    link.remove_last();
    link.remove_last();
    link.remove_last();
    link.print();
    println!();

    // 9. remove
    println!("remove 1: ");
    link.remove(1);
    link.print();
    println!();

    println!("remove 3: ");
    link.remove(3);
    link.print();
    println!();

    println!("remove 4: ");
    link.remove(4);
    link.print();
    println!();

    println!("create");
    let arr = vec![4, 2, 2, 2, 8, 8, 3, 3, 1, 9];
    let mut link = single::Link::new();
    link.create(arr);
    link.print();

    println!();
}

/// 测试 `双向链表`
fn test_double_link() {
    println!("double link:");
    let mut link = double::Link::new();
    // 1. add first
    link.add_first(3);
    link.add_first(2);

    // 2. add last
    link.add_last(5);
    link.add_last(6);

    // 3. add index
    link.add(1, 0); // add first
    link.add(8, 5); // add last
    link.add(4, 3);
    link.add(7, 6);

    // 4. remove first
    link.remove_first();
    link.remove_first();
    link.remove_first();

    // 5. remove last
    link.remove_last();
    link.print();

    // 6. remove index
    link.remove(1);
    link.remove(2);

    link.print();

    // create
    println!("create");
    let arr = vec![4, 2, 2, 2, 8, 8, 3, 3, 1, 9];
    let mut link = double::Link::new();
    link.create(arr);
    link.print();
    println!();
}

pub fn test() {
    println!("----- linked list start ------");
    test_single_link();
    test_double_link();
    println!("----- linked list end ------");
}
