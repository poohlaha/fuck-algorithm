/*!
  字典树(Dictionary Tree)
*/
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct TrieNode {
    children: HashMap<char, TrieNode>, // 子节点
    is_end: bool,                      // 是否是敏感词结尾
}

impl TrieNode {
    pub fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_end: false,
        }
    }

    // 向 Trie 中插入一个敏感词
    pub fn insert(&mut self, word: &str) {
        let mut node = self;
        for c in word.chars() {
            // 如果当前字符不存在, 则创建新节点
            node = node.children.entry(c).or_insert(TrieNode::default());
        }

        node.is_end = true;
    }

    // 匹配敏感词
    pub fn match_from(&self, text: &[char], start: usize) -> usize {
        let mut node = self;
        let mut index = start;
        while index < text.len() {
            let ch = text[index];
            // 继续匹配子节点
            if let Some(next_node) = node.children.get(&ch) {
                node = next_node;
                // 如果匹配到敏感词末尾, 返回当前匹配的长度
                if node.is_end {
                    return index - start + 1;
                }

                index += 1;
            } else {
                // 没有匹配到, 停止
                break;
            }
        }

        0
    }
}

pub struct Trie;

impl Trie {
    // 将文本中匹配到的 `敏感词` 替换为 `*`
    pub fn replace_sensitive_words(sensitive_words: &[String], text: &str) -> String {
        // 1. 构建 Trie
        let mut trie = TrieNode::new();
        // 2. 向 Trie 中插入敏感词, 此处在实际过程中可使用并行任务插入
        // 在实际使用中, Trie 是放在内存中的, 应该是全局统一设置, 使用 lazy_static
        for word in sensitive_words {
            trie.insert(word);
        }

        // 3. 转换文本为向量字符
        let mut chars: Vec<char> = text.chars().collect();

        // 4. 遍历文本, 从每个位置尝试匹配敏感词
        let mut i = 0;
        while i < chars.len() {
            let match_len = trie.match_from(&chars, i);
            if match_len > 0 {
                // 4.1. 匹配到敏感词：将该区间替换为 *
                for j in i..i + match_len {
                    chars[j] = '*';
                }

                // 4.2. 移动指针到敏感词后的下一个位置
                i += match_len;
            } else {
                i += 1;
            }
        }

        // 5. 返回替换后的字符串
        chars.iter().collect()
    }
}
