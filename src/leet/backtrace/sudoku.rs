/**
  37. 解数独
  力扣: https://leetcode.cn/problems/sudoku-solver/description/
  题目: 编写一个程序，通过填充空格来解决数独问题。
       数独的解法需 遵循如下规则：
       1. 数字 1-9 在每一行只能出现一次。
       2. 数字 1-9 在每一列只能出现一次。
       3. 数字 1-9 在每一个以粗实线分隔的 3x3 宫内只能出现一次。（请参考示例图）
          数独部分空格内已填入了数字，空白格用 '.' 表示。

      时间复杂度: O(9ᵏ)
      空间复杂度: O(1)
*/
pub struct Sudoku;

impl Sudoku {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        if board.len() == 0 {
            return;
        }

        let m = board.get(0).unwrap().len();
        let n = board.len();

        if m == 0 || n == 0 {
            return;
        }

        Self::backtrack(board, m, n);
    }

    fn backtrack(board: &mut Vec<Vec<char>>, m: usize, n: usize) -> bool {
        for row in 0..m {
            for col in 0..n {
                // 已有数字，跳过
                if board[row][col] != '.' {
                    continue;
                }

                for ch in '1'..='9' {
                    if Self::validate(board, row, col, ch, m, n) {
                        // 放入 ch
                        board[row][col] = ch;

                        // 如果成功填完整个棋盘，则返回 true
                        if Self::backtrack(board, m, n) {
                            return true;
                        }

                        // 否则，撤销选择（回溯）
                        board[row][col] = '.';
                    }
                }

                return false;
            }
        }

        true
    }

    // 检查 (row, col) 放置数字 ch 是否有效
    fn validate(
        board: &mut Vec<Vec<char>>,
        row: usize,
        col: usize,
        ch: char,
        m: usize,
        n: usize,
    ) -> bool {
        // 行检查
        for c in 0..m {
            if board[row][c] == ch {
                return false;
            }
        }

        // 列检查
        for r in 0..n {
            if board[r][col] == ch {
                return false;
            }
        }

        // 3 x 3 宫格检查
        // 每个坐标起点: (row / 3) x 3, (col / 3) x 3
        let block_row = (row / 3) * 3;
        let block_col = (col / 3) * 3;

        for r in block_row..block_row + 3 {
            for c in block_col..block_col + 3 {
                if board[r][c] == ch {
                    return false;
                }
            }
        }

        true
    }

    pub fn print(nums: &Vec<Vec<char>>) {
        for row in nums {
            println!("{:?}", row);
        }
    }
}
