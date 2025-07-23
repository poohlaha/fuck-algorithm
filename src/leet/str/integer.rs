/*!
     7. 整数反转
     力扣: https://leetcode.cn/problems/reverse-integer/description/
     题目: 给你一个 32 位的有符号整数 x ，返回将 x 中的数字部分反转后的结果。
          如果反转后整数超过32位的有符号整数的范围 [-2^31, 2^31 - 1], 就返回 0。
          假设环境不允许存储64位整数(有符号或无符号)
    解:
      使用逆序, 在求逆序前先求绝对值, 注意 i32::MAX 和 i32::MIN 越界

    参考: src/leet/link/sum.rs
*/

use std::cmp::min;

pub fn reverse(x: i32) -> i32 {
    if x > i32::MAX || x < i32::MIN || x == 0 || x == -0 {
        return 0;
    }

    // 是否负数
    let is_negative_number = x < 0;
    // 取绝对值
    let mut num = x.abs();

    let mut res: i32 = 0;
    while num != 0 {
        let digit = num % 10; // 余数
        num /= 10;

        if res > i32::MAX / 10 || res < i32::MIN / 10 {
            return 0;
        }

        res = res * 10 + digit
    }

    if is_negative_number {
        -res
    } else {
        res
    }
}

/**
    8. 字符串转换整数(atoi)
    力扣: https://leetcode.cn/problems/string-to-integer-atoi/description/
    题目: 请你来实现一个 myAtoi(string s) 函数，使其能将字符串转换成一个 32 位有符号整数。
         函数 myAtoi(string s) 的算法如下:
         1. 空格: 读入字符串并丢弃无用的前导空格（" "）
         2. 符号：检查下一个字符（假设还未到字符末尾）为 '-' 还是 '+'。如果两者都不存在，则假定结果为正
         3. 转换：通过跳过前置零来读取该整数，直到遇到非数字字符或到达字符串的结尾。如果没有读取数字，则结果为0
         3. 舍入：如果整数数超过 32 位有符号整数范围 [−231,  231 − 1] ，需要截断这个整数，使其保持在这个范围内。具体来说，小于 −231 的整数应该被舍入为 −231 ，大于 231 − 1 的整数应该被舍入为 231 − 1
*/
pub fn my_atoi(s: String) -> i32 {
    if s.is_empty() || s == "+" || s == "-" {
        return 0;
    }

    let bytes = s.as_bytes(); // 避免使用 Vec 分配内存
    let mut i = 0;
    let n = bytes.len();

    // 1. 跳过前导空格
    while i < n && bytes[i] == b' ' {
        i += 1;
    }

    if i == n {
        return 0;
    }

    // 2. 检查符号
    let mut sign = 1;
    if bytes[i] == b'-' {
        sign = -1;
        i += 1;
    } else if bytes[i] == b'+' {
        i += 1;
    }

    // 3. 解析数字
    let mut res: i32 = 0;
    while i < n && bytes[i].is_ascii_digit() {
        let digit = (bytes[i] - b'0') as i32;

        // 检查是否溢出
        if res > (i32::MAX - digit) / 10 {
            return if sign == 1 { i32::MAX } else { i32::MIN };
        }

        res = res * 10 + digit;
        i += 1;
    }

    res * sign
}
