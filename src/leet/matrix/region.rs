/*!
   二维区域和检索-矩阵不可变:
   地址: https://leetcode.cn/problems/range-sum-query-2d-immutable/description/
   问题: 给定一个二维矩阵 `matrix`，以下类型的多个请求
        计算其子矩形范围内元素的总和，该子矩阵的 `左上角` 为 `(row1, col1)` ，`右下角` 为 `(row2, col2)`
        实现 NumMatrix 类:
        `NumMatrix(int[][] matrix)` 给定整数矩阵 `matrix` 进行初始化
        `int sumRegion(int row1, int col1, int row2, int col2)` 返回 `左上角 (row1, col1)` 、`右下角 (row2, col2)` 所描述的子矩阵的元素 `总和`

    条件:
      1. 一个二维矩阵 `matrix`(行数、列数均不固定, 非空)
      2. 多次查询: 每次查询一个子矩阵的和, 子矩阵由 `(row1, col1) 到 `(row2, col2)` 组成

    解题思路(二维前缀和):
      要支持 `多次查询子矩阵和`, 就需要提前进行 `预处理`, 减少每次查询的时间开销
      1. 使用二维前缀和数组 `prefixSum`
         prefixSum[i][j]: 表示从原矩阵左上角 `(0,0)` 到 `(i-1, j-1)` 之间所有元素的和
         维度: [m + 1][n + 1], 比原矩阵多一行一列，方便处理边界情况
      2. 构造前缀和
         prefixSum[i][j] = 上 + 左 - 左上角重叠区域 + 当前值
         prefixSum[i][j] = prefixSum[i - 1][j]
                         + prefixSum[i][j - 1]
                         - prefixSum[i - 1][j - 1]
                         + matrix[i - 1][j - 1]
      3. 查询子矩阵和的公式
         查询从 `(row1, col1)` 到 `(row2, col2)` 的子矩阵和时，用前缀和数组:
         sumRegion(row1, col1, row2, col2) = prefixSum[row2 + 1][col2 + 1]
                                           - prefixSum[row1][col2 + 1]
                                           - prefixSum[row2 + 1][col1]
                                           - prefixSum[row1][col1]
      4. 时间复杂度
         构造前缀和: O(m × n)
         每次查询: O(1)

      5. 空间复杂度
         O(m × n)(m 是矩阵的行数, n 是矩阵的列数)

*/

pub struct Region {
    prefix: Vec<Vec<i64>>,
}

impl Region {
    // 初始化前缀和矩阵
    pub fn new(matrix: Vec<Vec<i32>>) -> Self {
        let m = matrix.len();

        if m == 0 {
            return Self { prefix: Vec::new() };
        }

        let n = matrix[0].len();

        // 前缀和维度: (m + 1) × (n + 1)
        let mut prefix = vec![vec![0i64; n + 1]; m + 1];

        // 构造前缀和
        for i in 0..m {
            for j in 0..n {
                // 当前值 + 左边 + 上边 - 左上角重叠区域(P[i−1][j−1])
                prefix[i + 1][j + 1] =
                    (matrix[i][j] as i64) + prefix[i][j + 1] + prefix[i + 1][j] - prefix[i][j];
            }
        }

        Self { prefix }
    }

    // 查询子矩阵和, 从 (row1, col1) 到 (row2, col2)
    pub fn query(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i64 {
        if self.prefix.len() == 0 {
            return 0;
        }

        // 修正坐标
        let (r1, r2) = if row1 <= row2 {
            (row1 as usize, row2 as usize)
        } else {
            (row2 as usize, row1 as usize)
        };
        let (c1, c2) = if col1 <= col2 {
            (col1 as usize, col2 as usize)
        } else {
            (col2 as usize, col1 as usize)
        };

        // 在 P 中查询 (row1 + 1, col1 + 1) 到 `(row2 + 1, col2 + 1) 的区域
        // 整个区域 - 上方多余区域 - 左边多余区域 + 左上角重叠区域(被减了两次, 要加回来)
        // (row2 + 1, col2 + 1) - (row1, col2 + 1) - (row2 + 1, col1) + (row1, col1)
        self.prefix[r2 + 1][c2 + 1] - self.prefix[r1][c2 + 1] - self.prefix[r2 + 1][c1]
            + self.prefix[r1][c1]
    }
}
