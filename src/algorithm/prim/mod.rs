/*!
    Prim 算法(像 Dijkstra)

    适用于 **无向连通图**

    **核心**:
    > **从一个点开始, 不断把“最便宜的边”接进来**

    **算法思路**:
    - 随便选一个起点
    - 维护:
      - 已在树中的点
      - 连接树和外部的最小边
    - 每次:
      - 选一条最小权重边
      - 把新点加入树
    - 直到所有点都进来

    **状态**
    - **✅ 已经在树里的点(Tree)**
    - **🔵 还没进树的点(Outside)**
    - **🌉 连接 Tree 和 Outside 的边**

    只看 **跨界边**, 不看内部边
*/

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

/**
  ```
    // graph[u] = Vec<(v, weight)>
    let graph = vec![
        vec![(1, 1), (2, 4)],         // 0
        vec![(0, 1), (2, 2), (3, 3)], // 1
        vec![(0, 4), (1, 2), (3, 1)], // 2
        vec![(1, 3), (2, 1)],         // 3
    ];
  ```
*/
pub fn prim(graph: Vec<Vec<(usize, i32)>>) {
    let n = graph.len();
    let inf = i32::MAX / 2;

    let mut tree: HashSet<usize> = HashSet::new(); // 已经在树里的点
    let mut outside: HashSet<usize> = (0..n).collect(); // 还没进树的点

    let mut min_edge = vec![inf; n]; // Outside 点到 Tree 的最小边权
    let mut heap = BinaryHeap::new(); // 最小堆：Reverse(weight), v

    let mut total_weight = 0;
    let mut edges = Vec::new(); // 所有的最小边

    // 1. 随便选一个起点
    let start = 0;

    // 1.1 加入 tree, 并从 outside 中移除
    tree.insert(start);
    outside.remove(&start);
    min_edge[start] = 0;
    heap.push(Reverse((0, start)));

    // 2. 循环(n 次)
    while !outside.is_empty() {
        // 弹出最小边对应的点
        let Reverse((w, u)) = heap.pop().unwrap();

        if tree.contains(&u) {
            continue; // 已经在树里，跳过
        }

        // 把新点加入树
        tree.insert(u);
        outside.remove(&u);
        total_weight += w;

        // 找是哪条边让 u 入树
        // 连接 u 和 Tree 的点, 把 u 加入 Tree 的最小边(weight = w)
        for &(v, weight) in &graph[u] {
            if tree.contains(&v) && weight == w {
                edges.push((v, u, w));
                break;
            }
        }

        // 更新邻居的 min_edge 并入堆
        for &(v, weight) in &graph[u] {
            if outside.contains(&v) && weight < min_edge[v] {
                min_edge[v] = weight;
                heap.push(Reverse((weight, v)));
            }
        }
    }

    println!("MST total weight = {}", total_weight);
    println!("MST edges = {:?}", edges);
}
