/*!
  2536. 子矩阵元素加 1
  地址: https://leetcode.cn/problems/increment-submatrices-by-one/description/
  问题: 给你一个正整数 n ，表示最初有一个 n x n 、下标从 0 开始的整数矩阵 mat ，矩阵中填满了 0
       另给你一个二维整数数组 query 。针对每个查询 query[i] = [row1i, col1i, row2i, col2i] ，请你执行下述操作:
       找出 `左上角` 为 `(row1i, col1i)` 且 `右下角` 为 `(row2i, col2i)` 的子矩阵，将子矩阵中的 `每个元素` 加 `1` 。也就是给所有满足 `row1i <= x <= row2i` 和 `col1i <= y <= col2i` 的 `mat[x][y]` 加 1
       返回执行完所有操作后得到的矩阵 mat 。

  提示:
     1. 1 <= n <= 500
     2. 1 <= queries.length <= 10的4次方
     3. 0 <= row1i <= row2i < n
     4. 0 <= col1i <= col2i < n

  解题思路:
    使用二维差分数组
    口诀:
     主对角线相加,
     负对角线相减,
     右下点在外边,
     负对角线全在外

    还原矩阵:
     左边 + 上边 - 左上角重叠区域 + D中当前值

  时间复杂度
    - 单次操作(区间加)：`O(1)`
    - 最终还原原数组：`O(n * m)` 前缀和一次过

  空间复杂度
    - O(n * m), 因为要维护 `原矩阵` + `差分矩阵`
*/

pub struct Difference;

impl Difference {
    // 构造二维差分数组
    fn build(v: i32, query: Vec<i32>, diff: &mut Vec<Vec<i32>>) {
        let x1 = query[0];
        let y1 = query[1];
        let x2 = query[2];
        let y2 = query[3];

        // 主对角线相加, 右下点在外边
        diff[x1 as usize][y1 as usize] += v; // 主对角线顶点
        diff[(x2 + 1) as usize][(y2 + 1) as usize] += v; // 主对角线右下角(在外边)

        // 负对角线相减, 负对角线全在外
        diff[x1 as usize][(y2 + 1) as usize] -= v; // 右上(右边一列清除)
        diff[(x2 + 1) as usize][y1 as usize] -= v; // 左下(下边一行清除)
    }

    // 还源矩阵
    fn restore(diff: Vec<Vec<i32>>, n: i32) -> Vec<Vec<i32>> {
        if diff.is_empty() {
            return Vec::new();
        }

        // 左边 + 上边 - 左上角重叠区域 + D中当前值
        let mut mat = vec![vec![0; n as usize]; n as usize];
        for i in 0..n as usize {
            for j in 0..n as usize {
                let mut v = diff[i][j];

                // 左边
                if i > 0 {
                    v += mat[i - 1][j];
                }

                // 上边
                if j > 0 {
                    v += mat[i][j - 1]
                }

                // 左上角重叠区域
                if i > 0 && j > 0 {
                    v -= mat[i - 1][j - 1];
                }

                mat[i][j] = v;
            }
        }

        mat
    }

    pub fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if n == 0 || queries.len() == 0 {
            return Vec::new();
        }

        let k = 1;

        // 构造二维差分数组
        // 用 `(n + 2) * (n + 2) 防止越界, 当然也可以用 `(n + 1) * (n + 1)`, 但需要判断是否越界
        let mut diff = vec![vec![0; (n + 2) as usize]; (n + 2) as usize];
        for query in queries {
            // 计算差分矩阵 D
            Self::build(k, query, &mut diff);
        }

        Self::restore(diff, n)
    }
}
