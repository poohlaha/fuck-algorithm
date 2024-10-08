//! BFS, 广度优先遍历

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

            // 将节点的相邻节点加入队列
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
        q1 = q2;
        q2 = temp;
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
