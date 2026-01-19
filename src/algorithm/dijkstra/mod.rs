/*!
  [Dijkstra 算法](src/graph/base/two.md)

  **算法步骤**
  1. 初始化 dist, dist[i] = 从起点到 i 的当前最短距离（未知时是 ∞）
  2. 起点入队(优先队列, 最小堆, Dijkstra 的“队列”必须是：按 dist 最小优先)
  3. 重复
     - 取出 dist 最小的点 u
     - 如果 u 已确定 → 跳过
     - 标记 u 已确定
     - 遍历 u 的所有边
       - 尝试更新邻居的 dist
       - 如果变小 → 入队
  ```
        (1)          (2)
  0 ----------> 1 --------> 3
   \            |
    \           |(1)
     \(4)       |
      \         v
        ------> 2 -----> 3
                   (1)

     0 → 1(1)
     0 → 2(4)
     1 → 2(1)
     1 → 3(2)
     2 → 3(1)

     邻接表:
     0: (1, 1), (2, 4)
     1: (2, 1), (3, 2)
     2: (3, 1)
     3: ()
   ```

   定义邻接表:
   ```
    let graph: Vec<Vec<(usize, i32)>> = vec![
        vec![(1, 1), (2, 4)], // 0
        vec![(2, 1), (3, 2)], // 1
        vec![(3, 1)],         // 2
        vec![],               // 3
    ];
   ```

   含义:
   ```
   graph[u] = [(v, weight), ...]
   ```
*/

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn dijkstra(graph: Vec<Vec<(usize, i32)>>) {
    let n = graph.len();
    let inf = i32::MAX / 2; // 防止相加时 i32 溢出

    // 1. 初始化 dist, dist[i] = 从起点到 i 的当前最短距离（未知时是 ∞）
    let mut dist = vec![inf; n];
    let mut visited = vec![false; n];

    // 2， 指定起点
    // 此时: dist = [0, ∞, ∞, ∞], visited = [false, false, false, false]
    let start = 0;
    dist[start] = 0;

    // 3. 起点入队(优先队列, 最小堆)
    // 存: (dist, node)
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, start)));

    // 4. 重复处理
    // 4.1 取出 dist 最小的点 u
    while let Some(Reverse((_, u))) = heap.pop() {
        // u: 当前候选点, d: 它当时入队的距离
        // 4.2 如果 u 已确定 → 跳过
        if visited[u] {
            continue;
        }

        // 4.3 标记 u 已确定
        visited[u] = true;

        // 4.4 遍历 u 的所有边
        for &(v, w) in &graph[u] {
            // 4.5 尝试更新邻居的 dist
            let k = dist[u] + w;
            if k < dist[v] {
                dist[v] = k;
                // 4.6 如果变小 → 入队
                heap.push(Reverse((dist[v], v)));
            }
        }
    }
}
