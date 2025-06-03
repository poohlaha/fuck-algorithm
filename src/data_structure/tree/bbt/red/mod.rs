/*!
红黑树, 在多线程中读写
Arc::ptr_eq: 判断两个 Arc<T> 是否指向同一个堆内存地址(即是不是同一个对象)
*/

use std::fmt::{Debug, Formatter};
use std::sync::{Arc, Mutex, Weak};

// 定义红黑树的颜色
#[derive(Clone, PartialEq, Copy, Debug)]
enum Color {
    Red,
    Black,
}

#[derive(Clone)]
struct TreeNode<T> {
    key: T,
    color: Color,
    parent: Option<Weak<Mutex<TreeNode<T>>>>, // 父节点, 使用 Weak 防止循环引用
    left: Option<Arc<Mutex<TreeNode<T>>>>,    // 左子节点
    right: Option<Arc<Mutex<TreeNode<T>>>>,   // 右子节点
}

impl<T: Ord + Debug> Debug for TreeNode<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TreeNode")
            .field("key", &self.key)
            .field("color", &self.color)
            .finish()
    }
}

impl<T: Ord + Debug> TreeNode<T> {
    fn new(key: T, color: Color) -> Self {
        Self {
            key,
            color,
            parent: None,
            left: None,
            right: None,
        }
    }

    fn create(key: T, color: Color) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self::new(key, color)))
    }
}

// 定义红黑树结构体
pub struct RedBlackTree<T> {
    root: Option<Arc<Mutex<TreeNode<T>>>>,
}

impl<T> RedBlackTree<T>
where
    T: Ord + Debug,
{
    pub fn new() -> Self {
        RedBlackTree { root: None }
    }

    // 插入, 创建红色新节点并插入
    pub fn insert(&mut self, key: T) {
        let node = TreeNode::create(key, Color::Red);
        self.bst_insert(node.clone()); // 普通 BST 插入
        self.fix_insert(node); // // 插入后修复红黑树性质
    }

    // 查找
    pub fn search(&self, key: T) -> Option<Arc<Mutex<TreeNode<T>>>> {
        let mut current = self.root.clone();
        while let Some(node_arc) = current {
            let node = node_arc.lock().unwrap();
            if node.key == key {
                return Some(node_arc.clone());
            } else if key < node.key {
                current = node.left.clone();
            } else {
                current = node.right.clone();
            }
        }

        None
    }

    // 删除
    pub fn remove(&mut self, key: T) {
        let node = self.search(key);
        if let Some(node) = node {
            self.remove_node(node);
        }
    }

    /*
          [10B]
          /   \
       [5R]  [15?]
             /   \
          [12R] [20B]
      删除节点 15
    */
    fn remove_node(&mut self, node: Arc<Mutex<TreeNode<T>>>) {
        let mut y = node.clone();
        let mut replace_node: Option<Arc<Mutex<TreeNode<T>>>> = None; // 替代节点

        // 1. 找到要删除的节点
        // 1.1  删除节点 `没有子节点`, `直接删除`
        let node_guard = node.lock().unwrap();
        if node_guard.left.is_none() && node_guard.right.is_none() {
            self.transplant(&node, &replace_node);
            return;
        }

        // 1.2 删除节点 `只有一个子节点`: 将 `子节点`接入 删除节点的 `父节点`
        if node_guard.left.is_none() {
            replace_node = node_guard.right.clone();
            self.transplant(&node, &replace_node);
            return;
        }

        if node_guard.right.is_none() {
            replace_node = node_guard.left.clone();
            self.transplant(&node, &replace_node);
            return;
        }

        // 1.3 删除节点 `有两个子节点`, 需要找到其`中序后继节点`, 并将其`替代删除节点`, 然后 `删除真正删除后继节点`
        // 找到其`中序后继节点`
        let y_min = self.minimum(y.lock().unwrap().right.clone().unwrap());
        y = y_min.clone(); // 把 `中序后继节点` 赋给 y

        // 1.3.2 记录 `后继节点删除前的颜色`, 只有当 `删除` 的是 `黑色节点` 时，才需要 `修复`
        // 记录 `后继节点删除前的颜色`
        let y_color = y.lock().unwrap().color;
        let mut fixed_node: Option<Arc<Mutex<TreeNode<T>>>> = None; // 需要修复的节点

        // 删除真正删除后继节点, 接到原来删除节点的位置
        // 1.3.1 如果 `y` 就是 `node.right`，那说明 `y` 是 `直接的后继`，可以 `直接替换`
        if Arc::ptr_eq(&self.get_parent_node(&y).unwrap(), &node) {
            fixed_node = y.lock().unwrap().right.clone();
        } else {
            // 1.3.2 如果 `y` `不是 node.right`，说明 `y` 是 `node.right` 的 `左边某个节点`，`y 本身也有位置、也有父节点`，这时候就要 `先删除 y 原来的位置`
            // 保存需要修复的节点, 即 y 的右子树
            fixed_node = y.lock().unwrap().right.clone();

            // 把 y 的位置用它的右子节点替代
            self.transplant(&y, &fixed_node);

            // 把 node 的右子树接到 y 的右边
            y.lock().unwrap().right = node.lock().unwrap().right.clone();

            // 更新 y 的 父节点
            if let Some(ref right) = y.lock().unwrap().right {
                right.lock().unwrap().parent = Some(Arc::downgrade(&y));
            }
        }

        // 1.3.3 把 `y 移动到 node 的位置`
        self.transplant(&node, &Some(y.clone()));
        // 把 node 的左子树挂到 y 的左边
        y.lock().unwrap().left = node.lock().unwrap().left.clone();
        // 更新父节点
        if let Some(ref left) = y.lock().unwrap().left {
            left.lock().unwrap().parent = Some(Arc::downgrade(&y));
        }
        // 更新 y 的节点为 node 的节点
        y.lock().unwrap().color = node.lock().unwrap().color;

        // 只有当 `删除` 的是 `黑色节点` 时，才需要 `修复`
        if y_color == Color::Black {
            // 修复节点, 当 fixed_node 为 None, 也是删除的黑色节点
            self.fix_delete(fixed_node.or_else(|| node.lock().unwrap().parent.as_ref()?.upgrade()));
        }
    }

    // 替换子树
    fn transplant(
        &mut self,
        node: &Arc<Mutex<TreeNode<T>>>,
        replace_node: &Option<Arc<Mutex<TreeNode<T>>>>,
    ) {
        let node_parent = self.get_parent_node(node);
        if let Some(parent) = node_parent {
            let mut parent = parent.lock().unwrap();
            if Arc::ptr_eq(node, parent.left.as_ref().unwrap()) {
                parent.left = replace_node.clone();
            } else {
                parent.right = replace_node.clone();
            }
        } else {
            self.root = replace_node.clone();
        }

        // 更新 replace node 的父节点
        if let Some(replace_node) = replace_node {
            replace_node.lock().unwrap().parent = node.lock().unwrap().parent.clone();
        }
    }

    // 修复删除的 5 种情况
    /*
          P                        P
         / \       -->            / \
        x   s        or          s   x
           / \                      / \
    */
    fn fix_delete(&mut self, mut node: Option<Arc<Mutex<TreeNode<T>>>>) {
        while node.is_some() {
            if let (Some(root), Some(n)) = (&self.root, &node) {
                if Arc::ptr_eq(root, n) && self.get_color(&node) == Color::Black {
                    let parent = self.get_parent_node(node.as_ref().unwrap());
                    if let Some(parent) = parent {
                        let mut is_left = if let Some(left) = parent.lock().unwrap().left.clone() {
                            Arc::ptr_eq(&left, &node.as_ref().unwrap())
                        } else {
                            false
                        };

                        // 获取兄弟节点
                        let sibling = if is_left {
                            parent.lock().unwrap().right.clone()
                        } else {
                            parent.lock().unwrap().left.clone()
                        };

                        // 修复的四种情况
                        if let Some(sibling) = sibling {
                            let sibling_lock = sibling.lock().unwrap();
                            let sibling_color = sibling_lock.color;

                            // 9.2.1. 兄弟是红色
                            if sibling_color == Color::Red {
                                // 1) 将 `s` `变黑`，`p` `变红` (S.color = Black, P.color = Red)
                                sibling.lock().unwrap().color = Color::Black;
                                parent.lock().unwrap().color = Color::Red;

                                // 2) 以 `p` 为中心做一次 `旋转`(左旋或右旋，取决于 x 是左孩子还是右孩子)
                                if is_left {
                                    self.left_rotate(&parent.clone());
                                } else {
                                    self.right_rotate(&parent.clone());
                                }

                                drop(sibling_lock);
                                continue;
                            }

                            // 9.2.2 兄弟是黑色
                            if sibling_color == Color::Black {
                                let sibling_left = sibling_lock.left.as_ref();
                                let sibling_right = sibling_lock.right.as_ref();

                                // a. 两个子节点也都是黑色
                                if !self.is_red(sibling_left) && !self.is_red(sibling_right) {
                                    // a.1 将 `s` `变红`，消除它的黑色 (S.color = Red)
                                    sibling.lock().unwrap().color = Color::Red;

                                    // a.2 把 `双重黑往上`(`x` 上移到 `p`) 传给 `p(x = p)`，继续 `循环` 修复
                                    node = Some(parent.clone());
                                } else {
                                    if is_left {
                                        // 节点在父节点左边
                                        // 9.2.3. 远侄子是黑色，近侄子是红色
                                        // 如果在左边, 远侄子是 sibling_right(黑色), 近侄子是 sibling_left(红色), 左红右黑
                                        if !self.is_red(sibling_right) && self.is_red(sibling_left)
                                        {
                                            // 将 `s` `变红`，`近侄子 a` `变黑` (S.color = Red, a.color = Black)
                                            sibling.lock().unwrap().color = Color::Red;
                                            if let Some(sibling_left) = sibling_left {
                                                sibling_left.lock().unwrap().color = Color::Black;
                                            }

                                            // 对 `s` 做一次 `旋转`(右旋或左旋，取决于 x 是左还是右)
                                            // drop(sibling_lock);
                                            self.right_rotate(&sibling.clone());
                                        }

                                        // 9.2.4 兄弟是黑色，远侄子是红色
                                        if self.is_red(sibling_right) {
                                            // 让 `s` 的 `颜色` `变成` `p` 的 `颜色` (S.color = P.color)
                                            sibling.lock().unwrap().color =
                                                parent.lock().unwrap().color;

                                            // 将 `p` 和 `远侄子 c` `变黑` (P.color = Black, c.color = Black)
                                            parent.lock().unwrap().color = Color::Red;
                                            if let Some(sibling_right) = sibling_right {
                                                sibling_right.lock().unwrap().color = Color::Black;
                                            }

                                            // drop(sibling);

                                            // 对 `p` 做一次 `旋转`(右旋或左旋，取决于 x 是左还是右)
                                            self.left_rotate(&parent.clone());

                                            // 双重黑消除，结束修复, 红黑性质已经恢复, 不需要再往上修了, 我们把 x 设为 root，下一轮循环就会退出
                                            node = self.root.clone();
                                        } else {
                                            // 9.2.2：两侄子都是黑色
                                            // a.1 将 `s` `变红`，消除它的黑色 (S.color = Red)
                                            sibling.lock().unwrap().color = Color::Red;

                                            // a.2 把 `双重黑往上`(`x` 上移到 `p`) 传给 `p(x = p)`，继续 `循环` 修复
                                            node = Some(parent.clone());
                                        }
                                    } else {
                                        // 节点在父节点右边
                                        // 9.2.3. 远侄子是黑色，近侄子是红色
                                        // 如果在右边, 远侄子是 sibling_left(黑色), 近侄子是 sibling_right(红色), 左黑右红
                                        if !self.is_red(sibling_left) && self.is_red(sibling_right)
                                        {
                                            // 将 `s` `变红`，`近侄子 a` `变黑` (S.color = Red, a.color = Black)
                                            sibling.lock().unwrap().color = Color::Red;
                                            if let Some(sibling_right) = sibling_right {
                                                sibling_right.lock().unwrap().color = Color::Black;
                                            }

                                            // 对 `s` 做一次 `旋转`(右旋或左旋，取决于 x 是左还是右)
                                            // drop(sibling_lock);
                                            self.left_rotate(&sibling.clone());
                                        }

                                        // 9.2.4 兄弟是黑色，远侄子是红色
                                        if self.is_red(sibling_left) {
                                            // 让 `s` 的 `颜色` `变成` `p` 的 `颜色` (S.color = P.color)
                                            sibling.lock().unwrap().color =
                                                parent.lock().unwrap().color;

                                            // 将 `p` 和 `远侄子 c` `变黑` (P.color = Black, c.color = Black)
                                            parent.lock().unwrap().color = Color::Black;
                                            if let Some(sibling_left) = sibling_left {
                                                sibling_left.lock().unwrap().color = Color::Black;
                                            }

                                            // drop(sibling);

                                            // 对 `p` 做一次 `旋转`(右旋或左旋，取决于 x 是左还是右)
                                            self.right_rotate(&parent.clone());

                                            // 双重黑消除，结束修复, 红黑性质已经恢复, 不需要再往上修了, 我们把 x 设为 root，下一轮循环就会退出
                                            node = self.root.clone();
                                        } else {
                                            // 9.2.2：两侄子都是黑色
                                            // a.1 将 `s` `变红`，消除它的黑色 (S.color = Red)
                                            sibling.lock().unwrap().color = Color::Red;

                                            // a.2 把 `双重黑往上`(`x` 上移到 `p`) 传给 `p(x = p)`，继续 `循环` 修复
                                            node = Some(parent.clone());
                                        }
                                    }
                                }
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        // 9.2.5. x 是根节点(特殊)
        // 如果 `x` 是 `根节点` 且是 `双重黑`，直接将其设为 `黑色` 即可，修复结束
        if let Some(node) = node {
            node.lock().unwrap().color = Color::Red;
        }
    }

    // 最小辅助函数, 用于找后继节点
    fn minimum(&mut self, node: Arc<Mutex<TreeNode<T>>>) -> Arc<Mutex<TreeNode<T>>> {
        match node.lock().unwrap().left.clone() {
            None => node.clone(),
            Some(left) => self.minimum(left.clone()),
        }
    }

    // 二叉搜索树插入
    fn bst_insert(&mut self, node: Arc<Mutex<TreeNode<T>>>) {
        let mut current = self.root.clone(); // 从根节点开始搜索位置
        let mut parent_node = None; // 父节点

        while let Some(cur) = current {
            parent_node = Some(cur.clone()); // 更新父节点
            current = if node.lock().unwrap().key < cur.lock().unwrap().key {
                cur.lock().unwrap().left.clone() // 查找左子树
            } else {
                cur.lock().unwrap().right.clone() // 查找右子树
            }
        }

        // 找到父节点
        if let Some(parent_node) = parent_node {
            // 赋值给当前节点的父节点
            node.lock().unwrap().parent = Some(Arc::downgrade(&parent_node));
            if node.lock().unwrap().key < parent_node.lock().unwrap().key {
                // 在父节点的左子树上
                parent_node.lock().unwrap().left = Some(node)
            } else {
                // 在父节点的右子树上
                parent_node.lock().unwrap().right = Some(node)
            }

            return;
        }

        // 没有父节点, 直接赋值给根节点
        self.root = Some(node);
    }

    // 修复插入造成的红黑树失衡
    fn fix_insert(&mut self, mut node: Arc<Mutex<TreeNode<T>>>) {
        // 获取父节点
        while let Some(parent) = self.get_parent_node(&node) {
            // 1. 父节点是黑色, 则无需修复
            if self.get_node_color(&parent) == Color::Black {
                break;
            }

            // 获取祖父节点
            let grand = self.get_grandparent_node(&node).unwrap();

            // 获取叔叔节点, 需要判断叔叔节点是祖父节点的左子还是右子
            let uncle = if Arc::ptr_eq(&grand, &grand.lock().unwrap().left.as_ref().unwrap()) {
                grand.lock().unwrap().right.clone()
            } else {
                grand.lock().unwrap().left.clone()
            };

            // 2. 父节点是红色 ❌（出现连续红色）
            // 2.1. 叔叔是红色 -> 变色并向上修复
            let mut uncle_color = self.get_color(&uncle);
            if uncle_color == Color::Red {
                // 设置 `父节点` 为 `黑色`
                parent.lock().unwrap().color = Color::Black;

                // 设置 `叔叔节点` 为 `黑色`
                if let Some(uncle) = uncle {
                    uncle.lock().unwrap().color = Color::Black;
                }

                // 设置 `祖父节点` 为 `红色`
                grand.lock().unwrap().color = Color::Red;
                node = grand; // 向上查找
            } else {
                // 叔叔节点是黑色
                // 2.2  叔叔是黑色，且 z 是 `内侧子树`(需要 `旋转`)
                // 把内侧结构转换成外侧结构, 内侧结构需要 `先变外`再进行标准的 `变色 + 单旋转` 操作
                if Arc::ptr_eq(&node, &parent.lock().unwrap().right.as_ref().unwrap())
                    && Arc::ptr_eq(&parent, &grand.lock().unwrap().left.as_ref().unwrap())
                {
                    // 父节点为祖父节点的左子, 当前节点为父节点的右子, 则为 `左-右`
                    // 2.2.1 内侧(左-右) → 先左旋再右旋, 此处只完成一次左旋, 右旋在下面的单旋转中实现
                    self.left_rotate(&parent);
                    node = parent;
                } else if Arc::ptr_eq(&node, &parent.lock().unwrap().left.as_ref().unwrap())
                    && Arc::ptr_eq(&parent, &grand.lock().unwrap().right.as_ref().unwrap())
                {
                    // 父节点为祖父节点的右子, 当前节点为父节点的左子, 则为 `右-左`
                    // 2.2.2 内侧(右-左) →  先右旋再左旋, 此处只完成一次右旋, 左旋在下面的单旋转中实现
                    self.right_rotate(&parent);
                    node = parent;
                }

                // 2.3 叔叔是黑色，z 是 `外侧子树`(旋转 + 变色), 外侧 → 父变黑，祖父变红，再旋转
                let parent = self.get_parent_node(&node).unwrap();
                let grand = self.get_grandparent_node(&node).unwrap();

                // 设置 `父节点` 为 `黑色`
                parent.lock().unwrap().color = Color::Black;

                // 设置 `祖父节点` 为 `红色`
                grand.lock().unwrap().color = Color::Red;

                // 单旋转, 根据父节点位置旋转, 完成上面的 `左-右` 和 `右-左` 的旋转
                if Arc::ptr_eq(&node, &parent.lock().unwrap().left.as_ref().unwrap()) {
                    self.right_rotate(&grand);
                } else {
                    self.left_rotate(&grand);
                }
            }
        }

        // 保证 `根节点` 为 `黑色`
        if let Some(root) = &self.root {
            root.lock().unwrap().color = Color::Black;
        }
    }

    // 左旋
    // 所有节点大小关系是： A < x < B < y < C
    /*
       x
      / \
     A   y         ===(左旋)==>          y
        / \                            / \
       B   C                          x   C
                                     / \
                                    A   B
    */
    fn left_rotate(&mut self, x: &Arc<Mutex<TreeNode<T>>>) {
        let y = x.lock().unwrap().right.take().unwrap(); // 取出右子树 y 及其子节点

        // 1. 设置  y 节点的 左子树 为 x 节点的右子树
        x.lock().unwrap().right = y.lock().unwrap().left.take();

        // 设置 1 中右子树的父节点为 x
        if let Some(right) = &x.lock().unwrap().right {
            right.lock().unwrap().parent = Some(Arc::downgrade(&x));
        }

        // 设置 y 的父节点为 x
        y.lock().unwrap().parent = x.lock().unwrap().parent.clone();

        // 判断 x 节点是原来 x 节点父节点的位置, 并重新设置新节点 y 的位置
        if let Some(parent) = self.get_parent_node(&x) {
            if Arc::ptr_eq(&x, &parent.lock().unwrap().left.as_ref().unwrap()) {
                parent.lock().unwrap().left = Some(y.clone());
            } else {
                parent.lock().unwrap().right = Some(y.clone());
            }
        } else {
            // 如果原来 x 节点已是根节点, 直接设置 y 为根节点
            self.root = Some(y.clone());
        }

        // 设置 y 的左子树为 x
        y.lock().unwrap().left = Some(x.clone());
        // 设置 x 的父节点为 y
        x.lock().unwrap().parent = Some(Arc::downgrade(&y));
    }

    // 右旋
    // 所有节点大小关系是： A < x < B < y < C
    /*
        y
       / \
      x   C         ===(右旋)==>            x
     / \                                  / \
    A   B                                A   y
                                            / \
                                           B   C
     */
    fn right_rotate(&mut self, y: &Arc<Mutex<TreeNode<T>>>) {
        // node 为 y 节点
        let x = y.lock().unwrap().left.take().unwrap(); // 取出 x 及其子节点

        // 1. 设置 y 的左子树为 x 节点的右子树
        y.lock().unwrap().left = x.lock().unwrap().right.clone();

        // 设置 1 中的父节点为 y 节点
        if let Some(y_left) = &y.lock().unwrap().left {
            y_left.lock().unwrap().parent = Some(Arc::downgrade(&y));
        }

        // // 判断 y 节点是原来 y 节点父节点的位置, 并重新设置新节点 x 的位置
        if let Some(parent) = self.get_parent_node(&y) {
            if Arc::ptr_eq(&y, &parent.lock().unwrap().left.as_ref().unwrap()) {
                parent.lock().unwrap().left = Some(x.clone());
            } else {
                parent.lock().unwrap().right = Some(x.clone());
            }
        } else {
            // 如果原来 y 节点已是根节点, 直接设置 x 为根节点
            self.root = Some(x.clone());
        }

        // 设置 x 节点的右子树为 y
        x.lock().unwrap().right = Some(y.clone());

        // 设置 y 节点的父节点为 x
        y.lock().unwrap().parent = Some(Arc::downgrade(&x));
    }

    // 获取节点颜色
    fn get_node_color(&mut self, node: &Arc<Mutex<TreeNode<T>>>) -> Color {
        node.lock().unwrap().color
    }

    // 获取节点颜色, 如果 `节点不存在`, 则默认为 `黑色`
    fn get_color(&mut self, node: &Option<Arc<Mutex<TreeNode<T>>>>) -> Color {
        match node {
            None => Color::Black,
            Some(node) => self.get_node_color(&node),
        }
    }

    // 获取节点的颜色
    fn color_of(&mut self, node: Option<&Arc<Mutex<TreeNode<T>>>>) -> Color {
        node.map_or(Color::Black, |n| n.lock().unwrap().color)
    }

    // 设置颜色
    fn set_color(&mut self, node: Option<&Arc<Mutex<TreeNode<T>>>>, color: Color) {
        if let Some(node) = node {
            node.lock().unwrap().color = color;
        }
    }

    // 判断节点是不是红色
    fn is_red(&mut self, node: Option<&Arc<Mutex<TreeNode<T>>>>) -> bool {
        self.color_of(node) == Color::Red
    }

    // 获取父节点
    fn get_parent_node(
        &mut self,
        node: &Arc<Mutex<TreeNode<T>>>,
    ) -> Option<Arc<Mutex<TreeNode<T>>>> {
        node.lock()
            .unwrap()
            .parent
            .as_ref()
            .and_then(|wp| wp.upgrade())
    }

    // 获取祖父节点(父节点的父节点称为祖父节点)
    fn get_grandparent_node(
        &mut self,
        node: &Arc<Mutex<TreeNode<T>>>,
    ) -> Option<Arc<Mutex<TreeNode<T>>>> {
        self.get_parent_node(&node)
            .and_then(|p| self.get_parent_node(&p))
    }
}
