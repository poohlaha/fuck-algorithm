/*!
  6. Z 字形变换
  力扣: https://leetcode.cn/problems/zigzag-conversion/
  题目: 将一个给定字符串 s 根据给定的行数 numRows ，以从上往下、从左到右进行 Z 字形排列。
       比如输入字符串为 "PAYPALISHIRING" 行数为 3 时，排列如下:
        P   A   H   N
        A P L S I I G
        Y   I   R
        之后，你的输出需要从左往右逐行读取，产生出一个新的字符串，比如："PAHNAPLSIIGYIR"。

  解: Z 字变形, 从上到下, 再斜着折返
*/

pub fn convert(s: String, num_rows: i32) -> String {
    if s.is_empty() || num_rows == 0 {
        return String::new();
    }

    if num_rows == 1 {
        return s;
    }

    let n = num_rows as usize;
    let mut rows = vec![String::new(); num_rows as usize];
    let chars: Vec<char> = s.chars().collect();

    let mut current_row = 0;
    let mut going_down = false;

    for &ch in chars.iter() {
        rows[current_row].push(ch);

        // 向下 | 向上折返
        if current_row == 0 || current_row == n - 1 {
            going_down = !going_down;
        }

        if going_down {
            current_row += 1;
        } else {
            current_row -= 1;
        }
    }

    rows.concat()
}
