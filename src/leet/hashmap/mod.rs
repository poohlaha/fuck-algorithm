use std::collections::HashMap;

pub mod test;

/**
  1394. 找出数组中的幸运数
  力扣: https://leetcode.cn/problems/find-lucky-integer-in-an-array/description
  题目: 在整数数组中，如果一个整数的出现频次和它的数值大小相等，我们就称这个整数为「幸运数」。
       给你一个整数数组 arr，请你从中找出并返回一个幸运数。
       1. 如果数组中存在多个幸运数，只需返回 最大 的那个。
       2. 如果数组中不含幸运数，则返回 -1 。
*/
pub fn find_lucky(arr: Vec<i32>) -> i32 {
    if arr.is_empty() {
        return -1;
    }

    let mut map = HashMap::new();

    for x in arr {
        *map.entry(x).or_insert(0) += 1;
    }

    let mut answer = -1;
    for (num, freq) in map {
        if num == freq {
            answer = answer.max(num);
        }
    }

    answer
}
