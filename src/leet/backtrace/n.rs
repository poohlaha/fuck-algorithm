pub struct Solution;

impl Solution {
    /**
      51. N 皇后
      地址: https://leetcode.cn/problems/n-queens/
      题目: 按照国际象棋的规则，皇后可以攻击与之处在同一行或同一列或同一斜线上的棋子。
           `n 皇后问题` 研究的是如何将 n 个皇后放置在 n×n 的棋盘上，并且使皇后彼此之间不能相互攻击。
            给你一个整数 n ，返回所有不同的 `n 皇后问题` 的解决方案。
            每一种解法包含一个不同的 `n 皇后问题` 的棋子放置方案，该方案中 'Q' 和 '.' 分别代表了皇后和空位。
      解:
         1. 从第 0 行开始深度放置皇后
         2. 当前行的第一列尝试放置:
            - 检查当前列和两条对角线上是否有皇后
            - 如有冲突, 撤销(继续下一列)
            - 如果无冲突, 放置皇后, 进入下一行
         3. 重复步骤 2, 直到:
            - 放到第 N 行, 表示已放置完 N 个皇后, 记录结果
            - 或者当前行已无可放列, 返回上一步(回溯), 撤销上一步的放置并继续尝试下一列
         4. 不断回溯 + 尝试直到遍历完所有状态

      如何识别主对角线:
        对于任意格子(row, col): row - col = 常数
        如:
          - (0,0), (1,1), (2,2), ... 都有 row - col = 0
          - (0,1), (1,2), (2,3) 都有 row - col = -1
          - (1,0), (2,1), (3,2) 都有 row - col = 1
        取值:
        row: 0 ~ n - 1
        col: 0 ~ n - 1
        最小值: row - col = 0 - (n - 1) = -(n - 1)
        最大值: row - col = n - 1 - 0 = n - 1
        所以: row - col: [-(n - 1), n - 1]
        共: -(n - 1) 到 0 到 n - 1 -> (n - 1) + (n - 1) + 1 = 2n - 1
      确定最大最小值:
       - row 增大会使值变大
       - col 增大会使值变小(因为要减去 col)
      所以:
      要最小，就让 row 尽量小，col 尽量大
      要最大，就让 row 尽量大，col 尽量小
      偏移:
       数组不能使用负下标, 需要将范围 `[-(n - 1), n - 1]` 平衡 `n - 1`
       [0, 2n - 2]
       现在 2n - 2 + 1 = 2n - 1 -> 2n - 1
       所以: 对于(row, col) 数组索引为: row - col + (n - 1)(取值)

     如何识别副对角线:
      对于任意格子(row, col): row + col = 常数
      如:
         - (0,2), (1,1), (2,0), ... 都有 row + col = 2
         - (0,3), (1,2), (2,1) 都有 row + col = 3
         - (1,2), (2,1), (3,0) 都有 row + col = 3
      取值:
      row: 0 ~ n - 1
      col: 0 ~ n - 1
      最小值: row + col: 0 - 0 = 0
      最大值: row + col: (n - 1) + (n - 1) = 2n -2
      所以: row + col: [0, 2n - 2]
      确定最大最小值:
        - row 增大会使值变大
        - col 增大会使值变大(因为要加上 col)
      所以:
        最小: row 尽量小，col 尽量小
        最大: row 尽量大，col 尽量大
      无需偏移:
        2n - 2 - 0 + 1 = 2n - 1
      所以: 对于(row, col) 数组索引为: row + col(取值)
    */
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        if n == 0 {
            return Vec::new();
        }

        let m = n as usize;
        let mut results: Vec<Vec<String>> = Vec::new();
        let mut board = vec![vec!['.'; m]; m]; // 棋盘

        let mut cols = vec![false; m]; // 列占用情况
        let mut diag1 = vec![false; 2 * m - 1]; // 主对角线占用情况, 左上 -> 右下
        let mut diag2 = vec![false; 2 * m - 1]; // 副对角线占用情况, 右上 -> 左下

        fn backtrace(
            row: usize,
            n: usize,
            board: &mut Vec<Vec<char>>,
            cols: &mut Vec<bool>,
            results: &mut Vec<Vec<String>>,
            diag1: &mut Vec<bool>,
            diag2: &mut Vec<bool>,
        ) {
            // 满足条件
            if row == n {
                let solution = board
                    .iter()
                    .map(|r| r.iter().collect::<String>())
                    .collect::<Vec<String>>();
                results.push(solution);
                return;
            }

            for col in 0..n {
                // 剪枝
                // 检查当前列和两条对角线上是否有皇后
                // 主对角线 row - col + n - 1
                // 副对角线: row + col
                let d1 = row + n - 1 - col; // 主对角线偏移
                let d2 = row + col; // 副对角线

                if cols[col] || diag1[d1] || diag2[d2] {
                    continue;
                }

                // 做选择
                board[row][col] = 'Q';
                cols[col] = true;
                diag1[d1] = true;
                diag2[d2] = true;

                // 递归探索
                backtrace(row + 1, n, board, cols, results, diag1, diag2);

                // 撤销选择
                board[row][col] = '.';
                cols[col] = false;
                diag1[d1] = false;
                diag2[d2] = false;
            }
        }

        backtrace(
            0,
            m,
            &mut board,
            &mut cols,
            &mut results,
            &mut diag1,
            &mut diag2,
        );
        results
    }
}
