/*!
  自平衡二叉搜索树(Self-balancing BST)
  这是 `二叉搜索树` 的一个子类，目的是防止二叉搜索树退化成链表（导致搜索效率变低）
    - AVL 树（严格平衡）
    - 红黑树（较宽松的平衡要求）
    - Treap（树堆）
    - Splay 树（伸展树）
  - 搜索时间复杂度：O(log n)
  - 在插入/删除时通过旋转保持平衡
*/

pub mod bst;
mod splay;
mod treap;
