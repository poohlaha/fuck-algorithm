//! 二叉树

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
    let mut depth: u32 = 0; //记录遍历到的节点的深度

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

        return std::cmp::max(left_depth, right_depth) + 1; // 1 是根节点
    }

    0
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
