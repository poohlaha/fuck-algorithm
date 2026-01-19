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

    // union 按 rank 合并
    // 并查集维护的是每个节点的“根”, 如果两个节点根相同, 说明它们已经连通(同一集合)，不管集合里还有多少其他节点
    // 每个集合都有一个“根节点”, 只要两个节点 find(x) == find(y)，说明它们的根相同，也就说明它们属于同一个集合
    // 高度小的树挂到高度高的树下面 → 保持树尽量平坦
    // 高度相等 → 挂到其中一个，并把其 rank + 1
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
