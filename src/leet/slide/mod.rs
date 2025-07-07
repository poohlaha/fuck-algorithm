use std::cmp::max;
use std::collections::HashMap;

pub mod test;

/**
  3. 无重复字符的最长子串
  题目: 给定一个字符串 s ，请你找出其中不含有重复字符的 最长 子串 的长度。
  力扣: https://leetcode.cn/problems/longest-substring-without-repeating-characters/description/

  解: 使用滑动窗口, 当存储的字符串 > 1 时, 收缩窗口
*/
pub fn length_of_longest_substring(s: String) -> i32 {
    if s.is_empty() {
        return 0;
    }

    let n = s.len();
    if n == 1 {
        return 1;
    }

    let chars = s.chars().collect::<Vec<char>>();
    let mut left = 0;
    let mut right = 0;
    let mut window: HashMap<char, usize> = HashMap::new();
    let mut result: i32 = 0;

    while right < n {
        let char = chars[right];
        right += 1;
        *window.entry(char).or_insert(0) += 1;

        while window.get(&char).unwrap_or(&0) > &1 {
            // 多个字符, 收缩窗口
            let d = chars[left];
            left += 1;
            *window.entry(d).or_insert(0) -= 1;
        }

        result = max(result, (right - left) as i32);
    }

    result
}
