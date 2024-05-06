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

/// 移除所有数值等于 k 的元素
/// 解题思路同 `删除有序数组中的重复项`
pub(crate) fn remove_k_element(v1: Vec<u32>, k: u32) -> (u32, Vec<u32>) {
    if v1.is_empty() {
        return (0, Vec::new());
    }

    let len = v1.len();
    let mut p1 = 0; // 快指针
    let mut p2 = 0; // 慢指针

    let mut v: Vec<u32> = Vec::new();
    let mut i = 0;
    while p1 < len {
        let value1 = v1.get(p1).unwrap();
        if value1.clone() != k {
            p2 = p1;
            i += 1;
            v.push(value1.clone());
        }

        p1 += 1;
    }

    return (i, v);
}

/// 将数组中的所有为 k 的元素移动到数组末尾
/// 通过 `移除所有数值等于 k 的元素` 获取所有值为 `k` 的元素, 然后在后面添加
pub(crate) fn move_k_element(v1: Vec<u32>, k: u32) -> Vec<u32> {
    let (new_len, new_v1) = remove_k_element(v1.clone(), k);
    if new_len == 0 || new_v1.len() == 0 {
        return Vec::new();
    }

    let i = v1.len() as u32 - new_len;
    let mut v2 = new_v1.clone();
    if i > 0 {
        for _ in 0..i {
            v2.push(k);
        }
    }

    return v2;
}

/// 二分查找(升序数组)
/**
  通过中位数来查找, 中位数计算: left + [left, right]
  1. 当 k < 中位数时, 说明 k 在中位数的左边, 左指针 left 需要减 1
  2. 当 k > 中位数时, 说明 k 在中位数的右边, 右指针 right 需要加 1
  3. 当 k = 中位数时, 说明找到值啦
**/
pub(crate) fn binary_search(v1: Vec<u32>, k: u32) -> i32 {
    if v1.is_empty() {
        return -1;
    }

    // 升序
    // v1.sort();

    // 降序
    // v1.sort_by(|a, b| b.cmp(a));

    let mut left = 0;
    let mut right = v1.len() - 1;

    // 判断第一个值是否等于 k
    let first = v1.get(left).unwrap().clone();
    if first == k {
        return left as i32;
    }

    // 判断最后一个会是是否等于 k
    let last = v1.get(right).unwrap().clone();
    if last == k {
        return right as i32;
    }

    while left <= right {
        // let middle = (left + right).div_ceil(2); // 向下取整, 可能有益处风险
        let middle = left + (right - left).div_ceil(2); // left + [left, right] 的中位数
        let value1 = v1.get(middle).unwrap().clone();
        if k == value1 {
            return middle as i32;
        } else if k < value1 {
            right = middle - 1; // 当小于时, 右边界在 middle 左边
        } else if k > value1 {
            left = middle + 1; // 当大于时, 左边界在 middle 右边
        }

        if right <= 0 || left >= v1.len() - 1 {
            return -1;
        }
    }

    return -1;
}

/// 寻找一个数, 如果有重复值, 则返回第一个找到的位置（寻找左侧边界的二分搜索)(升序数组)
/**
    通过中位数来查找, 中位数计算: left + [left, right]
    1. 当 k < 中位数时, 说明 k 在中位数的左边, 左指针 left 需要减 1
    2. 当 k > 中位数时, 说明 k 在中位数的右边, 右指针 right 需要加 1
    3. 当 k = 中位数时, 右指针 right 需要减 1, 并继续向左搜索
**/
pub(crate) fn find_left_bound(v1: Vec<u32>, k: u32) -> i32 {
    if v1.is_empty() {
        return -1;
    }

    let mut left = 0;
    let mut right = v1.len() - 1;
    let mut result: i32 = -1;

    // 判断第一个值是否等于 k
    let first = v1.get(left).unwrap().clone();
    if first == k {
        return left as i32;
    }

    while left <= right {
        // let middle = (left + right).div_ceil(2); // 向下取整, 可能有益处风险
        let middle = left + (right - left).div_ceil(2); // left + [left, right] 的中位数
        let value1 = v1.get(middle).unwrap().clone();
        if k == value1 {
            result = middle as i32;
            right = middle - 1; // 继续向左搜索
        } else if k < value1 {
            right = middle - 1; // 当小于时, 右边界在 middle 左边
        } else if k > value1 {
            left = middle + 1; // 当大于时, 左边界在 middle 右边
        }

        if right <= 0 || left >= v1.len() - 1 {
            return -1;
        }
    }

    result
}

/// 寻找一个数, 如果有重复值, 则返回第最后一个找到的位置（寻找右侧边界的二分搜索)(升序数组)
/**
  通过中位数来查找, 中位数计算: left + [left, right]
    1. 当 k < 中位数时, 说明 k 在中位数的左边, 左指针 left 需要减 1
    2. 当 k > 中位数时, 说明 k 在中位数的右边, 右指针 right 需要加 1
    3. 当 k = 中位数时, 右指针 left 需要加 1, 并继续向右搜索
**/
pub(crate) fn find_right_bound(v1: Vec<u32>, k: u32) -> i32 {
    if v1.is_empty() {
        return -1;
    }

    let mut left = 0;
    let mut right = v1.len() - 1;
    let mut result: i32 = -1;

    // 判断第一个值是否等于 k
    let last = v1.get(right).unwrap().clone();
    if last == k {
        return right as i32;
    }

    while left <= right {
        // let middle = (left + right).div_ceil(2); // 向下取整, 可能有益处风险
        let middle = left + (right - left).div_ceil(2); // left + [left, right] 的中位数
        let value1 = v1.get(middle).unwrap().clone();
        if k == value1 {
            result = middle as i32;
            left = middle + 1; // 继续向右搜索
        } else if k < value1 {
            right = middle - 1; // 当小于时, 右边界在 middle 左边
        } else if k > value1 {
            left = middle + 1; // 当大于时, 左边界在 middle 右边
        }

        if right <= 0 || left >= v1.len() - 1 {
            return -1;
        }
    }

    result
}

/// 两数之和
/**
  1. 使用 `二分搜索` 算法 O(n log n)
  2. 使用双指针从左向右搜索 O(n)
**/
pub(crate) fn two_sum(v1: Vec<u32>, k: u32) -> (i32, i32) {
    if v1.is_empty() {
        return (-1, -1);
    }

    let mut left = 0;
    let mut right = v1.len() - 1;

    while left < right {
        let value1 = v1.get(left).unwrap().clone();
        let value2 = v1.get(right).unwrap().clone();
        let sum = value1 + value2;
        if sum == k {
            return (left as i32, right as i32);
        } else if sum < k {
            // 让 sum 大一点
            left += 1;
        } else if sum > k {
            // 让 sum 小一点
            right -= 1;
        }
    }

    return (-1, -1);
}

/// 三数之和
/**
 固定一个数, 使用双指针算剩下的数
**/
pub(crate) fn three_sum(v1: Vec<u32>, k: u32) -> (i32, i32, i32) {
    if v1.is_empty() {
        return (-1, -1, -1);
    }

    for i in 0..v1.len() {
        let value = v1.get(i).unwrap().clone();
        let total = k - value; // 剩下的值, 使用双指针查找

        let mut left = i + 1;
        let mut right = v1.len() - 1;
        let value1 = v1.get(left).unwrap().clone();
        let value2 = v1.get(right).unwrap().clone();
        while left < right {
            let sum = value1 + value2;
            if sum == total {
                return (i as i32, left as i32, right as i32);
            } else if sum < total {
                // 让 sum 大一点
                left += 1;
            } else if sum > total {
                // 让 sum 小一点
                right -= 1;
            }
        }
    }

    return (-1, -1, -1);
}
