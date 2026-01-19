/*!
  [kahn 算法](src/graph/base/two.md)

  **算法步骤**
  - 统计每个点的入度
  - 把**入度为0的点**放进队列
  - 不断 BFS 处理:
    - 出列一个点 u
    - 把 u 加入结果序列
    - 删除 u 的所有出边
      - 相邻点入度 -1
      - 如果入度变成 0 → 入队
  - 最终:
    - 结果长度 = 点数 → 成功
    - 否则 → 图中有环
*/

use std::collections::VecDeque;

/**
   ```
   0 → 1 → 3
    \→ 2 →/

   用邻接表:
   0: [1, 2]
   1: [3]
   2: [3]
   3: []

   let graph: Vec<Vec<usize>> = vec![
        vec![1, 2], // 0 -> 1, 2
        vec![3],    // 1 -> 3
        vec![3],    // 2 -> 3
        vec![],     // 3
   ];

   - graph[u]：从 `u` 出发，能到达的所有点
   - 点编号是 `0..n-1`
   ```
*/
fn kahn(graph: Vec<Vec<usize>>) -> Vec<usize> {
    let n = graph.len();
    let mut indegree = vec![0; n];

    // 1. 统计每个点的入度
    // 遍历所有边, 只关心「这个点被指了几次」
    // 此时 indegree = [0, 1, 1, 2]
    for u in 0..n {
        for &v in &graph[u] {
            indegree[v] += 1;
        }
    }

    // 2. 把入度为 0 的点放进队列
    // 入度为 0 = 没有依赖 = 可以最先执行
    // 此时 queue = [0]
    let mut queue = VecDeque::new();
    for i in 0..n {
        if indegree[i] == 0 {
            queue.push_back(i);
        }
    }

    // 3. 不断 BFS 处理
    let mut result = Vec::new();
    while let Some(u) = queue.pop_front() {
        // 3.1 出列一个点 u, 把 u 加入结果序列
        result.push(u);

        // 3.2 删除 u 的所有出边
        for &v in &graph[u] {
            // 3.3 相邻点入度 -1
            indegree[v] -= 1;

            // 3.4 如果入度变成 0 → 入队
            if indegree[v] == 0 {
                queue.push_back(v);
            }
        }
    }

    // 4. 判断是否有环: 结果长度 = 点数 → 成功, 否则 → 图中有环
    if result.len() == n {
        println!("拓扑排序成功: {:?}", result);
        return result;
    } else {
        println!("图中存在环，无法拓扑排序");
        return Vec::new();
    }
}
