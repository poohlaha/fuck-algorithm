use crate::array::arr::remove_duplicates;

/// 测试 `删除有序数组中的重复项`
fn test_remove_duplicates() {
    println!("remove duplicates");
    let (len, arr) = remove_duplicates(vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4, 5, 5]);
    println!("remove duplicates new len: {}", len);
    println!("remove duplicates new array: {:?}", arr);
}

pub(crate) fn test() {
    test_remove_duplicates();
}
