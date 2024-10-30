//! BFS, 广度优先遍历, 层序遍历

use std::char::from_digit;

pub(crate) mod test;

/**
  解开密码锁的最少次数(打开转盘锁)
  力扣: https://leetcode.cn/problems/open-the-lock/description/
  题目: 你有一个带有四个圆形拨轮的转盘锁。每个拨轮都有10个数字： '0', '1', '2', '3', '4', '5', '6', '7', '8', '9' 。每个拨轮可以自由旋转：例如把 '9' 变为 '0'，'0' 变为 '9' 。每次旋转都只能旋转一个拨轮的一位数字。
       锁的初始数字为 '0000' ，一个代表四个拨轮的数字的字符串。
       列表 deadends 包含了一组死亡数字，一旦拨轮的数字和列表里的任何一个元素相同，这个锁将会被永久锁定，无法再被旋转。
       字符串 target 代表可以解锁的数字，你需要给出解锁需要的最小旋转次数，如果无论如何不能解锁，返回 -1 。
  答: 从 "0000" 开始，转一次，可以穷举出 "1000", "9000", "0100", "0900"... 共 8 种密码。然后，再以这 8 种密码作为基础，对每个密码再转一下，穷举出所有可能。
     可以抽象成一幅图，每个节点有 8 个相邻的节点，又让你求最短距离。
*/
pub(crate) fn open_lock(deadends: Vec<&str>, target: &str) -> i32 {
    if deadends.is_empty() || target.is_empty() {
        return -1;
    }

    let mut visited: Vec<String> = Vec::new(); // 记录已经穷举过的密码，防止走回头路
    let mut step = 0; // 从起点开始启动广度优先搜索
    let mut q: Vec<String> = vec![String::from("0000")]; // 从 0000 开始
    visited.push(String::from("0000"));

    while q.len() > 0 {
        for i in (0..q.len()).rev() {
            let cur = q.remove(i);

            // 判断是否包含死亡密码
            if deadends.contains(&cur.as_str()) {
                continue;
            }

            // 判断是否到达终点
            if cur == target {
                return step;
            }

            // 将节点的相邻节点加入队列
            for j in 0..4 {
                let up = plus_one(&cur, j);
                let down = minus_one(&cur, j);
                if !visited.contains(&up) {
                    q.push(up.clone());
                    visited.push(up.clone())
                }

                if !visited.contains(&down) {
                    q.push(down.clone());
                    visited.push(down.clone())
                }
            }
        }

        step += 1;
    }

    step
}

/// 双向 BFS
/// 传统的 BFS 框架就是从起点开始向四周扩散，遇到终点时停止；而双向 BFS 则是从起点和终点同时开始扩散，当两边有交集的时候停止。
/// 双向 BFS 其实只遍历了半棵树就出现了交集，也就是找到了最短距离
/// 顶部循环和顶部循环交替进行
pub(crate) fn bidirectional_open_lock(deadends: Vec<&str>, target: &str) -> i32 {
    if deadends.is_empty() || target.is_empty() {
        return -1;
    }

    let mut visited: Vec<String> = Vec::new(); // 记录已经穷举过的密码，防止走回头路
    let mut step = 0; // 从起点开始启动广度优先搜索
    let mut q1: Vec<String> = vec![String::from("0000")]; // 从 0000 开始
    let mut q2: Vec<String> = vec![String::from(target)]; // 从 target 开始

    while q1.len() > 0 && q2.len() > 0 {
        // 将 q1 中的所有节点向周围扩散
        let mut temp: Vec<String> = Vec::new();

        for i in (0..q1.len()).rev() {
            let cur = q1.remove(i);

            // 判断是否包含死亡密码
            if deadends.contains(&cur.as_str()) {
                continue;
            }

            if q2.contains(&cur) {
                return step;
            }

            visited.push(cur.clone());

            // 将一个节点的未遍历相邻节点加入集合
            for j in 0..4 {
                let up = plus_one(&cur, j);
                let down = minus_one(&cur, j);
                if !visited.contains(&up) {
                    temp.push(up);
                }

                if !visited.contains(&down) {
                    temp.push(down);
                }
            }
        }

        step += 1;
        q1 = q2; // 将 q2 的状态转移到 q1
        q2 = temp; // 将新生成的密码队列转移到 q2
    }

    step
}

// 将 s[j] 向上波动一次
fn plus_one(s: &str, j: i32) -> String {
    let size = j as usize;
    let new_char;
    if let Some(char) = s.chars().nth(size) {
        if char == '9' {
            new_char = '0';
        } else {
            let digit = char.to_digit(10).unwrap() + 1;
            new_char = from_digit(digit, 10).unwrap();
        }

        let mut str = String::from(s);
        str.replace_range(size..size + 1, new_char.to_string().as_str());
        return str;
    }

    return String::new();
}

// 将 s[j] 向下波动一次
fn minus_one(s: &str, j: i32) -> String {
    let size = j as usize;
    let new_char;
    if let Some(char) = s.chars().nth(size) {
        if char == '0' {
            new_char = '9';
        } else {
            let digit = char.to_digit(10).unwrap() - 1;
            new_char = from_digit(digit, 10).unwrap();
        }

        let mut str = String::from(s);
        str.replace_range(size..size + 1, new_char.to_string().as_str());
        return str;
    }

    return String::new();
}

/**
  滑动谜题
  力扣: https://leetcode.cn/problems/sliding-puzzle/description/
  题目: 在一个 2 x 3 的板上（board）有 5 块砖瓦，用数字 1~5 来表示, 以及一块空缺用 0 来表示。一次 移动 定义为选择 0 与一个相邻的数字（上下左右）进行交换.
       最终当板 board 的结果是 [[1,2,3],[4,5,0]] 谜板被解开。
       给出一个谜板的初始状态 board ，返回最少可以通过多少次移动解开谜板，如果不能解开谜板，则返回 -1 。

      2 4 1
      5 0 3
  答: 转换成一维数组: 2 4 1 5 0 3 -> 其中 neighbor[4] = { 1, 3, 5}, 第4个位置 0 的上索引为 1(数字 4), 左索引为 3(数字 5), 右索引为 5(数字 3), 没有下索引
*/
pub(crate) fn sliding_puzzle(
    nums: Vec<Vec<usize>>,
    target: Vec<Vec<usize>>,
    m: usize,
    n: usize,
) -> i32 {
    if nums.is_empty() || target.is_empty() {
        return -1;
    }

    fn convert_to_string(values: Vec<Vec<usize>>) -> String {
        return values
            .iter()
            .flat_map(|inner| inner.iter())
            .map(|&num| num.to_string())
            .collect::<Vec<String>>()
            .join("");
    }

    // 将 nums 转成一维数组字符串
    let target = convert_to_string(target);
    let start = convert_to_string(nums);

    let mut visited: Vec<String> = Vec::new(); // 记录已经穷举过的密码，防止走回头路
    let mut queue: Vec<String> = Vec::new();
    queue.push(start);

    let mut step = 0;

    while queue.len() > 0 {
        for i in (0..queue.len()).rev() {
            let cur = queue.remove(i); // swap_remove 会改变向量数据

            // 是否到达目标
            if cur == target {
                return step;
            }

            // 找到数字 0 的索引
            let mut index = 0;
            while cur.chars().nth(index).unwrap() != '0' {
                index += 1;
            }

            // 获取其相邻位置
            let neighbor: Vec<Vec<usize>> = generate_neighbor(m as i32, n as i32);

            if index >= neighbor.len() {
                continue;
            }

            // 将数字 0 和相邻的数字交换位置
            let neighbors = neighbor.get(index).unwrap();
            for nei in neighbors.iter() {
                let new_board = swap_neighbor(cur.clone(), nei.clone(), index);
                // 防止走回头路
                if !visited.contains(&new_board) {
                    visited.push(new_board.clone());
                    queue.push(new_board)
                }
            }
        }

        step += 1;
    }

    // 交换
    fn swap_neighbor(cur: String, nei: usize, index: usize) -> String {
        let mut chars: Vec<char> = cur.chars().collect();
        if nei < chars.len() && index < chars.len() {
            chars.swap(index, nei);
        }

        chars.iter().collect()
    }

    // 生成上下左右索引
    fn generate_neighbor(m: i32, n: i32) -> Vec<Vec<usize>> {
        let max = m * n;
        let mut neighbor = Vec::new();

        for i in 0..max {
            let mut neighbors: Vec<usize> = Vec::new();

            // 如果不是第一列，有左侧邻居
            if i % n != 0 {
                neighbors.push((i - 1) as usize)
            }

            // 如果不是最后一列，有右侧邻居
            if i % n != n - 1 {
                neighbors.push((i + 1) as usize)
            }

            // 如果不是第一行，有上方邻居
            if i >= n {
                neighbors.push((i - n) as usize);
            }

            // 如果不是最后一行，有下方邻居
            if i + n < m * n {
                neighbors.push((i + n) as usize);
            }

            neighbor.push(neighbors);
        }

        neighbor
    }

    step
}
