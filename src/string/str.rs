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

/// 查找所有字母异位词
/**
  把 t 字符串中的字符存入到集合A中, 设置左、右、有效指针, 初始化的值为 0, 设置集合 B, 用于存放 t 中字符值的出现个数
  1. 读取集合中的字符, 当存在于集合中 A 中时, B 集合统计出现次数, 当 B 集合中出现的次数 = A集中的次数时, 有效指针加 1
  2. 当左指针 - 右指针 >= 集合 A 的长度时
   1) 当左指针 - 右指针 = 集合 A 的长度, 说明满足要求, 存起来
   2) 读取左指针对应的字符, 当存在于集合中A中时, B 集合统计出现次数, 当 B 集合中出现的次数 = A集中的次数时, 有效指针减 1,
      当存在于集合中A中时, 集合中出现次数滅 1, 然后收缩窗口(左指针加 1), 直到不再包含集合A中的所有字符
**/
pub(crate) fn find_anagrams(s: &str, t: &str) -> Vec<usize> {
    if s.is_empty() || t.is_empty() {
        return Vec::new();
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
    let mut result: Vec<usize> = Vec::new();

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

        // 判断左侧窗口是否要收缩
        while right - left >= target.len() {
            if validate == target.len() {
                result.push(left);
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

    result
}

/// 最长无重复子串
/**
设置左、右、有效指针、最大长度、开始位置、结束位置, 初始化的值为 0, 设置集合 A, 用于存放 t 中字符值的出现个数
1. 读取集合中的字符, 把字符存入集合 A 中, 并统计出现的次数
2. 当集合 A 中出现次数 > 1 个时, 获取左指针对应字符, 集合 A 中字符减 1, 然后收缩窗口(左指针加 1)
3. 当 右指针 - 左指针 > 最大长度时, 设置最大长度 = 右指针 - 左指针, 并设置开始位置 = 左指针, 结束位置 = 右指针
 **/
pub(crate) fn length_of_longest_sub_string(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }

    let mut window: HashMap<char, usize> = HashMap::new(); // 待搜索字符串, 用来存储当前窗口中每个字符出现的次数
    let mut left = 0; // 左指针
    let mut right = 0; // 右指针
    let mut max_length = 0;
    let mut start = 0;
    let mut end = 0;

    let chars: Vec<char> = s.chars().collect();
    while right < chars.len() {
        let c = chars[right];
        right += 1;

        *window.entry(c).or_insert(0) += 1; // 存在则插入并增加1, 不存在则插入并增加1

        while window[&c] > 1 {
            let d = chars[left];
            left += 1;

            window.entry(d).and_modify(|x| *x -= 1);
        }

        if right - left > max_length {
            max_length = right - left;
            start = left;
            end = right;
        }
    }

    s[start..end].to_string()
}

/// 反转字符串
/**
  使用左右指针
  1. 左指针初始值为 0, 右指针初始化值为 最后一个数
  2. 当左指针值 < 右指针值时, 交换两索引位置
**/
pub(crate) fn reverse_str(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }

    let mut chars: Vec<char> = s.chars().collect();
    let mut left = 0;
    let mut right = chars.len() - 1;

    while left < right {
        chars.swap(left, right);
        left += 1;
        right -= 1;
    }

    chars.iter().collect()
}

/// 回文串判断
/**
使用左右指针, 左指针指向开头, 右指针指向结果, 同时相向移动指针，判断是否相等, 如果有一个值不相等，那第就不是回文串
回文串: level, aba, abba 等
 **/
pub(crate) fn is_palindrome(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    let mut chars: Vec<char> = s.chars().collect();
    let mut left = 0;
    let mut right = chars.len() - 1;

    while left <= right {
        let value1 = chars.get(left).unwrap().clone();
        let value2 = chars.get(right).unwrap().clone();
        if value1 != value2 {
            return false;
        }
        left += 1;
        right -= 1;
    }

    true
}

/// 查找最长回文串
/**
 从某个字符开始向两边扩散查找
 如果输入相同的 left 和 right，就相当于寻找长度为奇数的回文串
 如果输入相邻的 left 和 right，则相当于寻找长度为偶数的回文串。
*/
pub(crate) fn longest_palindrome(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }

    let mut chars: Vec<char> = s.chars().collect();
    let palindrome = |l: usize, r: usize| -> String {
        let mut left = l.clone() as i32;
        let mut right = r.clone() as i32;

        if left == 0 && right == 0 {
            return s[l..=r].to_string();
        }

        let len = (s.len() - 1) as i32;
        while left >= 0
            && right <= len
            && chars.get(left as usize).unwrap().clone()
                == chars.get(right as usize).unwrap().clone()
        {
            left -= 1;
            right += 1;
        }

        let ll = (left + 1) as usize;
        let rr = right as usize;
        s[ll..rr].to_string() // 右边不包含
    };

    // 如果输入相同的 left 和 right，就相当于寻找长度为奇数的回文串
    // 如果输入相邻的 left 和 right，则相当于寻找长度为偶数的回文串。
    let mut result = String::new();
    for i in 0..chars.len() {
        let s1 = palindrome(i, i); // 以当前字符为中心，回文串为奇数长度
        let s2 = palindrome(i, i + 1); // 以当前字符和下一个字符为中心，回文串为偶数长度
        result = if result.len() > s1.len() { result } else { s1 };
        result = if result.len() > s2.len() { result } else { s2 }
    }

    result
}
