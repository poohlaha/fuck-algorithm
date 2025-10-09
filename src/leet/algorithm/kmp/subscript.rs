/*!
  28. 找出字符串中第一个匹配项的下标
  力扣: https://leetcode.cn/problems/find-the-index-of-the-first-occurrence-in-a-string/description/
  题目: 给你两个字符串 haystack 和 needle ，请你在 haystack 字符串中找出 needle 字符串的第一个匹配项的下标（下标从 0 开始）。
       如果 needle 不是 haystack 的一部分，则返回  -1 。

  解: 使用 `KMP` 算法
*/
pub struct Subscript;

impl Subscript {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if haystack.is_empty() || needle.is_empty() {
            return -1;
        }

        // 定义 `模式串`
        let pattern = needle.chars().collect::<Vec<char>>();

        // 1. 定义 next(lps) 数组
        let n = pattern.len();
        let mut next = vec![0; n];

        // 1.1 定义辅助指针 `j`, 表示当前最长前后缀的长度
        let mut j = 0;

        // 从 i = 1 开始，因为 next[0] = 0
        for i in 1..n {
            // 1.2 比较 pattern[i] 和 pattern[j]
            // 1.2.1 不相等, 则回退, 当 j = 0 时, 无法回退, 设置 next[i] = j; 回退 j, j = next[j - 1], 并重新比较 pattern[i] 和 pattern[j]
            while j > 0 && pattern[i] != pattern[j] {
                j = next[j - 1];
            }

            // 1.2.2 相等, 则 i 不动, j + 1
            if pattern[i] == pattern[j] {
                j += 1;
            }

            // 设置 next[i] = j
            next[i] = j;
        }

        // 2. 匹配过程
        // 2.1 定义两指针, 一个指针 `i` 指向主串, 另一个指针 `j` 指向模式串, 初始为 `0`
        let mut j = 0; // `j` 指向模式串 pattern

        // 定义主串
        let text = haystack.chars().collect::<Vec<char>>();
        let m = text.len();

        // let mut result = vec![];
        for i in 0..m {
            // 2.2 比较 text[i] 和 pattern[j]
            // 1.2.1 不相等, 则回退, 当 j = 0 时, 无法回退; 回退 j, j = next[j - 1], 并重新比较 text[i] 和 pattern[j]
            while j > 0 && text[i] != pattern[j] {
                j = next[j - 1];
            }

            // 1.2.2 相等, 则 j + 1
            if text[i] == pattern[j] {
                j += 1;
            }

            // 如果 j == n，说明匹配成功
            // j 是模式串的长度, 匹配起始位置是 i + 1 - n
            if j == n {
                // 多匹配模式
                // result.push((i + 1 - n) as i32);  // 返回匹配起始下标
                // j = next[j - 1]; // 继续寻找下一个匹配
                return (i + 1 - n) as i32;
            }
        }

        -1
    }
}
