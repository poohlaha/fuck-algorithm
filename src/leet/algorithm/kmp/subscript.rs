/*!
  28. 找出字符串中第一个匹配项的下标
  力扣: https://leetcode.cn/problems/find-the-index-of-the-first-occurrence-in-a-string/description/
  题目: 给你两个字符串 haystack 和 needle ，请你在 haystack 字符串中找出 needle 字符串的第一个匹配项的下标（下标从 0 开始）。
       如果 needle 不是 haystack 的一部分，则返回  -1 。

  解: 使用 `KMP` 算法
*/

use std::cmp::max;
use std::collections::HashMap;

pub struct Subscript;

impl Subscript {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if haystack.is_empty() || needle.is_empty() {
            return -1;
        }

        // 定义 `模式串`
        let pattern = needle.chars().collect::<Vec<char>>();

        // 1. 定义 next(lps) 数组
        let m = pattern.len();
        let mut next = vec![0; m];

        // 1.1 定义辅助指针 `j`, 表示 `当前最长前后缀的长度`
        let mut j = 0;

        // 从 `i = 1` 到 `m` 开始循环，因为 `next[0] = 0`
        for i in 1..m {
            // 1.2 比较 pattern[i] 和 pattern[j]
            // 1.2.1 不相等, 则回退, 当 j = 0 时, 无法回退, 设置 next[i] = j; 回退 j, j = next[j - 1], 并重新比较 pattern[i] 和 pattern[j]
            while j > 0 && pattern[i] != pattern[j] {
                j = next[j - 1];
            }

            // 1.2.2 相等, 则 i 不动, j += 1
            if pattern[i] == pattern[j] {
                j += 1;
            }

            // 设置 next[i] = j
            next[i] = j;
        }

        // 2. 匹配过程
        // 2.1 定义一个指针 `j` 指向模式串, 初始为 `0`
        let mut j = 0; // `j` 指向模式串 pattern

        // 定义主串
        let text = haystack.chars().collect::<Vec<char>>();
        let m = text.len();

        // let mut result = vec![];
        // 2.2 从 i = 0 到 m 开始循环
        for i in 0..m {
            // 2.2 比较 text[i] 和 pattern[j]
            // 1.2.1 不相等, 则回退, 当 j = 0 时, 无法回退; 回退 j, j = next[j - 1], 并重新比较 text[i] 和 pattern[j]
            while j > 0 && text[i] != pattern[j] {
                j = next[j - 1];
            }

            // 1.2.2 相等, 则 `j += 1`
            if text[i] == pattern[j] {
                j += 1;
            }

            // 如果 `j = m`, `匹配成功`
            // j 是模式串的长度, 匹配起始位置是 i + 1 - m
            if j == m {
                // 多匹配模式
                // result.push((i + 1 - m) as i32);  // 返回匹配起始下标
                // j = next[j - 1]; // 继续寻找下一个匹配
                return (i + 1 - m) as i32;
            }
        }

        -1
    }

    // 使用 QS 算法
    pub fn str_str_qs(haystack: String, needle: String) -> i32 {
        if haystack.is_empty() || needle.is_empty() {
            return -1;
        }

        let pattern = needle.as_bytes();
        let m = pattern.len();

        // 1. 创建 `字母表`(`ASCII 256`) 的数组 `shift[]` 数组, 初始化值为 `pattern.len() + 1`
        let mut shift = vec![m + 1; 256];

        // 1.1 `shift[]` 取 `m - 最后一次出现的索引位置`, 即: `shift[c] = m - last_index(c)`
        for (i, &b) in pattern.iter().enumerate() {
            // 每次赋值会覆盖前面的，最终保留的是最右出现位置对应的 m - i
            shift[b as usize] = m - i;
        }

        // 2. 匹配过程
        let text = haystack.as_bytes();

        // 2.1 从 i = 0 开始循环(i + m < text.len())
        let mut i = 0;
        while i + m < text.len() {
            // 2.2 比较 `text[i..m + i] == pattern` 是否相等, 相等, 返回 `i`
            if &text[i..m + i] == pattern {
                return i as i32;
            }

            // 2.3 不相等, 查找窗口后的字符(text[i + m]) 在 shift 数组中是否存在, 存在取值, 不存在取默认(pattern.len() + 1)
            let next_char = if i + m < text.len() {
                text[i + m]
            } else {
                break;
            };

            // `i += 窗口后的字符的值`
            i += shift[next_char as usize];
        }

        -1
    }

    // 使用 BM 算法
    pub fn str_str_bm(haystack: String, needle: String) -> i32 {
        if haystack.is_empty() || needle.is_empty() {
            return -1;
        }

        // 1. 预处理阶段
        // 1.1 构建 `坏字符规则表 H`, `H[]` 取 `最后一次出现的索引位置`, 即: `H[c] = last_index(c)`, 不存在取默认值 `-1`
        let mut h: HashMap<u8, isize> = HashMap::new();

        let pattern = needle.as_bytes();
        for (i, &ch) in pattern.iter().enumerate() {
            // 覆盖，保留最后一次出现
            h.insert(ch, i as isize);
        }

        let m = needle.chars().count();
        // 1.2 构建 `好后缀规则表`, 定义两辅助函数 `suffix 数组` 和 `prefix 数组`, 用于确定 `当坏字符规则失效时，模式串应该向右移动多远`
        // 1.2.1 定义 `suffix 数组`(长度为 i 的好后缀，在模式串中 `另一个匹配子串` 的起始下标), 默认值为 `-1`, 长度为 `m`
        let mut suffix = vec![-1; m];

        // 1.2.2 定义 `prefix 数组`(模式串的长度为 i 的后缀，是否同时也是模式串的前缀), 默认值为 `false`, 长度为 `m`
        let mut prefix = vec![false; m];

        // 1.2.3 定义 i = m - 2(需要去除末尾), 从 `右向左` 循环
        let pattern = needle.as_bytes();

        let mut i: isize = m as isize - 2;
        while i >= 0 {
            // 1) 定义临时变量 `j = i`, `k = 0`
            let mut j = i;
            let mut k = 0;

            // 2) 比较 `pattern[j]` 和 `pattern[m - 1 - k]` 是否相等
            while j >= 0 && pattern[i as usize] == pattern[(m - 1 - k) as usize] {
                // 3) 相等, i -= 1, k += 1, 继续比较 `pattern[j]` 和 `pattern[m - 1 - k]` 是否相等
                j -= 1;
                k += 1;
            }

            // 4) 不相等, 没有任何后缀匹配成功, `suffix` 和 `prefix` `不变`

            // 5) 匹配到结果(k > 1), 设置 `suffix[k] = i + 1`, 表示 `长度为 k 的后缀 x，在 pattern 的第 i 个位置出现过`
            if k > 0 {
                suffix[k] = j + 1;
            }

            // 6) 匹配到了开头, 设置 `prefix[k] = true`, 表示 `模式串的长度为 k 的“后缀” == 模式串的“前缀”`, 即: `模式串的后 k 个字符` 和 `和 模式串的前 k 个字符` `完全相同`, 如: `ABA` 和 `A`
            if j < 0 {
                prefix[k] = true;
            }
        }

        // 2. 匹配阶段(从 `右向左` 比对 `pattern` 和 `text` 的窗口，失败就使用 `坏字符` 和 `好后缀` 决定下一步滑动距离)
        // 2.1 定义循环变量 `i = 0`
        let text = haystack.as_bytes();
        let mut i = 0isize;
        while (i + m as isize) < text.len() as isize {
            // 2.2 取出 `text[i .. m - 1 + i]` 和 `pattern` 比较
            // 2.2.1 从 `右向左` `逐个比较` 每个字符
            let mut j = (m - 1) as isize;

            // 向左移动接着比较, 比较窗口大小为 [0..j]
            while j >= 0 && text[(i + j) as usize] == pattern[j as usize] {
                j -= 1;
            }

            // 1) 匹配成功(j < 0), `i += 1`
            if j < 0 {
                i += 1;
                continue;
            }

            // 2) 匹配失败, 触发 `坏字符` 和 `好后缀规则`
            // 2.2.2 匹配 `坏字符规则`, 坏字符 `c = text[j]`, 查找是否在 `坏字符规则表 H` 中存在 `c`
            let bc = text[(i + j) as usize];
            let last = h.get(&bc).unwrap_or(&-1);

            // 坏字符规则移动距离 x = m - 1 - ch, x 可以是负数(如果 last > j)，表示坏字符在模式串右边 → 取至少移动 1 更安全
            let x = if j > *last { j - *last } else { 1 };

            // 2.2.3 匹配好后续规则
            // 好后缀长度 `k` 为 `m - 1 - j`
            let k = m as isize - 1 - j;
            let mut y = 0;

            // 1) 好后缀规则1: 查找 `suffix[k]`, 如果不为 `-1`, 则 `满足`, 取 `y = j - suffix[k] + 1`, 如果不为 `-1`, 则接着使用 `好后缀规则2`
            if k > 0 {
                if suffix[k as usize] != -1 {
                    y = j - suffix[k as usize] + 1
                } else {
                    // 2) 好后缀规则2: 查找 `prefix[k]`, 如果为 `true`, 则 `满足`, 取 `y = m - r(r 为 k 递减)`, 如果 `没有任何前缀匹配`(`y = 0`), 取 `y = m`
                    let mut r = k;
                    while r > 0 {
                        if prefix[r as usize] {
                            y = m as isize - r; // 将模式串前 r 个字符对齐好后缀末尾
                            break;
                        }
                        r -= 1;
                    }

                    if y == 0 {
                        y = m as isize; // 没有任何前缀匹配，整体滑动模式串长度
                    }
                }
            }

            // 滑动距离, 取 x 和 y 最大值: max(x, y), i += 滑动距离
            i += max(x, y)
        }

        -1
    }
}
