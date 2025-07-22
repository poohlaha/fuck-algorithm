/*!
  10. 正则表达式匹配
  力扣: https://leetcode.cn/problems/regular-expression-matching/description/
  题目: 给你一个字符串 s 和一个字符规律 p，请你来实现一个支持 '.' 和 '*' 的正则表达式匹配。
       '.' 匹配任意单个字符
       '*' 匹配零个或多个前面的那一个元素
       所谓匹配，是要涵盖 整个 字符串 s 的，而不是部分字符串。

  解: 使用 [动态规则](src/core/dp/practise.md)
*/

pub fn is_match(s: String, p: String) -> bool {
    if p.is_empty() && s.is_empty() {
        return true;
    }

    let s_bytes = s.as_bytes();
    let p_bytes = p.as_bytes();
    let m = s.len();
    let n = p.len();

    let mut dp = vec![vec![false; n + 1]; m + 1];

    // 边界条件(初始化)
    dp[0][0] = true;

    // 预处理：空字符串和模式匹配的情况，例如 s="" p="a*" "a*b*"
    // i = 0，固定表示 s 为空字符串
    for j in 1..=n {
        if p_bytes[j - 1] == b'*' && j >= 2 {
            dp[0][j] = dp[0][j - 2];
        }
    }

    for i in 1..=m {
        for j in 1..=n {
            // 1. 当 p[j - 1] != '*' 时
            if p_bytes[j - 1] != b'*' {
                // 1.1 s[i - 1] = p[j - 1] 或 p[j - 1] = '.', dp[i][j] = dp[i - 1][j - 1]
                if s_bytes[i - 1] == p_bytes[j - 1] || p_bytes[j - 1] == b'.' {
                    dp[i][j] = dp[i - 1][j - 1]
                }
            } else {
                // 2. 当 p[j - 1] == '*' 时
                // 2.1 * 表示前一个字符出现 0 次, 跳过 p[j - 2] 和 p[j - 1]
                if j >= 2 {
                    dp[i][j] = dp[i][j - 2];
                }

                // 2.2 * 表示前一个字符出现 至少 1 次, s[i - 1] == p[j - 2] 或 p[j - 2] == '.'
                if j >= 2 && i >= 1 && (p_bytes[j - 2] == s_bytes[i - 1] || p_bytes[j - 2] == b'.')
                {
                    dp[i][j] |= dp[i - 1][j];
                }
            }
        }
    }

    dp[m][n]
}
