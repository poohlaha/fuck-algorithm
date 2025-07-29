/*!
  14. 最长公共前缀
  力扣: https://leetcode.cn/problems/longest-common-prefix/
  题目: 编写一个函数来查找字符串数组中的最长公共前缀。
       如果不存在公共前缀，返回空字符串 ""。

  总结:
     最长公共前缀(Longest Common Prefix, LCP):
     - 从开头开始(索引 0)
     - 所有字符串在该位置的字符完全相同
     - 且满足 `最长` 的这一段
*/

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new();
    }

    if strs.len() == 1 {
        if let Some(&ref str) = strs.get(0) {
            return str.to_string();
        }

        return String::new();
    }

    // 1. 查找最短字符串, 以确定 left, right 的窗口值
    let min_str = strs.iter().min_by_key(|s| s.len());
    if let Some(min_str) = min_str {
        let mut left: i32 = 0;
        let mut right: i32 = min_str.len() as i32;

        // 二分
        while left <= right && left >= 0 && right > 0 {
            let mid = (left + right) / 2;

            // 判断所有前缀是否相同
            let mut prefix: String = String::new();
            let mut has_diff = true;
            for str in strs.iter() {
                let mut p: String = String::new();
                if mid == 0 {
                    p = str[0..=mid as usize].to_string();
                } else {
                    p = str[0..mid as usize].to_string();
                }

                if prefix.is_empty() {
                    prefix = p;
                    continue;
                }

                if p.as_bytes() != prefix.as_bytes() {
                    has_diff = false;
                    break;
                }
            }

            if has_diff {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        // 获取最长公共前缀长度: (left + right) / 2
        let len = ((left + right) / 2) as usize;
        if let Some(&ref str) = strs.get(0) {
            return str[0..len].to_string();
        }
    }

    String::new()
}
