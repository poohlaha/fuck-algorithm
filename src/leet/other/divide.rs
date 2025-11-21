/**
  29. 两数相除
  力扣: https://leetcode.cn/problems/divide-two-integers/description/
  题目: 给你两个整数，被除数 dividend 和除数 divisor。将两数相除，要求 不使用 乘法、除法和取余运算。
       整数除法应该向零截断，也就是截去（truncate）其小数部分。例如，8.345 将被截断为 8 ，-2.7335 将被截断至 -2 。
       返回被除数 dividend 除以除数 divisor 得到的 商 。
       注意：假设我们的环境只能存储 32 位 有符号整数，其数值范围是 [−231,  231 − 1] 。本题中，如果商 严格大于 231 − 1 ，则返回 231 − 1 ；如果商 严格小于 -231 ，则返回 -231 。
*/
pub fn divide(dividend: i32, divisor: i32) -> i32 {
    if divisor == 0 {
        panic!("除数不能为0")
    }

    const INT_MAX_I64: i64 = i32::MAX as i64; //  2147483647
    const INT_MIN_I64: i64 = i32::MIN as i64; // -2147483648

    if divisor == 0 {
        return i32::MAX;
    }

    // 记录符号
    // 一个是负，一个是正 → true(结果为负)
    // 两个都为正或都为负 → false(结果为正)
    let negative = (dividend < 0) ^ (divisor < 0);

    let mut dividend = (dividend as i64).abs();
    let mut divisor = (divisor as i64).abs();
    let mut result: i64 = 0;

    while dividend >= divisor {
        let mut temp = divisor;
        let mut multiple = 1;

        // 找到最大的 temp <= dividend 的 2^n * divisor, 即最大倍数
        // 左移一位，就是把所有位向左挪一格，右边补 0，相当于 ×2。
        while dividend >= (temp << 1) {
            temp <<= 1; // << 1 就是乘以 2
            multiple <<= 1;
        }

        dividend -= temp;
        result += multiple;
    }

    let mut signed: i64 = if negative { -result } else { result };

    // 裁剪到 32 位范围内（题目要求）
    if signed > INT_MAX_I64 {
        INT_MAX_I64 as i32
    } else if signed < INT_MIN_I64 {
        INT_MIN_I64 as i32
    } else {
        signed as i32
    }
}
