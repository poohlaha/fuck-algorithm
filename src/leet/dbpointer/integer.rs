/**
   9. 回文数
   力扣: https://leetcode.cn/problems/palindrome-number/description/
   题目: 给你一个整数 x ，如果 x 是一个回文整数，返回 true ；否则，返回 false 。
        回文数是指正序（从左向右）和倒序（从右向左）读都是一样的整数。
        例如，121 是回文，而 123 不是。

   解: 负数不是回文数, 单个数字是回文数, 其他转成字符串使用双指针
*/
pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    if x < 10 {
        return true;
    }

    let s = x.to_string();
    let (mut left, mut right) = (0, s.len() - 1);
    let bytes = s.as_bytes();

    while left < right {
        if bytes[left] != bytes[right] {
            return false;
        }

        left += 1;
        right -= 1;
    }

    true
}
