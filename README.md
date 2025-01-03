# Fuck Algorithm
  通过学习 `labuladong的算法笔记(https://labuladong.online/algo/home/)` 后整理的。


### 动态归纳法
动态规划算法属于分解问题（分治）的思路，它的关注点在整棵「子树」。
明确 `base case` -> 明确 `状态` -> 明确 `选择`  -> 定义 `dp 数组/函数` 的含义。
- 确定 base case
- 确定 `状态`，也就是原问题和子问题中会变化的变量
- 确定 `选择`，也就是导致 `状态` 产生变化的行为
- 明确 `dp 函数/数组` 的定义

### 回溯算法(类似深度优先遍历算法(DFS))
回溯算法属于遍历的思路，它的关注点在节点间的「树枝」, 回溯算法把「做选择」「撤销选择」的逻辑放在 `for` 循环 `里面`。
`DFS` 算法属于遍历的思路，它的关注点在单个「节点」, `DFS` 算法把「做选择」「撤销选择」的逻辑放在 `for` 循环 `外面`。
就是遍历一棵决策树的过程，树的每个叶子节点存放着一个合法答案。你把整棵树遍历一遍，把叶子节点上的答案都收集起来，就能得到所有的合法答案。复杂度都不可能低于 O(N!)。
- 路径: 已经做出的选择
- 选择列表: 当前可以做的选择
- 结束条件: 到达决策树底层，无法再做选择的条件
```text
for 选择 in 选择列表:
    # 做选择
    将该选择从选择列表移除
    路径.add(选择)
    backtrack(路径, 选择列表)
    # 撤销选择
    路径.remove(选择)
    将该选择再加入选择列表
```

### 广度优先遍历算法(BFS)
把一些问题抽象成图，从一个点开始，向四周开始扩散。一般来说，我们写 `BFS` 算法都是用 `队列` 这种数据结构，每次将一个节点周围的所有节点加入队列。
ps: `BFS` 找到的路径一定是最短的，但代价就是空间复杂度可能比 `DFS` 大很多。
- 明确 `start` 和 `target`
- 通过 `队列` 实现
- 多维数组可以转成一维数组字符串, 以些作为 `target`