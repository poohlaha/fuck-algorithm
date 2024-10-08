use crate::bfs::{bidirectional_open_lock, open_lock};

/// 测试 `解开密码锁的最少次数(打开转盘锁)`
pub(crate) fn test_open_lock() {
    let deadends = vec!["0201", "0101", "0102", "1212", "2002"];
    let target = "0202";
    let dept = open_lock(deadends.clone(), target);
    let dept2 = bidirectional_open_lock(deadends.clone(), target);
    println!("open lock: {}\n", dept);
    println!("bidirectional open lock: {}\n", dept2);
}

pub(crate) fn test() {
    println!("----- BFS start ------");
    test_open_lock();
    println!("----- BFS end ------");
    println!();
}
