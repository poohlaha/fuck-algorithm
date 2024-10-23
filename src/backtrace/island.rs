//!
//!
//!
/**!
 秒杀所有岛屿题目, 使用 DFS 或 BFS
 飞地: 岛屿被水域包围，且可以上下左右相连，斜对角的相邻不算相连

 普通岛屿：(关注的是陆地（1）)
     - 这是指由 1 组成的陆地。如果一个岛屿的任何部分（即任意一个 1）接触到网格的边界，它可能被称为开放岛屿。
     例如，在通常的岛屿问题中，任何与边界连接的陆地 1 都会导致整个岛屿被认为是开放的，因为它与外界连通。

 封闭岛屿:(关注的是水域（0）的封闭性)
     - 封闭岛屿指的是完全被 1 包围的水域 0，且该水域的任何部分都不能接触网格的边界。
     - 如果水域 0 有任何一部分接触到网格的边界（即 0 位于第一行、最后一行、第一列或最后一列），它就不是封闭的。
     - 封闭岛屿问题关注的是水域 0 的封闭性，而不是陆地 1 的封闭性。
*/
use std::collections::VecDeque;

/**
  飞地的数量
  力扣: https://leetcode.cn/problems/number-of-enclaves/description/
  题目: 给你一个大小为 m x n 的二进制矩阵 grid ，其中 0 表示一个海洋单元格、1 表示一个陆地单元格
      一次 移动 是指从一个陆地单元格走到另一个相邻（上、下、左、右）的陆地单元格或跨过 grid 的边界
      返回网格中 无法 在任意次数的移动中离开网格边界的陆地单元格的数量。

  0 0 0 0
  1 0 1 0
  0 1 1 0
  0 0 0 0
*/
pub(crate) fn islands_count(grid: Vec<Vec<char>>) -> u32 {
    let rows = grid.len();
    let mut count: u32 = 0;
    if rows == 0 {
        return count;
    }

    let cols = grid[0].len();
    let mut grid = grid;
    let mut visited: Vec<Vec<bool>> = vec![vec![false; cols]; rows];

    // 定义四个方向，分别是向下、向上、向右、向左
    let directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    fn dfs(
        grid: &mut Vec<Vec<char>>,
        visited: &mut Vec<Vec<bool>>,
        directions: &Vec<(i32, i32)>,
        row: i32,
        col: i32,
    ) {
        // 超出索引边界
        if row < 0 || col < 0 || row >= (grid.len() as i32) || col >= (grid[0].len() as i32) {
            return;
        }

        let row = row as usize;
        let col = col as usize;
        if visited[row][col] || grid[row][col] == '0' {
            return;
        }

        // 进入节点(row, col)
        visited[row][col] = true;

        // 递归遍历上下左右的节点
        for (dr, dc) in directions {
            let new_row = row as i32 + dr;
            let new_col = col as i32 + dc;
            dfs(grid, visited, &directions, new_row, new_col);
        }

        // 离开节点(row, col)
    }

    for row in 0..rows {
        if !grid[row].contains(&'1') {
            // 整行没有任何陆地
            continue;
        }

        for col in 0..cols {
            if grid[row][col] == '1' && !visited[row][col] {
                count += 1;
                dfs(&mut grid, &mut visited, &directions, row as i32, col as i32);
            }
        }
    }

    count
}

/// 使用 BFS
pub(crate) fn islands_count_by_bfs(grid: Vec<Vec<char>>) -> u32 {
    let rows = grid.len();
    let mut count: u32 = 0;
    if rows == 0 {
        return count;
    }

    let cols = grid[0].len();
    let mut grid = grid;
    let mut visited: Vec<Vec<bool>> = vec![vec![false; cols]; rows];

    // 定义四个方向，分别是向下、向上、向右、向左
    let directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    fn bfs(
        grid: &mut Vec<Vec<char>>,
        visited: &mut Vec<Vec<bool>>,
        directions: &Vec<(i32, i32)>,
        row: i32,
        col: i32,
    ) {
        // 超出索引边界
        if row < 0 || col < 0 || row >= (grid.len() as i32) || col >= (grid[0].len() as i32) {
            return;
        }

        let row = row as usize;
        let col = col as usize;

        if visited[row][col] || grid[row][col] == '0' {
            return;
        }

        let mut queue = VecDeque::new();
        queue.push_back((row, col));

        visited[row][col] = true;

        while let Some((row, col)) = queue.pop_front() {
            // 递归遍历上下左右的节点
            for (dr, dc) in directions {
                let new_row = row as i32 + dr;
                let new_col = col as i32 + dc;
                bfs(grid, visited, &directions, new_row, new_col);
            }
        }
    }

    for row in 0..rows {
        if !grid[row].contains(&'1') {
            // 整行没有任何陆地
            continue;
        }

        for col in 0..cols {
            if grid[row][col] == '1' && !visited[row][col] {
                count += 1;
                bfs(&mut grid, &mut visited, &directions, row as i32, col as i32);
            }
        }
    }

    count
}

/**
 统计封闭岛屿的数目(只关心水域)
 力扣: https://leetcode.cn/problems/number-of-closed-islands/description/
 题目: 二维矩阵 grid 由 0 （土地）和 1 （水）组成。岛是由最大的4个方向连通的 0 组成的群，封闭岛是一个 完全 由1包围（左、上、右、下）的岛。
      请返回 封闭岛屿 的数目。

 封闭岛屿: 完全被 1 包围的 0 区域，且任何部分都不接触网格的边界。如果有任何一个 0 接触到网格的边界（上下左右四个边框），这个岛屿就不是封闭的。

 1 1 1 1 1 1 1 0
 1 0 0 0 0 1 1 0
 1 0 1 0 1 1 1 0
 1 0 0 0 0 1 0 1
 1 1 1 1 1 1 1 0
*/
pub(crate) fn closed_islands_count(grid: Vec<Vec<char>>) -> u32 {
    let rows = grid.len();
    let mut count: u32 = 0;
    if rows == 0 {
        return count;
    }

    let cols = grid[0].len();
    let mut grid = grid;
    let mut visited: Vec<Vec<bool>> = vec![vec![false; cols]; rows];

    // 定义四个方向，分别是向下、向上、向右、向左
    let directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    // 只对未访问过的水域 0 进行遍历和标记
    fn dfs(
        grid: &mut Vec<Vec<char>>,
        visited: &mut Vec<Vec<bool>>,
        directions: &Vec<(i32, i32)>,
        row: i32,
        col: i32,
    ) {
        // 超出索引边界
        if row < 0 || col < 0 || row >= (grid.len() as i32) || col >= (grid[0].len() as i32) {
            return;
        }

        let row = row as usize;
        let col = col as usize;
        if visited[row][col] || grid[row][col] != '0' {
            return;
        }

        // 进入节点(row, col)
        visited[row][col] = true;

        // 递归遍历上下左右的节点
        for (dr, dc) in directions {
            let new_row = row as i32 + dr;
            let new_col = col as i32 + dc;
            dfs(grid, visited, &directions, new_row, new_col);
        }

        // 离开节点(row, col)
    }

    // 标记边界, 边界为 0 时不可能是封闭岛屿
    // 网格的四个边界：第一行、最后一行、第一列、最后一列上的 0 水域无论如何都不能是封闭的
    // 需要先标记这些水域以及它们相邻的所有水域
    // 标记这些边界的水域后，剩下未被标记的 0 就是潜在的封闭岛屿
    // 行边界
    for r in 0..rows {
        if grid[r][0] == '0' {
            dfs(&mut grid, &mut visited, &directions, r as i32, 0);
        }
        if grid[r][cols - 1] == '0' {
            dfs(
                &mut grid,
                &mut visited,
                &directions,
                r as i32,
                (cols - 1) as i32,
            );
        }
    }

    // 列边界
    for c in 0..cols {
        if grid[0][c] == '0' {
            dfs(&mut grid, &mut visited, &directions, 0, c as i32);
        }
        if grid[rows - 1][c] == '0' {
            dfs(
                &mut grid,
                &mut visited,
                &directions,
                (rows - 1) as i32,
                c as i32,
            );
        }
    }

    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            if grid[row][col] == '0' && !visited[row][col] {
                count += 1;
                dfs(&mut grid, &mut visited, &directions, row as i32, col as i32);
            }
        }
    }

    count
}

/**
  岛屿的最大面积
  力扣: https://leetcode.cn/problems/max-area-of-island/
  题目: 给你一个大小为 m x n 的二进制矩阵 grid 。
       岛屿 是由一些相邻的 1 (代表土地) 构成的组合，这里的「相邻」要求两个 1 必须在 水平或者竖直的四个方向上 相邻。你可以假设 grid 的四个边缘都被 0（代表水）包围着。
       岛屿的面积是岛上值为 1 的单元格的数目。
       计算并返回 grid 中最大的岛屿面积。如果没有岛屿，则返回面积为 0 。
*/
pub(crate) fn max_area_of_island(grid: Vec<Vec<char>>) -> u32 {
    let rows = grid.len();
    let mut count: u32 = 0;
    if rows == 0 {
        return count;
    }

    let mut max_are = 0;
    let cols = grid[0].len();
    let mut grid = grid;
    let mut visited: Vec<Vec<bool>> = vec![vec![false; cols]; rows];

    // 定义四个方向，分别是向下、向上、向右、向左
    let directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    fn dfs(
        grid: &mut Vec<Vec<char>>,
        visited: &mut Vec<Vec<bool>>,
        directions: &Vec<(i32, i32)>,
        row: i32,
        col: i32,
    ) -> u32 {
        // 超出索引边界
        if row < 0 || col < 0 || row >= (grid.len() as i32) || col >= (grid[0].len() as i32) {
            return 0;
        }

        let row = row as usize;
        let col = col as usize;
        if visited[row][col] || grid[row][col] == '0' {
            return 0;
        }

        // 进入节点(row, col)
        visited[row][col] = true;

        // 统计当前岛屿面积
        let mut area: u32 = 1;

        // 递归遍历上下左右的节点
        for (dr, dc) in directions {
            let new_row = row as i32 + dr;
            let new_col = col as i32 + dc;
            area += dfs(grid, visited, &directions, new_row, new_col);
        }

        // 离开节点(row, col)
        area
    }

    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == '1' && !visited[row][col] {
                let area = dfs(&mut grid, &mut visited, &directions, row as i32, col as i32);
                // max_are = max_are.max(area);
                max_are = std::cmp::max(max_are, area);
            }
        }
    }

    max_are
}

/**
  统计子岛屿
  力扣: https://leetcode.cn/problems/count-sub-islands/
  题目: 给你两个 m x n 的二进制矩阵 grid1 和 grid2 ，它们只包含 0 （表示水域）和 1 （表示陆地）。一个 岛屿 是由 四个方向 （水平或者竖直）上相邻的 1 组成的区域。任何矩阵以外的区域都视为水域。
       如果 grid2 的一个岛屿，被 grid1 的一个岛屿 完全 包含，也就是说 grid2 中该岛屿的每一个格子都被 grid1 中同一个岛屿完全包含，那么我们称 grid2 中的这个岛屿为 子岛屿 。
       请你返回 grid2 中 子岛屿 的 数目 。

   在 grid1 中，相应的陆地部分（即 grid1 中为 1 的位置）必须在 grid2 中也存在（即 grid2 中对应位置为 1）。
*/
pub(crate) fn count_sub_islands(grid1: Vec<Vec<char>>, grid2: Vec<Vec<char>>) -> u32 {
    if grid1.is_empty() || grid2.is_empty() {
        return 0;
    }

    let rows = grid1.len();
    let cols = grid1[0].len();
    let mut count = 0;

    if rows == 0 {
        return count;
    }

    let mut grid1 = grid1;
    let mut grid2 = grid2;
    let mut visited: Vec<Vec<bool>> = vec![vec![false; cols]; rows];

    // 定义四个方向，分别是向下、向上、向右、向左
    let directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    fn dfs(
        grid1: &mut Vec<Vec<char>>,
        grid2: &mut Vec<Vec<char>>,
        visited: &mut Vec<Vec<bool>>,
        directions: &Vec<(i32, i32)>,
        row: i32,
        col: i32,
        is_sub_island: &mut bool,
    ) {
        // 超出索引边界
        if row < 0 || col < 0 || row >= (grid1.len() as i32) || col >= (grid1[0].len() as i32) {
            return;
        }

        let row = row as usize;
        let col = col as usize;
        if visited[row][col] {
            return;
        }

        // grid2 中的单元格是水域
        if grid2[row][col] == '0' {
            return;
        }

        if grid1[row][col] == '0' {
            *is_sub_island = false;
        }

        // 进入节点(row, col)
        visited[row][col] = true;

        // 递归遍历上下左右的节点
        for (dr, dc) in directions {
            let new_row = row as i32 + dr;
            let new_col = col as i32 + dc;
            dfs(
                grid1,
                grid2,
                visited,
                &directions,
                new_row,
                new_col,
                is_sub_island,
            );
        }

        // 离开节点(row, col)
    }

    for row in 0..rows {
        for col in 0..cols {
            // 只有在 grid2 中是陆地时，才检查
            if grid2[row][col] != '0' && !visited[row][col] {
                let mut is_sub_island = true;
                dfs(
                    &mut grid1,
                    &mut grid2,
                    &mut visited,
                    &directions,
                    row as i32,
                    col as i32,
                    &mut is_sub_island,
                );
                if is_sub_island {
                    count += 1;
                }
            }
        }
    }

    count
}
