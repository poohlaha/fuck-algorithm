use std::cmp::max;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct BM {
    text: String,
    pattern: String,
    m: usize,              // pattern 长度
    h: HashMap<char, i32>, // 坏字符表
    suffix: Vec<i32>, // 好后续表 suffix, 记录模式串中后缀子串在模式串中其他位置出现的起始索引(除末尾外), 没有出现记为 `-1`
    prefix: Vec<bool>, // 好后续表 prefix, 记录后缀子串是否是模式串的前缀(`true` 或 `false`)
}

impl BM {
    pub fn new(text: &str, pattern: &str) -> Self {
        if text.is_empty() || pattern.is_empty() {
            return BM::default();
        }

        let chars: Vec<char> = pattern.chars().collect();
        let m = chars.len();

        // 构造坏字符表, 字符在最后一次出现的索引位置
        let mut h: HashMap<char, i32> = HashMap::new();
        for (i, c) in chars.iter().enumerate() {
            h.insert(*c, i as i32);
        }

        // 构造 suffix 和 prefix
        let mut suffix = vec![-1; m];
        let mut prefix = vec![false; m];

        // 从最后一个字符向前遍历
        for i in (0..m - 1).rev() {
            let mut j = i as i32;
            let mut k = 0; // 匹配长度
                           // 比较 j 位置和 i ~ m-1 的值
                           // 比较 pattern[j] 和 pattern[m - 1 - k] 是否相等
            while j >= 0 && chars[j as usize] == chars[m - 1 - k] {
                // 匹配成功
                j -= 1; // 向前走一步
                k += 1; // 长度 + 1
            }

            if k > 0 {
                if j < 0 {
                    // 匹配到了开头
                    prefix[k] = true;
                }

                suffix[k] = j + 1;
            }
        }

        Self {
            text: text.to_string(),
            pattern: pattern.to_string(),
            m,
            h,
            suffix,
            prefix,
        }
    }

    pub fn query(&self, text: &str, pattern: &str, allow_overlap: bool) -> i32 {
        if text.is_empty() || pattern.is_empty() {
            return -1;
        };

        // 从 `右向左` 比对 `pattern` 和 `text` 的窗口，失败就使用 `坏字符` 和 `好后缀` 决定下一步滑动距离
        let texts: Vec<char> = text.chars().collect();
        let patterns: Vec<char> = pattern.chars().collect();

        let len = texts.len() as i32;
        let m = patterns.len();
        let mut count = 0;

        //  取出 `text[i..m - 1]`(`m` 为 `pattern` 的 `长度 - 1`) 与 `pattern` 比较
        let mut i: i32 = 0;
        let m = m as i32;

        // `从模式串末尾开始向前匹配` 字符
        while i < len {
            let mut matched = true;
            if i + m > len {
                break;
            }

            let l = i + (m - 1);
            for j in (i..=l).rev() {
                let pi = j - i;
                let t = texts.get(j as usize);
                let p = patterns.get(pi as usize);

                if t.is_none() || p.is_none() {
                    continue;
                }

                let t = t.unwrap();
                let p = p.unwrap();

                // 相同则继续匹配
                if p == t {
                    continue;
                }

                matched = false;

                // 不相同触发坏字符和好后续规则
                // 查坏字符表 H[t]
                let step = self.h.get(t).unwrap_or(&-1);
                // j - A
                let x = max(1, pi - *step); // 对齐字符 t

                // 匹配好后续规则
                let mut y = 0;
                let k = m - 1 - pi;
                let suffix_k = self.suffix.get(k as usize).unwrap_or(&-1);
                let prefix_k = self.prefix.get(k as usize).unwrap_or(&false);
                if *suffix_k != -1 {
                    // 好后续规则1: 查找 suffix[k] 是否为 -1
                    // y = j - suffix[k] + 1
                    y = (j as i32) - suffix_k + 1;
                } else if *prefix_k {
                    // 好后续规则2: 查找 prefix[k] 是否为 true
                    // y = m - k
                    y = m - k;
                } else {
                    // 以上条件全部不满足: y = m
                    y = m;
                }

                // 取较大值 `max(x, y)`, 将模式串 `右移对应步数`
                i += max(x, y);
                break;
            }

            if matched {
                count += 1;
                // 是否允许重叠
                i += if allow_overlap { 1 } else { m };
            }
        }

        count
    }
}
