//! 数组操作

/// 删除有序数组中的重复项
/**
使用快慢指针
1. 使用两个指针，一个指向当前元素（慢指针），另一个用于遍历数组（快指针）
2. 遍历数组，如果快指针指向的元素与慢指针指向的元素相同，则快指针向后移动，直到找到一个不同的元素为止
3. 将快指针指向的不同元素的值复制到慢指针指向的下一个位置，并将慢指针向后移动一个位
*/
pub(crate) fn remove_duplicates(v1: Vec<u32>) -> (u32, Vec<u32>) {
    if v1.is_empty() {
        return (0, Vec::new());
    }

    let len = v1.len();
    let mut p1 = 0; // 快指针
    let mut p2 = 0; // 慢指针

    let mut v: Vec<u32> = Vec::new();
    let value = v1.get(0).unwrap();
    v.push(value.clone());

    let mut i = 1;
    while p1 < len {
        let value1 = v1.get(p1).unwrap();
        let value2 = v1.get(p2).unwrap();
        if value1 != value2 {
            p2 = p1;
            i += 1;
            v.push(value1.clone());
        }

        p1 += 1;
    }

    return (i, v);
}
