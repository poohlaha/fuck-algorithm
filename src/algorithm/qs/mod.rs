use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct QuickSearch {
    text: String,
    pattern: String,
    m: usize,
    shift: HashMap<char, usize>,
}

impl QuickSearch {
    pub fn new(text: &str, pattern: &str) -> QuickSearch {
        if text.is_empty() || pattern.is_empty() {
            return QuickSearch::default();
        }

        let chars: Vec<char> = pattern.chars().collect();
        let m = chars.len();

        // 构建 `shift` 数组
        let mut shift: HashMap<char, usize> = HashMap::new();
        for (i, ch) in chars.iter().enumerate() {
            shift.insert(*ch, m - i);
        }

        QuickSearch {
            text: text.to_string(),
            pattern: pattern.to_string(),
            m,
            shift,
        }
    }

    pub fn search(&self) -> Vec<usize> {
        if self.text.is_empty() || self.pattern.is_empty() {
            return Vec::new();
        };

        let mut results: Vec<usize> = Vec::new();
        let texts: Vec<char> = self.text.chars().collect();
        let patterns: Vec<char> = self.pattern.chars().collect();

        let len = self.m + 1;

        let mut i = 0;
        while i + self.m <= texts.len() {
            // 从主串 `text` 的位置 `i = 0` 开始向右滑动窗口
            let r = i + self.m;

            // 比较 `窗口字符` 和整个 `pattern` 是否匹配
            if &texts[i..r] == patterns.as_slice() {
                results.push(i);
            }

            // 观察 `text[i + m]`(`窗口后一位字符`)
            let c = texts.get(r);
            if let Some(c) = c {
                // 用这个字符查 `shift[c]`，然后让 `i += shift[c]`
                let step = self.shift.get(c).unwrap_or(&len);
                i += *step;
            } else {
                break;
            }
        }

        results
    }
}
