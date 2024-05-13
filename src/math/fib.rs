//! 斐波那契数列
/**
斐波那契数列是指这样一个数列：1、1、2、3、5、8、13、21、34、……在数学上，斐波那契数列以递归的方法定义：
F{1} = 1, F{2} = 1, F(n) = F(n - 1) + F(n - 2)(n >= 3)
斐波那契数列中的每个数等于其前两个数的和，而第一个和第二个数均为 1

自顶而下:
 - 使用递归的方式解决问题，通常从问题的顶部开始，逐步向下解决子问题，直到达到基本情况（边界条件）。
 - 递归过程中，通常会使用记忆化搜索（Memoization）来避免重复计算

自底向上:
- 使用迭代的方式解决问题，通常从最小的子问题开始，逐步计算出所有的子问题的解，直到达到目标问题的解。
- 通常不使用递归，而是使用循环来迭代计算。

**/

/// 暴力递归
pub(crate) fn fib(n: u32) -> u32 {
    if n == 1 || n == 2 {
        return 1;
    }

    return fib(n - 1) + fib(n - 2);
}

/// 带备忘录的递归解法(自顶向下)
/**
带「备忘录」的递归算法，把一棵存在巨量冗余的递归树通过「剪枝」，改造成了一幅不存在冗余的递归图，极大减少了子问题（即递归图中节点）的个数。
*/
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

/// dp 数组的迭代（递推）解法(自底而上)
pub(crate) fn db_cycle_fib(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }

    let max = n as usize;
    let mut memo = vec![0u32; max + 1]; // 斐波那契数列的长度为 n 时，数组 memo 中应该包含 n 个元素来存储每个位置的斐波那契数值
    memo[0] = 1;
    memo[1] = 1;

    for i in 2..=n {
        let x1 = i - 1;
        let x2 = i - 2;
        memo[i as usize] = memo[x1 as usize] + memo[x2 as usize];
    }

    memo[(n - 1) as usize]
}

/// 常用, 优化空间复杂度为O(1)
pub(crate) fn db_normal_fib(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return n;
    }

    // 从 0 开始
    let mut dp_i_1 = 1;
    let mut dp_i_2 = 0;
    for i in 2..=n {
        let dp_i = dp_i_1 + dp_i_2;
        dp_i_2 = dp_i_1;
        dp_i_1 = dp_i
    }

    dp_i_1
}
