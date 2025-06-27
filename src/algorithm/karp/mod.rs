use std::collections::{HashMap, HashSet};

#[derive(Debug, Default)]
pub struct RabinKarp {
    text: String,
    pattern: String,
    m: usize,
    n: usize,
    p1: u64, // 基数 p(常选小质数，如 31 或 101，用于加权字符位置)
    q1: u64, // 模数 q(通常选大质数，如 10^9+7，防止哈希值溢出)
    p2: u64,
    q2: u64,
    hash1: u64,
    hash2: u64,
}

/// 预定义一组质数候选，按升序排列
const PRIMES: &[u64] = &[31, 53, 101, 131, 257, 509, 1009, 10007, 50021];

impl RabinKarp {
    pub fn new(text: &str, pattern: &str) -> Self {
        if text.is_empty() || pattern.is_empty() {
            return RabinKarp::default();
        }

        // 获取最大字符
        let max_char_val = text.bytes().chain(pattern.bytes()).max().unwrap_or(0);
        let (p1, p2) = Self::choose_tow_p(max_char_val);

        let patterns: Vec<char> = pattern.chars().collect();
        let texts: Vec<char> = text.chars().collect();

        let m = patterns.len();
        let n = texts.len();

        let q1 = 1_000_000_007u64;
        let q2 = 1_000_000_009u64;

        //  用一个哈希函数 `h(s)` 映射字符串 `s` 为 `数字`
        // hash_p = Σ pattern[i] * p^(m-1 - i)(i 从 0 到 m-1)
        let hash1 = Self::get_hash(&patterns, m, p1, q1);
        let hash2 = Self::get_hash(&patterns, m, p2, q1);

        Self {
            text: text.to_string(),
            pattern: pattern.to_string(),
            m,
            n,
            p1,
            p2,
            q1,
            q2,
            hash1,
            hash2,
        }
    }

    // 获取基数 p
    fn choose_tow_p(max_char_val: u8) -> (u64, u64) {
        let mut p1 = 0;
        let mut p2 = 0;
        for &prime in PRIMES {
            if prime > max_char_val as u64 {
                if p1 == 0 {
                    p1 = prime;
                } else if p2 == 0 {
                    p2 = prime;
                }
            }
        }

        if p1 == 0 {
            p1 = 1000003
        }

        if p2 == 0 {
            p2 = 1000033
        }

        (p1, p2)
    }

    // as u32就能获取字符的Unicode码点(十进制整数)
    fn convert_unicode_num(c: char) -> u32 {
        c as u32
    }

    // 获取初始化 hash, 防止多次开销, 直接存前一个数
    // 多次取模块不影响正确性, 模运算在加法、乘法下与原数同余
    fn get_hash(patterns: &Vec<char>, m: usize, p: u64, q: u64) -> u64 {
        let mut powers = vec![1u64; m];

        for i in 1..m {
            powers[i] = ((powers[i - 1]) * p) % q
        }

        let mut hash = 0;
        for (i, &c) in patterns.iter().enumerate() {
            let num = Self::convert_unicode_num(c);
            let power = powers[m - 1 - i];
            hash = (hash + (num as u64 * power) % q) % q;
        }

        hash
    }

    // 获取匹配的 hash
    fn get_match_hash(
        prev_hash: u64,
        prev_start_value: u64,
        new_value: Option<&char>,
        p: u64,
        q: u64,
    ) -> u64 {
        if let Some(new_value) = new_value {
            return (((prev_hash - prev_start_value) * p) % q
                + (Self::convert_unicode_num(*new_value) as u64) * p.pow(0))
                % q;
        }

        0
    }

    pub fn query(&self) -> Vec<usize> {
        if self.text.is_empty() || self.pattern.is_empty() {
            return Vec::new();
        }

        let mut index_list: Vec<usize> = Vec::new();
        let texts: Vec<char> = self.text.chars().collect();
        let patterns: Vec<char> = self.pattern.chars().collect();

        let mut map: HashMap<usize, (u64, u64, u64)> = HashMap::new();
        for i in 0..=(self.n - self.m) {
            let text: Vec<char> = texts[i..i + self.m].to_vec();
            // 计算 hash: hash = (Σ text[i + j] * p^(m - 1 - j)) % q

            let mut hash1 = 0u64;
            let mut hash2 = 0u64;
            let mut first_char_val = 0u64;
            if i == 0 {
                hash1 = Self::get_hash(&text, self.m, self.p1, self.q1);
                hash2 = Self::get_hash(&text, self.m, self.p2, self.q2);
                first_char_val = Self::convert_unicode_num(text[i]) as u64;
            } else {
                let prev = map.get(&(i - 1));
                if let Some((prev_start_value, prev_hash1, prev_hash2)) = prev {
                    hash1 = Self::get_match_hash(
                        *prev_hash1,
                        *prev_start_value,
                        text.last(),
                        self.p1,
                        self.q1,
                    );
                    hash2 = Self::get_match_hash(
                        *prev_hash2,
                        *prev_start_value,
                        text.last(),
                        self.p2,
                        self.q2,
                    );
                } else {
                    hash1 = Self::get_hash(&text, self.m, self.p1, self.q1);
                    hash2 = Self::get_hash(&text, self.m, self.p2, self.q2);
                }

                first_char_val = Self::convert_unicode_num(text[i - 1]) as u64;
            }

            map.insert(i, (first_char_val, hash1, hash2));

            // 判断双哈希是否相等, 如果相等, 从左向右比较字符
            let mut matched = true;
            if hash1 == self.hash1 && hash2 == self.hash2 {
                for j in 0..self.m {
                    if text[j] != patterns[j] {
                        matched = false;
                        break;
                    }
                }

                if matched {
                    index_list.push(i);
                }
            }
        }

        index_list
    }

    // 给定一个字符串，找出所有长度为 10 的子串中重复出现的片段
    /**
     187. 区间加法
     地址: https://leetcode.cn/problems/repeated-dna-sequences/description/
     题目: DNA序列 由一系列核苷酸组成，缩写为 'A', 'C', 'G' 和 'T'
          例如，"ACGAATTCCG" 是一个 DNA序列
          在研究 DNA 时，识别 DNA 中的重复序列非常有用
          给定一个表示 DNA序列 的字符串 s ，返回所有在 DNA 分子中出现不止一次的 长度为 10 的序列(子字符串)。你可以按 任意顺序 返回答案

     解:
       字符集为 4(A/C/G/T), 可以用 2 位来表示一个字符
       长度为 10 的子串只需 2 * 10 = 20
       可以直接用一个 32 位整数作为哈希值(无碰撞)

       bitmask = (1 << 20) - 1 // 只保留低 20 位
       hash = ((hash << 2) | new_char_code) & bitmask

    */
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        if s.is_empty() {
            return Vec::new();
        }

        let m = 10u64;
        let n = s.len() as u64;

        if n < m {
            return Vec::new();
        }

        // 1. 获取非重复编码
        let unique_chars: HashSet<char> = s.chars().collect();
        let mut unique_vec: Vec<char> = unique_chars.into_iter().collect();

        // 排序
        unique_vec.sort();

        // 2. 判断需要多少位编码
        // let bits = (unique_vec.len() as f64).log2().ceil() as u64;
        // 可以直接改成 2
        let bits = 2;

        // 3. 分配编码
        let mut encode_map: HashMap<char, u32> = HashMap::new();
        for (i, c) in unique_vec.iter().enumerate() {
            encode_map.insert(*c, i as u32); // 存数字, 内存中会直接转成二进制 01 10 11 这种
        }

        // 3. 构建滚动哈希(bit 编码) `bitmask = (1 << (m * 4) - 1`, 其中 `m` 为 `pattern` 长度
        let bitmask = (1 << (bits * m)) - 1;

        // 4. 初始化 `hash = 0`, 遍历 `i` 从 `0 ～ n - m`(n 为 `text` 长度, m 为 `pattern` 长度), 取出 `text[i..i+m)`, 逐字遍历每个字符的 `hash`
        let texts: Vec<char> = s.chars().collect();

        // 创建一个 2^20 = 1,048,576 位的元素,
        let mut seen = vec![0u8; 1 << 20]; // 1M空间, 0=未出现, 1=出现1次, 2=已加入结果
        let mut results: HashSet<String> = HashSet::new();

        // 获取第一个窗口 hash
        let mut hash = 0u64;
        for i in 0..m as usize {
            // 计算 hash: ((hash << 2) | new_char_code) & bitmask
            let new_char_code = *encode_map.get(&texts[i]).unwrap_or(&0) as u64;
            hash = ((hash << bits) | new_char_code) & bitmask
        }

        seen[hash as usize] = 1;

        for i in 1..=(n - m) as usize {
            // 计算 hash: ((hash << 2) | new_char_code) & bitmask
            let new_char_code = *encode_map.get(&texts[i + (m - 1) as usize]).unwrap_or(&0) as u64;
            hash = (hash << bits | new_char_code) & bitmask;

            //  查找 seen 中是否存在 text
            match seen[hash as usize] {
                0 => {
                    seen[hash as usize] = 1;
                }
                1 => {
                    results.insert(s[i..i + m as usize].to_string());
                    seen[hash as usize] = 2; // 已记录
                }
                2 => {
                    // 已加入结果，忽略
                }
                _ => {}
            }
        }

        results.into_iter().collect()
    }
}
