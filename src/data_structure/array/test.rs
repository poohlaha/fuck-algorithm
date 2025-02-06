use crate::data_structure::array::Array;

/// 测试 `动态数组`
fn test_dynamic_array() {
    println!("dynamic array:");
    let mut arr = Array::new();
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

pub fn test() {
    println!("----- array start ------");
    test_dynamic_array();
    println!("----- array end ------");
}
