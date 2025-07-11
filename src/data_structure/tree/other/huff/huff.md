# 哈夫曼树(Huffman Tree)
  哈夫曼树是一种 `带权路径长度(即各叶子节点的权值乘以其到根节点的路径长度的和)最短的二叉树`, 也称为 `最优二叉树`, 它是一种用于为数据压缩的树结构, 主要用于构建霍夫曼编码, 从而实现最优的无损编码
  
1. 特性
   - `带权路径长度最小`: 所有可能的二叉树中，哈夫曼树使带权路径长度（WPL, Weighted Path Length）最小
   - `所有叶子节点代表符号`，其权值为该符号的频率或概率
   - `非叶子节点的权值等于其左右子节点权值之和`
   - `是一棵完全二叉树（或接近完全）`: 结构紧凑，便于编码
   - `编码的前缀性质`: 任何一个编码都不是另一个编码的前缀，保证解码的唯一性
   - `构建过程基于贪心策略`: 每次选取权值最小的两个节点合并
   
2. 时间复杂度
   - 构造哈夫曼树的主要操作是不断从一组节点中选择两个最小权值节点合并，直到合成一棵树
   - 常用实现是利用 `优先队列（小顶堆）`，每次取两个最小元素
   - 时间复杂度: O(n log n)(其中 n 是叶子节点数(符号种类数))
   
3. 应用场景
   - 数据压缩: 比如文本、图像等文件压缩，JPEG和PNG图像格式的无损压缩底层算法就用到了哈夫曼编码
   - 通信系统: 信源编码，减少传输数据量，节省带宽
   - 文件格式: 如 `ZIP`、`GZIP` 等压缩文件格式都用哈夫曼编码
   - 信息论: 实现最优前缀码，是编码理论的基础
   - 机器学习与自然语言处理: 有时用于构建词汇编码等

4. 举例(插入)
   ```text
    假设我们当前已经有了 4 个符号及其频率:
    符号       频率
    A          5
    B          9
    C          12
    D          13
    这些频率(5, 9, 12, 13)是哈夫曼树当前待合并的最小堆(或优先队列)
   
    现在 `插入` 一个新符号
    符号        频率
    E          16
    
    详细步骤
    - 把新符号 E 放入当前节点集合
      集合变成:  
    符号       频率
    A          5
    B          9
    C          12
    D          13
    E          16
   
     - 重新排序（或在最小堆里重新放入）
       目标：找到 `频率最小` 的两个节点
       最小:
       - A: 5
       - B: 9
   
     - 合并最小的两个节点
       - 新节点(父节点)频率: 5 + 9 = 14
       - 左右子节点: A、B
   
     - 更新集合
       新集合:
     符号/子树	   频率
     (A, B)        14
     C             12
     D             13
     E             16
   
    - 继续找最小的两个节点合并
      最小:
      - C: 12
      - D: 13
   
    - 合并最小的两个节点
      - 新节点(父节点)频率: 12 + 13 = 25
      - 左右子节点: C、D
   
    - 更新集合
       新集合:
     符号/子树	   频率
     (A, B)        14
     (C, D)        25
     E             16   
   
    - 继续找最小的两个节点合并
      最小:
      - (A, B): 14
      - E: 16
   
    - 合并最小的两个节点
      - 新节点(父节点)频率: 14 + 16 = 30
      - 左右子节点: (A, B)、E
   
    - 更新集合
       新集合:
     符号/子树	   频率
     (C, D)        25
     (A, B, E)     30
   
    - 继续找最小的两个节点合并
      最小:
      - (C, D): 25
      - (A, B, E): 30
   
    - 合并最小的两个节点
      - 新节点(父节点)频率: 25 + 30 = 55
      - 左右子节点: (C, D)、(A, B, E)    
   
    - 最终
            (55)
           /    \
        (25)    (30)
        /  \     /  \
      (C)  (D) (14)  (E)
                    /  \
                  (A)  (B)
   ```
   
5. 举例(修改)
   - 哈夫曼树本身 `不支持局部修改`，只能 `重新建树`
   
6. 举例(删除)
   - 哈夫曼树本身 `不支持局部删除`，只能 `重新建树`

7. 举例(查找)
  - 哈夫曼树 `没有二叉搜索性质`(左右子树没有有序性), 但它依然是二叉树，查找过程就是 `常规二叉树遍历`

8. 构建过程
  - 通过频率表构造哈夫曼树
  - 递归生成编码表
  - 解码二进制串 → 符号串

  - 通过频率表构造哈夫曼树
    哈夫曼树是 `最优前缀编码树`: `频率高` 的 `字符路径短`，`频率低` 的 `字符路径长`, 其本质是贪心合并: 每次合并 `当前频率最小` 的两棵子树，直到 `只剩一棵树`
    - 创建叶子节点
    ```text
    'A' 频率 5 → 叶子节点 (5, A)  
    'B' 频率 9 → 叶子节点 (9, B)  
    ...
    ```
    - 把所有叶子节点放进最小堆, 最小频率优先
    - 重复操作，直到堆里只剩 `1` 个根节点
      - 每次从堆里弹出最小的两个节点 `left`、`right`
      - 创建新父节点: `parent.frequency = left.frequency + right.frequency`, left、right 分别是子树
      - 把 `parent` 再放入堆中
      - 这样频率小的合并在更底层，频率大的路径更短
      - 最终，堆里只剩下 `一个根节点` ，就是 `完整的哈夫曼树`
    
  - 递归生成编码表
    用于快速查找每个字符对应的编码(01串)
    - 从 `根节点` 开始，路径用字符串 `path` 记录
      - 左走加 `0`，右走加 `1`
    - 遇到 `叶子节点` (`symbol != None`)
      - 说明找到一个字符的完整编码路径
      - 把 `path` 放入 `code_map` 中: `code_map.insert(symbol, path)`
    - 遇到 `非叶子节点` 继续 `递归`
    - 得到 `所有字符 → 编码的映射表`(哈希表，快速定位)

  - 解码二进制串 ➜ 符号串
    - 遍历二进制串(如 `01010101...`)
    - 从 `根节点` 出发：遇到 `0 走左` ，遇到 `1 走右`
    - 走到 `叶子节点`(symbol 有值) → `输出一个符号` → `重置到根节点继续下一段`
    - 对于每个比特位，动态移动到左/右子树
    - 一旦到达 `叶子(即找到一个完整的编码路径)` → `拼接符号` → `重新从根出发`

9. 什么是带权路径长度？
   带权路径长度(Weighted Path Length，简称 WPL), 所有 `叶子节点` 的 (`权值` × `从根到该节点的路径长度`) 之 `和`
   - 为啥是`带权`？
     - `普通` 路径长度：只看 `从根到节点走了几步`
     - `带权` 路径长度：在 `普通路径长度基础` 上，`乘以` 节点的 `权值`(比如 `频率`、`出现次数`)
     - `权值` 意味着这个节点的 `重要性`
   ```text
    假设有一棵树:
           root
         /    \
        A      B
              / \
             C   D
   
    叶子节点: A、C、D
    - A 权值 5, 深度 1
    - C 权值 3, 深度 2  
    - D 权值 2, 深度 2  
   
    计算:
       WPL = ( 5 * 1) + (3 * 2) + (2 * 2) = 5 + 6 + 4 = 15
    ```
   - 为啥 `WPL` 重要?
     哈夫曼树 的目标就是: `让 WPL 尽可能小！`, 因为
     - `权值大` 的节点(`高频`) `放浅` (`路径短`)
     - `权值小` 的节点(`低频`) `放深` (`路径长`)
   这就是哈夫曼树 `最优编码` 的核心意义
   
   - 总结:
     出现多的，压缩得短; 出现少的，放到更深, 整体二进制总长度最小
     - 权值 = 这个节点出现的次数(高频字母出现多，低频字母出现少) 
     - 路径长度 = 这个字母编码时占的 `位数`
     - `权值大` 的 `重要` 节点，`路径长度小`(`节省位数`)
     - `权值小` 的节点，`路径长`(无所谓，`出现少`)
    