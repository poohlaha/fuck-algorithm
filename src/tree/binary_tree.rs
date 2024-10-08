//! 二叉树

use std::collections::VecDeque;

pub(crate) struct TreeNode<E> {
    pub(crate) element: E,
    pub(crate) left: Option<Box<TreeNode<E>>>,
    pub(crate) right: Option<Box<TreeNode<E>>>,
}

impl<E> TreeNode<E> {
    pub fn new(element: E) -> Self {
        TreeNode {
            element,
            left: None,
            right: None,
        }
    }
}

/// 查找二叉树的最大深度(前序遍历, 快速排序)(时间复杂度为 O(n))
/**
  根节点深度从 1 开始, 先遍历左子节点, 再遍历右子节点,
  1. 前序位置的代码在刚刚进入一个二叉树节点的时候执行；
  2. 后序位置的代码在将要离开一个二叉树节点的时候执行；
  3. 中序位置的代码在一个二叉树节点左子树都遍历完，即将开始遍历右子树的时候执行。
*/

pub(crate) fn get_max_depth(root: TreeNode<u32>) -> u32 {
    let mut result: u32 = 0; // 记录最大深度
    let depth: u32 = 0; //记录遍历到的节点的深度

    fn traverse(node: Option<Box<TreeNode<u32>>>, depth: u32, result: &mut u32) {
        if let Some(node) = node {
            // 前序位置
            let mut depth = depth + 1; // 增加深度

            // 到达叶子节点，更新最大深度
            if node.left.is_none() && node.right.is_none() {
                *result = std::cmp::max(*result, depth);
            }

            traverse(node.left, depth, result);
            traverse(node.right, depth, result);

            // 后序位置
            depth -= 1;
        }
    }

    traverse(Some(Box::new(root)), depth, &mut result);
    result
}

/// 查找二叉树的最大深度2(后序遍历, 归并排序)(时间复杂度为 O(n))
pub(crate) fn get_max_depth2(root: Option<Box<TreeNode<u32>>>) -> u32 {
    if let Some(root) = root {
        let left_depth = get_max_depth2(root.left);
        let right_depth = get_max_depth2(root.right);

        // 返回当前节点的深度（左右子树中深度较大的一个加上当前节点）
        return std::cmp::max(left_depth, right_depth) + 1; // 1 是根节点
    }

    0
}

/**
二叉树的最小深度
力扣: https://leetcode.cn/problems/minimum-depth-of-binary-tree/description/
题目: 给定一个二叉树，找出其最小深度。最小深度是从根节点到最近叶子节点的最短路径上的节点数量。说明：叶子节点是指没有子节点的节点。
答: `起点` 就是 `root` 根节点, `终点` 就是最靠近根节点的那个 `叶子节点`
`while` 循环和 `for` 循环的配合，`while` 循环控制 `一层一层往下走`，`for` 循环利用 `size` 变量控制 `从左到右遍历每一层二叉树节点`
 */
pub(crate) fn get_min_dept(root: TreeNode<u32>) -> u32 {
    let mut queue: Vec<TreeNode<u32>> = Vec::new();
    queue.push(root); // start

    // root 本身就是一层，depth 初始化为 1
    let mut dept: u32 = 1;

    while queue.len() > 0 {
        let size = queue.len();
        // 将当前队列中的所有节点向四周扩散
        for i in 0..size {
            let cur = queue.remove(i);
            // 判断是否到达终点
            if cur.left.is_none() && cur.right.is_none() {
                return dept;
            }

            // 将 cur 的相邻节点加入队列
            if let Some(left) = cur.left {
                queue.push(*left);
            }

            if let Some(right) = cur.right {
                queue.push(*right);
            }
        }

        dept += 1;
    }

    dept
}

/// 前序遍历
/**
  根节点在首位, 然后左子树的前序遍历结果, 最后是右子树的前序遍历结果
  一棵二叉树的前序遍历结果 = 根节点 + 左子树的前序遍历结果 + 右子树的前序遍历结果
**/
pub(crate) fn preorder_traverse(root: Option<Box<TreeNode<u32>>>) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();
    if let Some(root) = root {
        result.push(root.element); // 根节点在首位
        result.extend(preorder_traverse(root.left));
        result.extend(preorder_traverse(root.right));
    }
    result
}

/// 中序遍历
/**
  左子树的前序遍历结果, 然后是根节点在中间, 最后是右子树的前序遍历结果
*/
pub(crate) fn inorder_traverse(root: Option<Box<TreeNode<u32>>>) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();
    if let Some(root) = root {
        result.extend(preorder_traverse(root.left));
        result.push(root.element); // 根节点在中间
        result.extend(preorder_traverse(root.right));
    }
    result
}

/// 后序遍历
/**
左子树的前序遍历结果, 然后是右子树的前序遍历结果, 最后是根节点在末尾
 */
pub(crate) fn postorder_traverse(root: Option<Box<TreeNode<u32>>>) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();
    if let Some(root) = root {
        result.extend(preorder_traverse(root.left));
        result.extend(preorder_traverse(root.right));
        result.push(root.element); // 根节点在末尾
    }
    result
}

/// 计算每个节点所在的层数
/**
 根节点看做第一层
*/
pub(crate) fn get_pre_node_layer(root: Option<Box<TreeNode<u32>>>, level: u32) {
    if let Some(node) = root {
        // 打印当前节点的层数和节点值
        println!("Node {} is at level {}", node.element, level);

        // 递归处理左子树和右子树，层数加 1
        get_pre_node_layer(node.left, level + 1);
        get_pre_node_layer(node.right, level + 1);
    }
}

/// 计算每个节点的左右子树各有多少节点
pub(crate) fn get_node_count(root: Option<Box<TreeNode<u32>>>) -> u32 {
    if let Some(node) = root {
        let left_count = get_node_count(node.left);
        let right_count = get_node_count(node.right);

        // 后序位置
        println!(
            "node `{}` left count: {}, right count:{}",
            node.element, left_count, right_count
        );

        return left_count + right_count + 1;
    }

    0
}

/// 计算二叉树的最长直径长度(后序遍历)
pub(crate) fn diameter_of_binary_tree(root: TreeNode<u32>) -> u32 {
    let mut max_diameter: u32 = 0;

    fn max_depth(root: Option<Box<TreeNode<u32>>>, max_diameter: &mut u32) -> u32 {
        if let Some(node) = root {
            let left_max = max_depth(node.left, max_diameter);
            let right_max = max_depth(node.right, max_diameter);

            // 后序位置，顺便计算最大直径
            *max_diameter = std::cmp::max(*max_diameter, left_max + right_max);

            // 返回当前节点的深度（左右子树中深度较大的一个加上当前节点）
            return 1 + std::cmp::max(left_max, right_max);
        }

        0
    }

    max_depth(Some(Box::new(root)), &mut max_diameter)
}

/// 层序遍历(广度优先搜索（BFS))
/**
    层序遍历的过程中，可以使用队列（FIFO）来保存待访问的节点，保证按照从上到下、从左到右的顺序逐层遍历。这样遍历的结果就是二叉树节点的层级顺序。
  1. 将根节点放入队列中。
  2. 从队列中取出一个节点，访问该节点，并将其所有未访问过的子节点（如果有）依次放入队列中。
  3. 重复步骤 2 直到队列为空，表示所有节点都已访问。
*/
pub(crate) fn level_traverse(root: Option<Box<TreeNode<u32>>>) -> Vec<u32> {
    let mut queue = VecDeque::new(); // 双端队列
    let mut result = Vec::new();

    if let Some(root) = root {
        queue.push_back(root);

        while !queue.is_empty() {
            if let Some(node) = queue.pop_front() {
                result.push(node.element);

                if let Some(left) = node.left {
                    queue.push_back(left)
                }

                if let Some(right) = node.right {
                    queue.push_back(right)
                }
            }
        }
    }

    result
}

/// 全排列问题(交换)
pub fn permute(nums: Vec<u32>) -> Vec<Vec<u32>> {
    let mut res: Vec<Vec<u32>> = Vec::new();
    let start: usize = 0;

    let mut new_nums = nums.clone();

    fn backtrack(start: usize, nums: &mut Vec<u32>, result: &mut Vec<Vec<u32>>) {
        if start == nums.len() {
            result.push(nums.clone());
            return;
        }

        for i in start..nums.len() {
            nums.swap(start, i);
            backtrack(start + 1, nums, result);
            nums.swap(start, i); // 回溯
        }
    }

    backtrack(start, &mut new_nums, &mut res);
    return res;
}

/**
    全排列问题(非交换)
    力扣: https://leetcode.cn/problems/permutations/description/

                         []
              1/            2|             3\
              [1]           [2]             [3]
          2/     3\      1/    3|         1/    2|
         [1,2]  [1,3]   [2,1]   [2,3]    [3,1]   [3,2]
           |      |       |       |       |        |
       [1,2,3]  [1,3,2] [2,1,3] [2,3,1] [3,1,2]  [3,2,1]

    排列问题本身就是让你穷举元素的位置，nums[i] 之后也可以出现 nums[i] 左边的元素，所以之前的那一套玩不转了，需要额外使用 used 数组来标记哪些元素还可以被选择。
*/
pub fn permute_new(nums: Vec<u32>) -> Vec<Vec<u32>> {
    let mut res: Vec<Vec<u32>> = Vec::new();
    let mut track: Vec<u32> = Vec::new();
    let max = nums.len() as usize;
    let mut used = vec![false; max];

    fn backtrack(
        track: &mut Vec<u32>,
        nums: &Vec<u32>,
        res: &mut Vec<Vec<u32>>,
        used: &mut Vec<bool>,
    ) {
        if track.len() == nums.len() {
            res.push(nums.clone());
            return;
        }

        for i in 0..nums.len() {
            if used[i] {
                // 已存在, 跳过
                continue;
            }

            // 做选择
            track.push(nums[i]);
            used[i] = true;
            backtrack(track, nums, res, used);

            // 取消选择
            track.pop();
            used[i] = false;
        }
    }

    backtrack(&mut track, &nums, &mut res, &mut used);
    return res;
}

/// 8皇后问题
pub fn solve_n_queens(n: usize) -> Vec<Vec<String>> {
    // 初始化 n * n 的横盘
    let mut board = vec![vec![".".to_string(); n]; n]; // '.' 表示空，'Q' 表示皇后，初始化空棋盘
    let mut res: Vec<Vec<String>> = Vec::new();

    // 路径：board 中小于 row 的那些行都已经成功放置了皇后
    // 选择列表：第 row 行的所有列都是放置皇后的选择
    // 结束条件：row 超过 board 的最后一行
    fn backtrack(board: &mut Vec<Vec<String>>, row: usize, res: &mut Vec<Vec<String>>) {
        // 触发结束条件
        if row == board.len() {
            let solution: Vec<String> = board.iter().map(|r| r.join("")).collect();
            res.push(solution);
            return;
        }

        let n = board[row].len();
        for col in 0..n {
            // 排除不合法选择
            if row > 0 {
                if !is_valid(board, row, col) {
                    continue;
                }
            }

            // 做选择
            board[row][col] = "Q".to_string();
            // 进入下一行决策
            backtrack(board, row + 1, res);
            // 撤销选择
            board[row][col] = ".".to_string();
        }
    }

    // 是否可以在 board[row][col] 放置皇后？
    fn is_valid(board: &mut Vec<Vec<String>>, row: usize, col: usize) -> bool {
        let n = board.len() as i32;
        // 检查列是否有皇后互相冲突
        for i in 0..=row {
            if board[i][col] == "Q".to_string() {
                return false;
            }
        }

        // 检查左上方是否有皇后互相冲突
        let mut i = row as i32 - 1;
        let mut j = col as i32 - 1;
        while i >= 0 && j >= 0 {
            if board[i as usize][col] == "Q".to_string() {
                return false;
            }

            if i == 0 || j == 0 {
                break;
            }

            i -= 1;
            j -= 1;
        }

        //  检查右上方是否有皇后互相冲突
        let mut i = row as i32 - 1;
        let mut j = col as i32 + 1;
        while i >= 0 && j < n {
            if board[i as usize][col] == "Q".to_string() {
                return false;
            }

            if i == 0 {
                break;
            }

            i -= 1;
            j += 1;
        }

        return true;
    }

    backtrack(&mut board, 0, &mut res);
    return res;
}
