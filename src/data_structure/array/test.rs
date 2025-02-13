use crate::data_structure::array::normal;
use crate::data_structure::array::ring::Ring;
use crate::data_structure::array::skip::Skip;

/// 测试 `动态数组`
fn test_dynamic_array() {
    println!("dynamic array:");
    let mut arr = normal::Array::new();
    // add
    arr.add(1);
    arr.add(5);
    arr.add(8);

    arr.add_by_index(10, 0);
    arr.add_by_index(12, 1);
    arr.add_by_index(21, 3);
    arr.add_by_index(15, 5);
    println!("insert arr: {:?}", arr.display());

    // update
    arr.update(22, 3);
    arr.update(34, 5);
    println!("update arr: {:?}", arr.display());

    // delete
    arr.delete(10);
    arr.delete(5);
    arr.delete(1);
    println!("delete arr: {:?}", arr.display());

    // get
    println!("get arr index: {}, value: {:?}", 0, arr.get(0));
    println!("get arr index: {}, value: {:?}", 3, arr.get(3));
    println!("get arr index: {}, value: {:?}", 5, arr.get(5));

    // 大小
    println!("get arr size: {}", arr.size());

    // 是否为空
    println!("get arr is empty: {}", arr.is_empty());
    println!();
}

/// 测试 `环形数组`
fn test_ring_array() {
    println!("ring array:");
    let mut ring = Ring::new();

    // 1. add first
    println!("add first:");
    ring.add_first(3);
    ring.add_first(2);
    ring.add_first(1);
    ring.print();

    // 2. add last
    println!("add last:");
    ring.add_last(4);
    ring.add_last(5);
    ring.add_last(6);
    ring.print();

    // 3. remove fist
    println!("remove first:");
    ring.remove_first();
    ring.remove_first();
    ring.remove_first();
    ring.print();

    println!("remove last:");
    ring.remove_last();
    ring.print();

    // 3. get first
    println!("get first: {:?}", ring.get_first());
    println!("get last: {:?}", ring.get_last());

    println!();
}

/// 测试 `跳表`
pub fn test_skip() {
    println!("skip list:");
    let mut skip = Skip::new();
    skip.add(3);
    skip.add(6);
    skip.add(7);
    skip.add(9);
    skip.add(12);
    skip.add(19);
    skip.add(17);
    skip.add(26);
    skip.add(21);
    skip.add(25);
    skip.print();
    println!();
}

pub fn test() {
    println!("----- array start ------");
    test_dynamic_array();
    test_ring_array();
    test_skip();
    println!("----- array end ------");
}
