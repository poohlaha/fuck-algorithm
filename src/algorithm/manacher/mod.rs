use std::cmp::min;

pub struct Manacher;

impl Manacher {
    // 最长回文串
    pub fn longest_palindrome(s: String) -> String {
        if s.is_empty() {
            return String::new();
        }

        // 1. 在每个字符中间插入特殊符号 `#`, 让字符串长度统一变成 `奇数`, 方便处理两种回文长度(奇数/偶数)
        let mut str = String::from("#");
        for c in s.chars() {
            str.push(c);
            str.push('#');
        }

        // 2. 定义辅助数组 P
        let t: Vec<char> = str.chars().collect();
        let len = t.len();
        let mut p = vec![0; len];

        // 3. 主过程(利用对称性加速)
        // 3.1 维护两变量 `C`、`R`, 初始化都为 `0`, 逐步更新
        let mut c = 0;
        let mut r = 0;

        // 开始遍历
        for i in 0..len {
            // 判断 i < R 是否成立
            if i < r {
                // `i' = 2 * C - i`(i关于 C 的对称点)
                // 初始 `P[i] = min(P[i'], R - i)`
                p[i] = min(p[2 * c - i], r - i);
            }

            // i >= R -> 暴力扩展

            // 从p[i]已经知道的扩展长度开始暴力扩展
            let mut index = p[i] + 1;

            while i >= index && i + index < len && t[i - index] == t[i + index] {
                p[i] += 1;
                index += 1;
            }

            // 扩展需要更新 c 和 r
            if i + p[i] > r {
                c = i;
                r = i + p[i];
            }
        }

        // 4. 获取结果
        // 4.1 找出 `P[i] 最大值` → 代表 `最长回文串的半径`
        let max = p
            .iter()
            .enumerate()
            .max_by_key(|&(_idx, &val)| val)
            .map(|(idx, &val)| (idx, val));
        if let Some((i, val)) = max {
            // start  = (i - P[i]) / 2
            // len    = P[i]
            // substr = original[start..start + len]
            // (T[i - P[i]], T[i + P[i])
            let start = (i - val) / 2;
            let len = val;
            let sub = &s[start..start + len];
            return sub.to_string();
        }

        String::new()
    }
}
