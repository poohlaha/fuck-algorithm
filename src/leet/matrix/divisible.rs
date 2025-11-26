/**
   2435. 矩阵中和能被 K 整除的路径
   力扣: https://leetcode.cn/problems/paths-in-matrix-whose-sum-is-divisible-by-k/description
   题目: 给你一个下标从 0 开始的 m x n 整数矩阵 grid 和一个整数 k 。你从起点 (0, 0) 出发，每一步只能往 下 或者往 右 ，你想要到达终点 (m - 1, n - 1) 。
        请你返回路径和能被 k 整除的路径数目，由于答案可能很大，返回答案对 109 + 7 取余 的结果。

   思路:
        1. 使用回溯算法, 坐标和相加最后 % k = 0
        ps: 当 grid 过大时, 使用回溯算法会导致超出时间限制

       2. 使用动态规则, 三维数组

       假设: matrix = 100 x 100, k = 50
       时间: 100 * 100 & 50 = 500000
       回溯: 10^58
       DP: 500000
       差了 50 个数量级

   DP:
      时间复杂度: O(m * n * k)
      空间复杂度: O(n * k)
*/
pub fn number_of_paths_not_recommended(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    if grid.is_empty() || k == 0 {
        return 0;
    }

    let m = grid.len();
    let n = grid[0].len();
    let mut count = 0;

    fn backtrace(
        grid: &Vec<Vec<i32>>,
        row: i32,
        col: i32,
        current: i64,
        k: i64,
        count: &mut i32,
        m: i32,
        n: i32,
    ) {
        let val = grid[row as usize][col as usize];
        let new_modulo = (current + val as i64) % k;

        // 到达终点
        if row == m - 1 && col == n - 1 {
            if new_modulo == 0 {
                *count += 1;
            }

            return;
        }

        // 向右
        if col + 1 < n {
            backtrace(grid, row, col + 1, new_modulo, k, count, m, n);
        }

        // 向下
        if row + 1 < m {
            backtrace(grid, row + 1, col, new_modulo, k, count, m, n);
        }
    }

    // 从 (0,0) 开始, current_mod = 0
    backtrace(&grid, 0, 0, 0, k as i64, &mut count, m as i32, n as i32);

    count
}

// dp[i][j][r] = 走到 (i,j) 时，路径和 mod k = r 的路径数量
// 每个格子存 3 个值, r 为当前格子 % k 的余数, 取值: 0 ~ k - 1, 每个格子会有 k 个“余数桶”
// 每一个格子 (i,j) 都对应 k 种可能的路径和取模结果
pub fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    if grid.is_empty() || k == 0 {
        return 0;
    }

    let m = grid.len();
    let n = grid[0].len();
    let k = k as usize;
    let modulo = 1_000_000_007; // 10^9 + 7

    let mut dp = vec![vec![vec![0i64; k]; n]; m];
    let start_mod = (grid[0][0] as usize) % k;
    dp[0][0][start_mod] = 1;

    for i in 0..m {
        for j in 0..n {
            if i == 0 && j == 0 {
                continue;
            }

            let val_mod = (grid[i][j] as usize) % k;
            for r in 0..k {
                // 上方
                if i > 0 {
                    let prev = dp[i - 1][j][r];
                    if prev > 0 {
                        // 到达当前格子时，路径和的余数, 范围 0 ~ k -1
                        let new_r = (r + val_mod) % k;
                        // dp = ((dp % MOD) + (prev % MOD)) % MOD => dp = (dp + prev) % MOD
                        dp[i][j][new_r] = (dp[i][j][new_r] + prev) % modulo; // 路径数量, 需要添加上一个格子的路径数, 再放置对应的桶中
                    }
                }

                // 左方
                if j > 0 {
                    let prev = dp[i][j - 1][r];
                    if prev > 0 {
                        let new_r = (r + val_mod) % k;
                        dp[i][j][new_r] = (dp[i][j][new_r] + prev) % modulo;
                    }
                }
            }
        }
    }

    // 能被 k 整除 → 路径和 % k == 0, 对应余数 r = 0 的桶
    dp[m - 1][n - 1][0] as i32
}

/**
   滚动二维 DP
   一行是“上一行”，用来参考; 一行是“当前行”，用来存新的结果
   每次换行时，必须擦掉当前行，否则上一行的数据会污染新的结果

  grid:
       | 5 2 4 |
       | 3 0 5 |
       | 0 7 2 |
  k: 3

  初始化:
     起点 (0,0)：val = 5 → val % 3 = 2
     dp[0][0] = [0, 0, 1] -- 余数桶 [0, 1, 2]

  1. 第一行 i = 0
     清空当前行: 除了 (0,0) 其他格子都 0

     j = 0:
     dp[0][0] = [0, 0, 1]

     j = 1:
     val = 2 → val % 3 = 2
     左边: dp[0][0] = [0, 0, 1]
     遍历 r = 0 .. 2

     r            dp[0][0][r]     new_r = (r + 2) % 3      dp[0][1][new_r]
     0            0               2                        0 + 0 = 0
     1            0               0                        0 + 0 = 0
     2            1               1                        0 + 1 = 1

     此时:
     dp[0][1][0] = 0
     dp[0][1][1] = 1
     dp[0][1][2] = 0

     最终: dp[0][1] = [0, 1, 0]

     j = 2
     val = 4 → val % 3 =1
     左边: dp[0][1] = [0, 1, 0]
     ...
     dp[0][2] = [0, 0, 1]

     ...
*/
pub fn number_of_paths_scroll(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    if grid.is_empty() || k == 0 {
        return 0;
    }

    let m = grid.len();
    let n = grid[0].len();
    let k = k as usize;
    let modulo = 1_000_000_007;

    // dp[2][n][k] 滚动数组：0表示上一行，1表示当前行
    let mut dp = vec![vec![vec![0i64; k]; n]; 2];

    // 起点初始化
    for i in 0..m {
        // 2 表示两行, 上一行(prev) 和 当前行(cur)
        // % 2 是为了在 0 和 1 之间切换, 节省空间
        let cur = i % 2; // 当前行存放结果
        let prev = 1 - cur; // 上一行存放上一行结果

        // 清空当前行
        for j in 0..n {
            for r in 0..k {
                dp[cur][j][r] = 0;
            }
        }

        for j in 0..n {
            let val_mod = (grid[i][j] as usize) % k;

            // 起点特殊处理
            if i == 0 && j == 0 {
                dp[cur][j][val_mod] = 1;
                continue;
            }

            for r in 0..k {
                // 上方格子
                if i > 0 {
                    let prev_val = dp[prev][j][r];
                    if prev_val > 0 {
                        let new_r = (r + val_mod) % k;
                        dp[cur][j][new_r] = (dp[cur][j][new_r] + prev_val) % modulo;
                    }
                }

                // 左方格子
                if j > 0 {
                    let prev_val = dp[cur][j - 1][r];
                    if prev_val > 0 {
                        let new_r = (r + val_mod) % k;
                        dp[cur][j][new_r] = (dp[cur][j][new_r] + prev_val) % modulo;
                    }
                }
            }
        }
    }

    // 原始网格最后一行是 i = m - 1
    // 滚动 DP 用 % 2 来切换两行
    // 终点所在行是 m - 1，滚动数组索引 = (m - 1) % 2
    dp[(m - 1) % 2][n - 1][0] as i32
}
