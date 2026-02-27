/*!
  A* 算法
*/

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

/// 网格中的一个点
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

/// A* 中存入优先队列的节点
#[derive(Debug)]
struct Node {
    point: Point,
    g: f64, // 从起点到当前点的真实代价
    h: f64, // 启发式估计代价
    f: f64, // 总代价 f = g + h
}

/// 让 BinaryHeap 变成“最小堆”
/// 默认是最大堆
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f.partial_cmp(&self.f).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.f == other.f
    }
}

impl Eq for Node {}

/// 欧几里得距离启发函数
fn heuristic(a: Point, b: Point) -> f64 {
    let dx = (a.x - b.x) as f64;
    let dy = (a.y - b.y) as f64;
    (dx * dx + dy * dy).sqrt()
}

/// 获取 8 方向邻居
fn get_neighbors(p: Point, width: i32, height: i32) -> Vec<(Point, f64)> {
    // (dx, dy, move_cost), 上下左右为 1 格, 对角线为 2 格
    // 对角线: √(1² + 1²) = √2
    let directions = vec![
        (1, 0, 1.0),           // 右
        (-1, 0, 1.0),          // 左
        (0, 1, 1.0),           // 下
        (0, -1, 1.0),          // 上
        (1, 1, 2f64.sqrt()),   // 右下
        (-1, -1, 2f64.sqrt()), // 左上
        (1, -1, 2f64.sqrt()),  // 右上
        (-1, 1, 2f64.sqrt()),  // 左下
    ];

    let mut neighbors = Vec::new();
    for (dx, dy, cost) in directions {
        let nx = p.x + dx;
        let ny = p.y + dy;

        // 边界检查
        if nx >= 0 && nx < width && ny >= 0 && ny < height {
            neighbors.push((Point { x: nx, y: ny }, cost));
        }
    }

    neighbors
}

/// A* 主函数
fn astar(start: Point, goal: Point, width: i32, height: i32) -> Option<Vec<Point>> {
    let mut open = BinaryHeap::new(); // open
    let mut came_from: HashMap<Point, Point> = HashMap::new(); // 记录路径
    let mut g_score: HashMap<Point, f64> = HashMap::new(); // 每个点当前最优 g
    let mut closed: HashSet<Point> = HashSet::new(); // closed

    g_score.insert(start, 0.0f64);

    let h = heuristic(start, goal);
    open.push(Node {
        point: start,
        g: 0.0,
        h,
        f: h,
    });

    while let Some(current) = open.pop() {
        // 过滤已经处理过的节点
        if closed.contains(&current.point) {
            continue;
        }

        // 如果到达终点
        if current.point == goal {
            // 回溯路径
            let mut path = vec![goal];
            let mut p = goal;

            while let Some(prev) = came_from.get(&p) {
                path.push(*prev);
                p = *prev;
            }

            path.reverse();
            return Some(path);
        }

        // 加入到 closed
        closed.insert(current.point);

        // 当前最优 g
        let current_g = *g_score.get(&current.point).unwrap();

        // 扩展邻居
        for (neighbor, move_cost) in get_neighbors(current.point, width, height) {
            // 判断是否在 closed 中
            if closed.contains(&neighbor) {
                continue;
            }

            let tentative_g = current_g + move_cost;

            // 查找 g_score 中是否存在节点
            let best_g = g_score.get(&neighbor).cloned().unwrap_or(f64::INFINITY);

            // 小于则更新
            if tentative_g < best_g {
                // 找到更优路径
                came_from.insert(neighbor, current.point);
                g_score.insert(neighbor, tentative_g);

                let h = heuristic(neighbor, goal);
                let f = tentative_g + h;

                // 放入 open 中
                open.push(Node {
                    point: neighbor,
                    g: tentative_g,
                    h,
                    f,
                });
            }
        }
    }

    None // 无路径
}
