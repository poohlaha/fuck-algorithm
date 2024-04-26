//! 字符串操作

use std::collections::HashMap;

/// 最小覆盖子串
/// 给一个字符串 s 、一个字符串 t 。返回 s 中涵盖 t 所有字符的最小子串。如果 s 中不存在涵盖 t 所有字符的子串，则返回空字符串 ""
/**
 设置左右指针, 分别指向开头
 1. 右指针向右移动，每次移动一步。我们不断更新窗口内的数据，直到窗口包含了目标字符串中的所有字符。
 2. 开始缩小窗口, 左指针向右移动，每次移动一步。不断更新窗口内的数据，直到窗口不再包含目标字符串中的所有字符。
*/
pub(crate) fn find_min_str(s: &str, t: &str) -> String {
    if s.is_empty() || t.is_empty() {
        return String::new();
    }

    let mut target: HashMap<char, usize> = HashMap::new(); // 目标字符串, 用来存储字符串 t 中每个字符出现的次数
    let mut window: HashMap<char, usize> = HashMap::new(); // 待搜索字符串, 用来存储当前窗口中每个字符出现的次数

    // 统计字符串 t 中的字符
    for ch in t.chars() {
        *target.entry(ch).or_insert(0) += 1;
    }

    // 设置左右指针, 分别指向开头
    let mut left = 0; // 左指针
    let mut right = 0; // 右指针
    let mut validate = 0; // 有效值, 等于 target 的长度

    // 记录最小覆盖子串的起始索引及长度
    let mut start = 0; // 最小覆盖子串的起始索引
    let mut len = usize::MAX; // 最小覆盖子串的长度

    let chars: Vec<char> = s.chars().collect();
    while right < chars.len() {
        let c = chars[right];
        right += 1;

        // 判断是否存在, 存在则添加到 window 中
        if let Some(b) = target.get(&c) {
            *window.entry(c).or_insert(0) += 1; // 存在则插入并增加1, 不存在则插入并增加1
            if window[&c] == *b {
                validate += 1;
            }
        }

        // 当都包含 t 的所有字符后, 开始移动左指针
        // 判断是否需要收缩窗口, 左指针向右移动，每次移动一步。我们不断更新窗口内的数据，直到窗口不再包含目标字符串中的所有字符。
        while validate == target.len() {
            // 更新最小覆盖子串, 收缩窗口
            if right - left < len {
                start = left;
                len = right - left;
            }

            let d = chars[left];
            left += 1;

            // 直到窗口不再包含目标字符串中的所有字符
            if let Some(b) = target.get(&d) {
                if window[&d] == *b {
                    validate -= 1;
                }

                window.entry(d).and_modify(|x| *x -= 1);
            }
        }
    }

    if len == usize::MAX {
        return String::new();
    }

    chars[start..start + len].iter().collect()
}
