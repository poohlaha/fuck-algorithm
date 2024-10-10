//! 回溯问题

/**
 解数独, 球模型
 力扣: https://leetcode.cn/problems/sudoku-solver/description/
 题目:
     编写一个程序，通过填充空格来解决数独问题。
     数独的解法需 遵循如下规则：
     1. 数字 `1-9` 在每一行只能出现一次。
     2. 数字 `1-9` 在每一列只能出现一次。
     3. 数字 `1-9` 在每一个以粗实线分隔的 `3 x 3` 宫内只能出现一次。（请参考示例图）
     数独部分空格内已填入了数字，空白格用 '.' 表示。

     5 3 . . 7 . . . .
     6 . . 1 9 5 . . .
     . 9 8 . . . . 6 .
     8 . . . 6 . . . 3
     4 . . 8 . 3 . . 1
     7 . . . 2 . . . 6
     . 6 . . . . 2 8 .
     . . . 4 1 9 . . 5
     . . . . 8 . . 7 9

  答:
     1. 确定 3x3 方框的左上角位置
        对于任意位置 (i, j)，它所在的 3x3 方框的左上角坐标可以通过以下公式计算得到：
        - 行起点：(i / 3) * 3
        - 列起点：(j / 3) * 3
        如: 当 (i, j) = (4, 4) 时：
           - (i / 3) * 3 = (4 / 3) * 3 = 1 * 3 = 3
           - (j / 3) * 3 = (4 / 3) * 3 = 1 * 3 = 3
           所以，方框的左上角坐标为 (3, 3)。
     2. 遍历 3x3 方框内的所有元素
        要遍历 3x3 方框内的 9 个格子，我们需要在左上角的基础上偏移行列。x 在 0..9 的范围内逐渐增大，表示方框中每个元素的位置。
        - 行偏移：x / 3
        - 列偏移：x % 3

     完整公式为：
      - 行：(i / 3) * 3 + x / 3
      - 列：(j / 3) * 3 + x % 3

     示例:
      假设 i = 4, j = 4, ch = "5", n = 9:
      1. 计算方框的左上角：(i / 3) * 3 = 3，(j / 3) * 3 = 3。
      2. 遍历 3x3 方框内的 9 个元素时：
       - 当 x = 0 时，位置为 (3 + 0 / 3, 3 + 0 % 3) = (3, 3)
       - 当 x = 1 时，位置为 (3 + 1 / 3, 3 + 1 % 3) = (3, 4)
       - 当 x = 2 时，位置为 (3 + 2 / 3, 3 + 2 % 3) = (3, 5)
       - 当 x = 3 时，位置为 (4, 3)
       - ...
       - 当 x = 8 时，位置为 (5, 5)
      这样，x 依次遍历了 3x3 方框内的每个位置，通过 board[(i / 3) * 3 + x / 3][(j / 3) * 3 + x % 3] 访问到了每个格子的内容，方便检查是否存在重复的 ch。

     确定 3 * 3, 以某个值中心点向外放射出 `行 3` * `列3`
     1. 确定 `左上角坐标`
        找到包含位置 (i, j) 的 3 x 3 区域的左上角, 为了找到包含 (6, 5) 的 3x3 方框，可以按以下方式确定左上角位置：
        - 行的起始：每个 3x3 方框从某一行的 3 倍数开始。例如，行 6 在 6 // 3 * 3 = 6 行开始。
        - 列的起始：每个 3x3 方框从某一列的 3 倍数开始。例如，列 5 在 5 // 3 * 3 = 3 列开始。
       所以，以 (6, 5) 为中心的 3x3 区域的左上角在 (6, 3)。
       - 左上角行索引：(i // 3) * 3 -- start_row
       - 左上角列索引：(j // 3) * 3 -- start_col
     2. 使用偏移量获取整个 3x3 方框
        从左上角 (start_row, start_col) 开始，逐步增加行列偏移，得到所有 3x3 区域的坐标。可以用两个嵌套的循环来完成：
       - 外层循环：控制行偏移，从 0 到 2。
       - 内层循环：控制列偏移，从 0 到 2。
       对于每一对 (row_offset, col_offset)，可以计算出每个坐标为:
       (row, col) = (start_row + row_offset, start_col + col_offset)
      例: 以下是 (i, j) = (6, 5) 的 3x3 区域的所有坐标:
          1. 左上角坐标 (start_row, start_col) = (6, 3)
          2. 使用行列偏移 (row_offset, col_offset) 从 0 到 2 遍历出所有坐标：
          ```txt
           (start_row + 0, start_col + 0) -> (6, 3)
           (start_row + 0, start_col + 1) -> (6, 4)
           (start_row + 0, start_col + 2) -> (6, 5)

           (start_row + 1, start_col + 0) -> (7, 3)
           (start_row + 1, start_col + 1) -> (7, 4)
           (start_row + 1, start_col + 2) -> (7, 5)

           (start_row + 2, start_col + 0) -> (8, 3)
           (start_row + 2, start_col + 1) -> (8, 4)
           (start_row + 2, start_col + 2) -> (8, 5)
          ```
          3. 得到的 3x3 区域坐标集
             最终得到的 (6, 5) 周围 3x3 区域的完整坐标集为：
           ```txt
           [
               (6, 3), (6, 4), (6, 5),
               (7, 3), (7, 4), (7, 5),
               (8, 3), (8, 4), (8, 5)
           ]
          ```
       最终公式: `(row, col) = ((i // 3) * 3 + row_offset, (j // 3) * 3 + col_offset)`
*/

pub(crate) fn solve_sudo_su(nums: &mut Vec<Vec<&str>>) {
    if nums.len() == 0 {
        return;
    }

    let m = nums.get(0).unwrap().len();
    let n = nums.len();

    if m == 0 || n == 0 {
        return;
    }

    fn backtrack(board: &mut Vec<Vec<&str>>, i: usize, j: usize, m: usize, n: usize) -> bool {
        // 穷举到最后一列, 换行重新开始
        if j == n {
            return backtrack(board, i + 1, 0, m, n);
        }

        // 穷举完了最后一行，完成了所有的穷举, 结束
        if i == m {
            return true;
        }

        // 如果是预设的数字, 则跳过
        if board[i][j] != "" {
            return backtrack(board, i, j + 1, m, n);
        }

        let chars = ["1", "2", "3", "4", "5", "6", "7", "8", "9"]; // 数字为 1 - 9
        for ch in &chars {
            // 判断是否合法
            if !validate(board, i, j, ch, n) {
                continue;
            }

            board[i][j] = ch;

            // 如果找到答案就结束, 做选择
            if backtrack(board, i, j + 1, m, n) {
                return true;
            }

            // 撤销选择
            board[i][j] = "";
        }

        false
    }

    // 3 * 3 需要循环 9 次
    fn validate(board: &mut Vec<Vec<&str>>, i: usize, j: usize, ch: &str, n: usize) -> bool {
        for x in 0..n {
            // 判断行是否存在重复
            if board[i][x] == ch {
                return false;
            }

            // 判断列是不是重复
            if board[x][j] == ch {
                return false;
            }

            // 判断 3 x 3 方框是否有重复, / 为向下取整, 行和列行偏移都是从 0 到 2
            // 行偏移 x / 3：当 x 在 0, 1, 2 时，x / 3 = 0，即第一行；当 x 在 3, 4, 5 时，x / 3 = 1，即第二行；当 x 在 6, 7, 8 时，x / 3 = 2，即第三行
            // 列偏移 x % 3：当 x 为 0, 3, 6 时，x % 3 = 0，即第一列；当 x 为 1, 4, 7 时，x % 3 = 1，即第二列；当 x 为 2, 5, 8 时，x % 3 = 2，即第三列
            // start_row + 0, start_col + (0, 1, 2) -- 第一行
            // start_row + 1, start_col + (0, 1, 2) -- 第二行
            // start_row + 2, start_col + (0, 1, 2) -- 第三行
            // row_offset: 0, 1, 2 --> x / 3
            // col_offset: 0, 1, 2 --> x % 3
            if board[(i / 3) * 3 + (x / 3)][(j / 3) * 3 + x % 3] == ch {
                return false;
            }
        }

        true
    }

    fn print(nums: &Vec<Vec<&str>>) {
        for row in nums {
            println!("{:?}", row);
        }

        println!();
    }

    backtrack(nums, 0, 0, m, n);
}
