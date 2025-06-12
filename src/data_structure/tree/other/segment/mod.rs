/*!
线段数(Segment Tree)
- 用于处理区间查询和区间更新问题。它能够在 `O(log n)` 时间内回答有关数组区间的各种查询
1. 基本原理
   - 构建线段树：是一棵 `完全二叉树`，每个节点代表一个区间的结果。每个叶子节点对应数组中的一个元素，而每个非叶子节点则表示其左右子区间的合并结果
   - 区间查询：能够在 `O(log n)` 时间内查询某个区间的合并结果(如区间和、区间最小值等)
   - 区间更新：当数组的某个元素发生变化时，线段树能够在 `O(log n)` 时间内更新与该元素相关的所有区间
2. 时间复杂度
   - 区间查询: O(log n)
   - 区间更新: O(log n)
   - 构建线段树: O(n)
2. 应用场景
  - 区间和查询: 查询数组某一段区间的和
  - 区间最小值/最大值查询: 查询某一段区间的最小值或最大值
  - 区间更新：更新数组某一段区间的值
*/

use std::sync::{Arc, Mutex};

pub struct TreeNode {
    start: usize, // 区间起点
    end: usize,   // 区间终点
    sum: i64,     // 区间和
    lazy: i64,    // 懒惰标记：该节点子区间需要延迟更新的值
    left: Option<Arc<Mutex<TreeNode>>>,
    right: Option<Arc<Mutex<TreeNode>>>,
}

impl TreeNode {
    pub fn new(start: usize, end: usize) -> Arc<Mutex<TreeNode>> {
        Arc::new(Mutex::new(TreeNode {
            start,
            end,
            sum: 0,
            lazy: 0,
            left: None,
            right: None,
        }))
    }
}

pub struct SegmentTree;

impl SegmentTree {
    // 构建线段树
    pub fn build(nums: &[i64], start: usize, end: usize) -> Arc<Mutex<TreeNode>> {
        let node = TreeNode::new(start, end);
        let mut node_lock = node.lock().unwrap();

        // 叶子节点直接赋值
        if start == end {
            node_lock.sum = nums[start];
        } else {
            let mid = (start + end) / 2;

            // 分治构建左右子树
            node_lock.left = Some(Self::build(nums, start, mid));
            node_lock.right = Some(Self::build(nums, mid + 1, end));

            // 回溯: 计算父节点区间和 = 左子 + 右子
            let mut sum = 0;
            if let Some(left) = node_lock.left.as_ref() {
                let left = left.lock().unwrap();
                sum += left.sum;
            }

            if let Some(right) = node_lock.right.as_ref() {
                let right = right.lock().unwrap();
                sum += right.sum;
            }

            node_lock.sum = sum;
        }

        drop(node_lock);
        node
    }

    // 推送懒惰标记（将当前节点的 lazy 下发给左右子树）
    fn push(node: &Arc<Mutex<TreeNode>>) {
        let mut n = node.lock().unwrap();
        if n.lazy != 0 {
            let lazy = n.lazy;

            // 将 lazy 向左右子树传递
            if let Some(left) = n.left.as_ref() {
                let mut l = left.lock().unwrap();
                l.lazy += lazy;
                l.sum += (l.end - l.start + 1) as i64 * lazy;
            }

            if let Some(right) = n.right.as_ref() {
                let mut r = right.lock().unwrap();
                r.lazy += lazy;
                r.sum += (r.end - r.start + 1) as i64 * lazy;
            }

            // 清除当前节点 lazy
            n.lazy = 0
        }
    }

    // 区间更新: 将 val 加到 [l, r] 区间上
    pub fn update_range(node: &Arc<Mutex<TreeNode>>, l: usize, r: usize, val: i64) {
        let mut n = node.lock().unwrap();
        // 判断三种情况
        // 1. 完全不重叠 -> 跳过
        if n.start > r || n.end < l {
            return;
        }

        // 2. 完全包含 -> 直接 `打懒惰标记` 并 `更新当前 sum`
        if n.start >= l && n.end <= r {
            // `sum += 区间长度 * val` 是 `快速更新节点和`
            n.sum += (n.end - n.start + 1) as i64 * val;
            n.lazy += val;
            return;
        }

        drop(n);

        // 3. 部分重叠 -> `递归下传懒惰标记` 后 `递归更新` `左孩子` 和 `右孩子`
        Self::push(&node);

        if let Some(left) = node.lock().unwrap().left.as_ref() {
            Self::update_range(left, l, r, val);
        }

        if let Some(right) = node.lock().unwrap().right.as_ref() {
            Self::update_range(right, l, r, val);
        }

        // 4. 回溯更新当前节点的 sum
        let mut n = node.lock().unwrap();

        let mut sum = 0;
        if let Some(left) = n.left.as_ref() {
            sum += left.lock().unwrap().sum;
        }

        if let Some(right) = n.right.as_ref() {
            sum += right.lock().unwrap().sum;
        }

        n.sum = sum;
    }

    // 区间查询：求 [l, r] 区间的和
    pub fn query_range(node: &Arc<Mutex<TreeNode>>, l: usize, r: usize) -> i64 {
        let n = node.lock().unwrap();

        // 判断三种情况
        // 1. 完全不重叠 -> 返回 `0`
        if n.start > r || n.end < l {
            return 0;
        }

        // 2. 完全包含 -> 返回 `当前节点值(不用递归)`
        if n.start >= l && n.end <= r {
            return n.sum;
        }

        drop(n);

        // 3. 部分重叠 -> 递归 `左孩子` 和 `左孩子`
        // 先 push 再递归查询
        Self::push(node);

        // 递归左孩子
        let left_sum = if let Some(left) = node.lock().unwrap().left.as_ref() {
            Self::query_range(left, l, r)
        } else {
            0
        };

        let right_sum = if let Some(right) = node.lock().unwrap().right.as_ref() {
            Self::query_range(right, l, r)
        } else {
            0
        };

        // 4. 回溯更新当前节点的 sum
        left_sum + right_sum
    }
}
