/*!
 滑动窗口算法
*/

use std::collections::HashMap;

/**
 76. 最小覆盖子串
 力扣: https://leetcode.cn/problems/minimum-window-substring/description/
 题目: 给你一个字符串 s 、一个字符串 t 。返回 s 中涵盖 t 所有字符的最小子串。如果 s 中不存在涵盖 t 所有字符的子串，则返回空字符串 ""
 解:
   1. 创建 `频率表 need(记录每个字符出现的频率)` 和 `哈希表 window(用于统计窗口中各字符出现的次数)`
   2. 初始化两个指针 `left = 0` 、`right = 0`、`valid = 0`
   3. 记录最终找到的最小窗口位置和长度
   4. 向右扩展 `right`, 更新窗口中字符的统计
   5. 如果当前窗口包含 `t` 的所有字符(valid 满足), 开始尝试收缩 `left`，寻找更小的窗口
   6. 在收缩过程中，更新最小窗口
   6. 重复扩展/收缩，直到遍历完整个字符串 `s`
*/
pub fn solution(s: &str, t: &str) -> String {
    // 频率表, 记录 t 中每个字符出现的频率
    let mut need = HashMap::new();
    // 哈希表, 用于统计窗口中各字符出现的次数
    let mut window = HashMap::new();

    // 1. 记录 t 中每个字符出现的频率, 如 'A' : 2, 'B': '1', 'C': '1'
    for ch in t.chars() {
        *need.entry(ch).or_insert(0) += 1;
    }

    // 2. 初始化两个指针 `left = 0` 、`right = 0`、`valid = 0`
    let mut left = 0;
    let mut right = 0;
    let mut valid = 0; // 统计满足条件的字符个数

    // 3. 记录最终找到的最小窗口位置和长度
    let mut start = 0;
    let mut min_len = usize::MAX; // 记录窗口的最小长度

    let chars: Vec<char> = s.chars().collect();

    while right < chars.len() {
        // 4. 向右扩展 `right`, 更新窗口中字符的统计
        let c = chars[right];
        right += 1;

        // 更新窗口中字符的统计
        if need.contains_key(&c) {
            // 更新 window 的字符个数
            *window.entry(c).or_insert(0) += 1;
            // 当 window 中某个字符个数 和 频率表中的某个字符个数 一样时, 增加长度
            if window[&c] == need[&c] {
                valid += 1;
            }
        }

        // 如果当前窗口包含 `t` 的所有字符(valid 满足), 开始尝试收缩 `left`，寻找更小的窗口
        // 只要窗口满足，就持续收缩
        while valid == need.len() {
            // 5. 在收缩过程中，更新最小窗口
            // 窗口长度: right - left, 最小长度: min_left
            if right - left < min_len {
                min_len = right - left;
                start = left;
            }

            // 收缩 left 窗口
            let l = chars[left];
            left += 1;

            if need.contains_key(&l) {
                // 当 window 中某个字符个数 和 频率表中的某个字符个数 一样时, 减少长度
                if window[&l] == need[&l] {
                    valid -= 1;
                }

                // 删除 window 中的字符个数
                *window.entry(l).or_insert(0) -= 1;
            }
        }
    }

    if min_len == usize::MAX {
        return String::new();
    }

    chars[start..start + min_len].iter().collect()
}
