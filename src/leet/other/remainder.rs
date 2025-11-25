/**
   1015. 可被 K 整除的最小整数
   力扣: https://leetcode.cn/problems/smallest-integer-divisible-by-k/description/
   题目: 给定正整数 k ，你需要找出可以被 k 整除的、仅包含数字 1 的最 小 正整数 n 的长度。
        返回 n 的长度。如果不存在这样的 n ，就返回-1。
        注意： n 可能不符合 64 位带符号整数。

   思路: 使用 `余数递推`
   解: (10 x r + 1) % k = 0
      ps: 任何以数字 1 结尾的十进制整数都是奇数, 奇数不可能被 2 整除。

   时间复杂度: O(k)
   空间复杂度: O(1)
*/
pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
    if k < 0 {
        return -1;
    }

    let k = k as usize;

    // 如果 k 与 10 不互质（即 k 含 2 或 5），则全 1 的数永远不会被整除
    // 如果 k 能被 2 或 5 整除，那么没有任何全 1 数能整除它
    // 全 1 数永远是奇数 ⇒ 不能整除 2 的倍数
    // 全 1 数结尾永远是 1 ⇒ 不能整除 5(一个数若要能被 5 整除，末位必须是 0 或 5)
    if k % 2 == 0 || k % 5 == 0 {
        return -1;
    }

    let mut remainder = 0usize;

    for length in 1..=k {
        remainder = (remainder * 10 + 1) % k;
        if remainder == 0 {
            return length as i32;
        }
    }

    -1
}
