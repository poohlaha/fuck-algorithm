/**
  474. 一和零
  力扣:
  题目: 给你一个二进制字符串数组 strs 和两个整数 m 和 n 。
       请你找出并返回 strs 的最大子集的长度，该子集中 最多 有 m 个 0 和 n 个 1 。
       如果 x 的所有元素也是 y 的元素，集合 x 是集合 y 的 子集 。

  思路: 典型的 0-1 背包问题的二维版本(二维 0-1 背包)
  解:
     1. 判断是否可用 DP
        1.1 是否有“最优子结构”？
            一个问题的最优解可以由子问题的最优解构造出来。
            如果选择了 s → 剩余容量变为 (i - zeros(s), j - ones(s))
            如果不选择 s → 剩余容量保持 (i, j)

        1.2 是否有“重叠子问题”？
            子问题会被多次计算，如果直接递归会重复做同样的事。
            对同一组字符串和容量 (i, j)，可能会在不同决策路径上重复出现

        1.3 是否无后效性？
            当前子问题的最优解只依赖于当前状态，不依赖之前如何到达该状态的路径
            当前剩余容量 (i, j) 的最优解只取决于剩余容量和剩余字符串，而 不关心之前选过哪些路径

     2. 定义状态(State)
        状态是子问题的标识，用来唯一描述子问题，让我们知道当前计算的“场景”是什么。

        获取状态:
        在剩余容量 i 个 0、j 个 1 的条件下, 最多能选多少个字符串
        dp[i][j] = 在剩余 i 个 0, j 个 1 的情况下, 最多能选取的字符串数量

     3. 列出选择(Choice)
        在当前状态下，我们可以做出的决策或操作，通常对应“选”或“不选”。
        选择 s →  可以从子问题 dp[i - zeros(s)][j - ones(s)] 转移过来, 再 +1
        不选择 s → 此时 dp[i][j] 保持不变

     4. 写出转移方程(递推公式)
         将“选择”转化为最优性方程: dp[i][j] = max(dp[i][j], dp[i - zeros(s)][j - ones(s)] + 1), i >= zeros(s) & j >= ones(s)

     5. 边界条件(初始化)
        dp[i][j] = 0

     6. 计算顺序
        因为 dp[i][j] = max(dp[i][j], dp[i - zeros(s)][j - ones(s)]) + 1, i >= zeros(s) & j >= ones(s), 计算 dp[i][dpj] 时依赖 dp[i - zeros(s)][j - ones(s)], 因此:
        - 必须倒序遍历 i 和 j，保证 dp[i - zeros(s)][j - ones(s)] 在计算 dp[i][j] 前已被计算
        - 外层循环: i 从 m 倒序 到 zeros, i = zeros ..= m
        - 内层循环: j 从 n 倒序 到 n, j = ones ..= n

     7. 返回结果
        最终要求 `dp[i][j] = 在剩余 i 个 0、j 个 1 的情况下，最大可选字符串数量`, 因些返回：
        dp[m][n]

   时间复杂度: O(L * m * n)
   空间复杂度: O(m * n)
*/
pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
    if strs.is_empty() {
        return 0;
    }

    if m == 0 && n == 0 {
        return 0;
    }

    let mut dp = vec![vec![0; n as usize + 1]; m as usize + 1];

    for s in strs {
        let zeros = s.chars().filter(|&c| c == '0').count();
        let ones = s.len() - zeros;

        for i in (zeros as i32..=m).rev() {
            for j in (ones as i32..=n).rev() {
                dp[i as usize][j as usize] = dp[i as usize][j as usize]
                    .max(dp[(i - zeros as i32) as usize][(j - ones as i32) as usize] + 1);
            }
        }
    }

    dp[m as usize][n as usize]
}
