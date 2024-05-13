//! 斐波那契数列
/**
斐波那契数列是指这样一个数列：1、1、2、3、5、8、13、21、34、……在数学上，斐波那契数列以递归的方法定义：
F{1} = 1, F{2} = 1, F(n) = F(n - 1) + F(n - 2)(n >= 3)
斐波那契数列中的每个数等于其前两个数的和，而第一个和第二个数均为 1
**/

/// 暴力递归
pub(crate) fn fib(n: u32) -> u32 {
    if n == 1 || n == 2 {
        return 1;
    }

    return fib(n - 1) + fib(n - 2);
}

/// 带备忘录的递归解法
pub(crate) fn dp_fib(n: u32) -> u32 {
    let max = n as usize;
    let mut memo = vec![0u32; max + 1]; // 斐波那契数列的长度为 n 时，数组 memo 中应该包含 n 个元素来存储每个位置的斐波那契数值

    fn dp(memo: &mut Vec<u32>, n: u32) -> u32 {
        if n == 0 || n == 1 {
            return n;
        }

        if memo[n as usize] != 0 {
            return memo[n as usize];
        }

        memo[n as usize] = dp(memo, n - 1) + dp(memo, n - 2);
        memo[n as usize]
    }

    dp(&mut memo, n)
}
