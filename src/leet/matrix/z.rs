/*!
  498. 对角线遍历
  力扣: https://leetcode.cn/problems/diagonal-traverse/description/
  题目: 给你一个大小为 m x n 的矩阵 mat ，请以对角线遍历的顺序，用一个数组返回这个矩阵中的所有元素。
  输入: mat = [[1,2,3],[4,5,6],[7,8,9]]
  输出: [1,2,4,7,5,3,6,8,9]

  输入: mat = [[1,2],[3,4]]
  输出: [1,2,3,4]

  解:
    矩阵类 `Z` 字变形
    沿对角线上升: 能上就上，遇边界优先右移，右不动再下移;沿对角线下降: 能下就下，遇边界优先下移，下不动再右移

    从左上角 (0,0) 开始
    沿对角线上升（向右上），到边界后切换为沿对角线下降（向左下）
    重复，直至遍历完整个矩阵
*/

pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
    let m = mat.len();
    if m == 0 {
        return Vec::new();
    }

    let n = mat[0].len();
    if n == 0 {
        return Vec::new();
    }

    if m == 1 {
        if let Some(first) = mat.get(0) {
            return first.clone();
        }

        return Vec::new();
    }

    let mut result = Vec::with_capacity(m * n);

    // 起点
    let (mut row, mut col) = (0, 0);
    let mut direction = 1; // 1: 沿对角线一直上升, -1: 沿对角线一直下降

    // - dir == 1 (向右上): 能右上就右上，遇边界优先右移，否则下移并切换方向
    // - dir == -1 (向左下): 能左下就左下，遇边界优先下移，否则右移并切换方向
    for _ in 0..m * n {
        result.push(mat[row][col]);

        // 沿对角线一直上升(从左下往右上)
        if direction == 1 {
            if col == n - 1 {
                // 到达最右边边界，无法继续右上，向下移并切换方向
                row += 1; // 不能右移, 只能下移
                direction = -1; // 沿对角线下降
            } else if row == 0 {
                // 在顶部, 只能往右走
                col += 1;
                direction = -1; // 沿对角线下降
            } else {
                // 继续往右上走
                row -= 1;
                col += 1;
            }
        } else {
            // 沿对角线一直下降(从右上往左下)
            if row == m - 1 {
                // 到达底部边界，无法继续左下，向右移并切换方向
                col += 1; // 不能下降, 只能右移
                direction = 1; // 沿对角线上升
            } else if col == 0 {
                // 到达最左边界，向下移并切换方向
                row += 1; // 下降
                direction = 1; // 沿对角线上升
            } else {
                // 继续往左下走
                row += 1;
                col -= 1;
            }
        }
    }

    result
}
