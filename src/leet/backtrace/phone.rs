/*!
   17. 电话号码的字母组合
   力扣: https://leetcode.cn/problems/letter-combinations-of-a-phone-number/description/
   题目: 给定一个仅包含数字 2-9 的字符串，返回所有它能表示的字母组合。答案可以按 任意顺序 返回。
        给出数字到字母的映射如下（与电话按键相同）。注意 1 不对应任何字母。
*/

pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return Vec::new();
    }

    let mapping = vec![
        "",     // 0
        "",     // 1
        "abc",  // 2
        "def",  // 3
        "ghi",  // 4
        "jkl",  // 5
        "mno",  // 6
        "pqrs", // 7
        "tuv",  // 8
        "wxyz", // 9
    ];

    let mut results = vec![];
    let mut path = String::new();

    fn backtrack(
        index: usize,
        digits: &Vec<char>,
        mapping: &Vec<&str>,
        path: &mut String,
        results: &mut Vec<String>,
    ) {
        if index == digits.len() {
            results.push(path.clone());
            return;
        }

        // 把 `字符数字` 转成 `数字`
        let digit = digits[index].to_digit(10).unwrap_or(0) as usize;
        for ch in mapping[digit].chars() {
            // 做选择
            path.push(ch);
            backtrack(index + 1, digits, mapping, path, results);

            // 撤销选择
            path.pop();
        }
    }

    backtrack(
        0,
        &digits.chars().collect(),
        &mapping,
        &mut path,
        &mut results,
    );

    results
}
