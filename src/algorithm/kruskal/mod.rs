/*!
  最小生成树(MST)
   **核心**:
   > **从最小的边开始选, 只要不成环就要**

   **算法步骤**:
   - 把所有边接权重排序
   - 从小到大选边
     - 如果加入后不成环 → 要
     - 否则 → 跳过
   - 直到选够 n - 1 条边
*/

/**
   let edges = vec![
        Edge { u: 0, v: 1, weight: 1 },
        Edge { u: 2, v: 3, weight: 1 },
        Edge { u: 1, v: 2, weight: 2 },
        Edge { u: 1, v: 3, weight: 3 },
        Edge { u: 0, v: 2, weight: 4 },
    ];
*/

// 边
#[derive(Debug)]
struct Edge {
    u: usize,
    v: usize,
    weight: i32,
}

// 并查集(每个节点的“根”)
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    // find 带路径压缩
    // 根节点是“自己指向自己”的节点(parent[x] == x)
    // 集合 {0, 1}, 如果 parent[0] = 0, parent[1] = 0，那么 0 是根，1 的根是 0
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }

        self.parent[x]
    }

    /**
     1️⃣ 判断两个点是否已经在同一集合
     2️⃣ 如果是 → 不能合并(会成环)
     3️⃣ 如果不是 → 用「按秩合并」把两个集合合成一个

     在并查集中:
     - 一个 集合 = 一棵树
     - 树的 根节点 = 集合代表
     - parent[x] 指向:
       - 自己(说明是根)
       - 或者父节点

     如:
        ```
         parent = [0, 0, 2, 3]
        ```
        表示:
         - 集合: {0, 1}
         - 集合: {2}
         - 集合: {3}

      root_x == root_y:
      - x 和 y 在同一棵树里
      - 图里已经有路径连通
      - 再连一条边就会 成环

      rank: 这棵树的高度上界(近似高度)

      > 并查集维护的是每个节点的“根”, 如果两个节点根相同, 说明它们已经连通(同一集合)，不管集合里还有多少其他节点
      >
      > 每个集合都有一个“根节点”, 只要两个节点 find(x) == find(y)，说明它们的根相同，也就说明它们属于同一个集合
      >
      > 高度小的树挂到高度高的树下面 → 保持树尽量平坦
      >
      > 高度相等 → 挂到其中一个，并把其 rank + 1

      **为什么不能随便挂？**

        如果你总是这样合并:
        ```
        root_x → root_y
        ```

        很容易出现:

        ```
        1 → 2 → 3 → 4 → 5 → 6 → ...
        ```

        形成 **链表结构**:
        - find 会退化成 O(n)
        - Kruskal 整体会变慢

     **按秩合并的核心原则**:
     > 矮树挂到高树下面

     这样:
     - 树高度不会暴涨
     - 查找路径更短
     - 并查集几乎是 O(1)

     **rank 只在“等高合并”时增长**

      这能保证:
      - rank 最大值 ≤ log₂(n)
      - 并查集操作复杂度 ≈ O(α(n))（反 Ackermann）
    */
    // union 按 rank 合并
    fn union(&mut self, x: usize, y: usize) -> bool {
        // 找根
        let root_x = self.find(x);
        let root_y = self.find(y);

        // 已经在同一集合, 会成环
        if root_x == root_y {
            return false;
        }

        // 不同集合 → 可以合并
        if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y; // x 的根挂到 y 的根下面
        } else if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x; // y 的根挂到 x 的根下面
        } else {
            self.parent[root_y] = root_x; // 任意挂，通常挂 y 到 x
            self.rank[root_x] += 1; // 树高度增加
        }

        true
    }
}

/*
  当前这条边的两个端点，会不会成环？ <= 等价 => (find(u) == find(v) ?)
  - 是 → 不能选(会成环)
  - 否 → 选这条边 + union(u, v)

  Kruskal = 排序后的“逐边试探”算法
  并查集 = 一个“只回答成不成环”的黑盒
*/
fn kruskal(n: usize, mut edges: Vec<Edge>) -> Vec<Edge> {
    // 1. 按权重升序排序
    edges.sort_by(|a, b| a.weight.cmp(&b.weight));

    let mut uf = UnionFind::new(n);
    let mut mst = Vec::new();

    // 2. 从小到大选边
    for edge in edges {
        // 2.1 如果加入后不成环 → 要
        if uf.union(edge.u, edge.v) {
            mst.push(edge);

            // 2.2 直到选够 n - 1 条边
            if mst.len() == n - 1 {
                break;
            }
        }
    }

    mst
}
