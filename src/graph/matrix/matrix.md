# 矩阵(Matrix)
  矩阵(Matrix) 是一个 `按照长方形排列的`、由`数字(或符号)`组成的 `二维数组`

1. 特性
  - 矩阵的大小用 `行 × 列` 表示, 称为矩阵的 `维度`
  ```text
   A =  | 1  2  3 |
        | 4  5  6 |
   
   表示 `A` 是一个 `2 × 3` 的矩阵
   用 A[i][j] 表示矩阵中第 i 行第 j 列的元素(下标从 1 或 0 开始, 看语言环境)
   ```

  - 概念
    - 行(Row): 横向的一排元素
    - 列(Column): 纵向的一列元素
    - 维度: 行 * 列
    - 方阵: 行数 = 列数的矩阵(如 3 × 3、4 × 4)
    - 零矩阵: 所有元素都 `0` 的矩阵
    - 单位矩阵: 对角线是 `1`, 其余是 `0` 的方阵
    - 转置矩阵: 把矩阵的行变成列(即 `A[i][j]` -> `A[j][i]`)

2. 时间复杂度
   - 加减法
     - O(m×n)
     - 逐元素操作，每个元素操作一次
   - 乘法
     - O(m×n×p)（常见情况）
     - A(m×n) × B(n×p)，三层循环
   - 特别算法
     - O(n^2.81) ~ O(n^2)
     - 使用 Strassen 等高级算法能加快乘法
   - 逆矩阵
     - O(n³)
     - 需要高斯消元等操作

3. 使用场景
   - 图论
     - 邻接矩阵表示图结构(判断是否有边)
   - 线性代数
     - 解线性方程组，计算特征值，向量空间等
   - 计算机图形学
     - 用矩阵表示图形变换，如旋转、缩放、平移
   - 机器学习/AI
     - 神经网络中的输入、权重、激活都是矩阵
   - 物理/工程建模
     - 用矩阵描述状态变化、系统方程，如电路、力学系统
   - 图像处理
     - 图像本质上是像素矩阵，处理如滤波、变换时都是矩阵操作

4. 举例
   - 加法
     - 两矩阵维度相同
     - 对应位置的元素 `逐一相加`
   ```text
    前提: 矩阵 A 和 B 维度必须相同，如都是 2×3
    A = | 1 2 3 |      B = | 7 8 9 |
        | 4 5 6 |          | 1 2 3 |

    A + B = | 1+7  2+8  3+9 | = | 8 10 12 |
            | 4+1  5+2  6+3 |   | 5  7  9 |
   ```
   
   - 减法
     - 两矩阵维度相同
     - 对应位置的元素 `逐一相减`
   ```text
   前提: 矩阵 A 和 B 维度必须相同，如都是 2×3
   A = | 1 2 3 |      B = | 7 8 9 |
       | 4 5 6 |          | 1 2 3 |

   A - B = | 1-7  2-8  3-9 | = | -6 -6 -6 |
           | 4-1  5-2  6-3 |   | 3  3  3 |
   ```
   
   - 乘法
     - `A` 的 `列数` == `B` 的 `行数`
     - `A` 第 `i` 行 与 `B` 第 `j` 列的 `点积`
     - 乘法 = 行向量 × 列向量
   ```text
   前提: `A` 的 `列数` == `B` 的 `行数`
   A = 2x3        B = 3x2        A × B = 2x2

   A = | 1 2 3 |     B = | 7  8 |
       | 4 5 6 |         | 9 10 |
                         |11 12 |
   第 [i][j] 个元素，表示 `A` 第 `i` 行 与 `B` 第 `j` 列的 `点积`(内积):
   A × B = | (1 * 7 + 2 * 9 + 3 * 11) (1* 8 + 2 * 10 + 3 * 12)   |
           | (4 * 7 + 5 * 9 + 6 * 11) (4 * 8 + 5 * 10 + 6 * 12)  |
         = |  58    64 |
           |  139  154 |
   这个操作是最常见的，也是神经网络中最核心的运算之一
   ```

    - 转置
      - 将行列互换
    
    ```text
      A = | 1 2 |
          | 3 4 |
          | 5 6 |
   
      Aᵀ = | 1 3 5 |
           | 2 4 6 | 
    ```
   
- 逆矩阵(Inverse Matrix)
  其本质: 相当于 `矩阵除法`
  ```text
  如果有一个矩阵 `A`, 存在另一个矩阵 `B(也可以是同一个矩阵 `A`)`, 使得:
  A × B = B × A = I
  
  那么, 称 `B` 是 `A` 的逆矩阵, 记作:
  A⁻¹
  
  A⁻¹ * A = I
  
  `I` 是 `单位矩阵(对角线是 1，其余是 0 的方阵)`
  ```
  
  - 满足条件
    - `A` 是 `方阵`
    - `A` 的行或列不能线性相关(即 `秩 = n`)
    - 行列式 `det(A) ≠ 0`(`det(A) = 0` 叫 `奇异矩阵`)
  ```text
   A = | a b |
       | c d |
  
  满足行列式 `det(A) ≠ 0`: det(A) = a * d - b * c ≠ 0
  那么逆矩阵:
  A⁻¹ = (1 / (ad - bc)) × |  d -b |
                          | -c  a |  
  
  口诀: `对角换位, 副对角变负, 除以行列式`
  
  如何记:
   1. 主对角线(a 和 d)交换，副对角线(b 和 c)反号
   | a b |  (主对角线交换)-> | d b |  (副对角线反号)-> | d -b |
   | c d |                 | c a |                 | -c a |
  
   2. 求 det(A) = a * d - b * c
   
   3. 最后: 除以 `行列式`
   | d -b |  ÷ det(A)
   | -c a |   
   -> (1 / (ad - bc)) × |  d -b |
                        | -c  a |  
  ```
  
  - 高斯约旦消元
    - `行变换`，把左边的 `A` 变成单位矩阵 `I`
    - 让右边的 `I` 被变换成 `A⁻¹`
      - [ I | A⁻¹ ]
  
  - 举例(2 × 2)
  ```text
   A = | 2 1 |
       | 5 3 |
  
   I = | 1 0 |
       | 0 1 |
  
   A 的行列式: 2 * 3 - 1 * 5 = 1
  
   A × A⁻¹ = I
   推导过程(使用高斯约旦消元法):
   1. 构建增广矩阵 | A | I |
   [ 2 1 | 1 0 ]
   [ 5 3 | 0 1 ]
  
  2. 消元
    使用 `第1行第1列` 消去 `第2行第1列`
    - 第1行第1列元素: 2(非 1)
    - 归一: R1 <- 1/2 R1
    - 第1行: [ 1 0.5 | 0.5 0 ]
  
  2.1 消除 `第2行第1列`
    R2 <- R2 - (a21 / a11 = 5) * R1
       -> [ 5 3 | 0 1 ] - 5 * [ 1 0.5 | 0.5 0 ]
       -> [ 5 3 | 0 1 ] - [ 5 2.5 | 2.5 0]
       -> [ 0 0.5 | -2.5 1 ]
  
  2.2 使用 `第2行第2列` 消除 `第1行第2列`
      R1 <- R1 - (a12 / a22 = 1) * R2
         -> [ 1 0.5 | 0.5 0 ] - 1 * [ 0 0.5 | -2.5 1 ]
         -> [ 1 0.5 | 0.5 0 ] - [ 0 0.5 | -2.5 1 ]
         -> [ 1 0 | 3 -1 ]
  
  得到矩阵:
   [ 1   0 |    3 -1 ]
   [ 0 0.5 | -2.5  1 ] <- 未归一
  
  归一后得到矩阵:
   [ 1 0 |  3 -1 ]
   [ 0 1 | -5  2 ]
  
  所以得到 A⁻¹ = |  3  -1 |
                | -5   2 |
  
  使用公式:
   A⁻¹ = (1 / ( 2 * 3 - 1 * 5)) * |  3 -1 |
                                  | -5  2 |
       = |  3  -1 |
         | -5   2 |
  ```
  
  - 举例(3 × 3)
  ```text
   A⁻¹ = (1 / det(A) × adj(A)
   - det(A): A 的行列式
   - adj(A): A 的伴随矩阵(adjugate matrix)
  
  步骤:
  1. 求行列式 `det(A)`
  2. 求伴随矩阵 `adj(A)`
  3. A⁻¹ = (1 / det(A)) × adj(A)

  伴随矩阵 `adj(A)`:
   伴随矩阵 = 所有元素的 `代数余子式矩阵` 的 `转置`
   代数余子式(可以参考行列式里的计算): 
    - 去掉第 i 行和第 j 列后，剩下 2×2 矩阵的行列式
    - 加上符号: (–1)^(i+j)
    - 符号排列
      | +  -  + |
      | -  +  - |
      | +  -  + |

  例:
  A = | 1 2 3 |
      | 0 1 4 |
      | 5 6 0 |
  
  1. 求 det(A):
     det(A) = 1 * (1 * 0 - 4 * 6) - 2 * (0 * 0 - 4 * 5) + 3 * (0 * 6 - 1 * 5)
            = 1 * (0 - 24) - 2 * (0 - 20) + 3 * (0 - 5)
            = -24 - 2 * (-20) + 3 * (-5)
            = -24 + 40 - 15
            = 1
  
  2. 求伴随矩阵 `adj(A)`
  2.1 代数余子式矩阵: 
     | +(1 * 0 - 4 * 6) -(0 * 0 - 4 * 5) +(0 * 6 - 1 * 5) |
     | -(2 * 0 - 3 * 6) +(1 * 0 - 3 * 5) -(1 * 6 - 2 * 5) |
     | +(2 * 4 - 3 * 1) -(1 * 4 - 3 * 0) +(1 * 1 - 2 * 0) |
  
  -> |  (0 - 24) -(0 - 20) (0 -  5) |
     | -(0 - 18) (0 - 15) -(6 - 10) |
     |  (8 -  3) -(4 -  0) (1 -  0) |
  
  -> | -24  20 -5 |
     |  18 -15  4 |
     |   5  -4  1 |
  
  2.2 转置(将行列互换)
     | -24   18   5 |
     |  20  -15  -4 |
     |  -5   4    1 |
  
  2.3 adj(A)
     adj(A) = | -24   18   5 |
              |  20  -15  -4 |
              |  -5   4    1 |
     
  3.  A⁻¹ = (1 / det(A)) × adj(A)
   A⁻¹ = | -24   18   5 |
         |  20  -15  -4 |
         |  -5    4   1 |
  
  推导(使用高斯约旦消元):
  A = | 1 2 3 |
      | 0 1 4 |
      | 5 6 0 |
  
  I: 单位矩阵, 主对角线是 1，其余全是 0
  I = | 1 0 0 |
      | 0 1 0 |
      | 0 0 1 |
  
  1. 增广矩阵
    [ 1 2 3 | 1 0 0 ]
    [ 0 1 4 | 0 1 0 ]
    [ 5 6 0 | 0 0 1 ]
  
  1. 用 `第1行第1列`, 消除 `第2行第1列`、`第3行第1列`
  1.1 消除 `第2行第1列`
      `第2行第1列` 元素为 `0`: 无需消元
         
  1.2 消除 `第3行第1列`
      R3 <- R3 - (a31 / a11 = 5) * R1
         -> [ 5 6 0 | 0 0 1 ] - 5 * [ 1 2 3 | 1 0 0 ]
         -> [ 5 6 0 | 0 0 1 ] - [ 5 10 15 | 5 0 0 ]
         -> [ 0 -4 -15 | -5 0 1 ]
  
  现在矩阵:
  [  1  2   3 |  1 0 0 ]
  [  0  1   4 |  0 1 0 ]
  [  0 -4 -15 | -5 0 1 ]  
  
  2. 用 `第2行第2列`, 消除`第3行第2列`、`第1行第2列`
  
  2.1 消除`第3行第2列`
      R3 <- R3 - (a32 / a22 = -4) * R2
         -> [ 0 -4 -15 | -5 0 1 ] + 4 * [ 0 1 4 | 0 1 0 ]
         -> [ 0 -4 -15 | -5 0 1 ] + [ 0 4 -16 | 0 4 0 ]
         -> [ 0 0 1 | -5 4 1 ]
  
  2.2 消除`第1行第2列`
      R1 <- R1 - (a12 / a22 = 2) * R2
         -> [ 1 2 3 | 1 0 0 ] - 2 * [ 0 1 4 | 0 1 0 ]
         -> [ 1 2 3 | 1 0 0 ] - [ 0 2 8 | 0 2 0 ]
         -> [ 1 0 -5 | 1 -2 0 ]
  
  现在矩阵:
  [ 1 0 -5 |  1 -2 0 ]
  [ 0 1  4 |  0  1 0 ]
  [ 0 0  1 | -5  4 1 ]
  
  3. 接着消元
     使用 `第3行第3列`, 消除 `第1行第3列`、`第2行第3列`
     - 第3行第3列: 1
  3.1 消除 `第1行第3列`
     R1 <- R1 - (a13 / a33 = -5) * R3
        -> [ 1 0 -5 | 1 -2 0 ] + 5 * [ 0 0 1 | -5 4 1 ]
        -> [ 1 0 -5 | 1 -2 0 ] + [ 0 0 5 | -25 20 5 ]
        -> [ 1 0 0 | -24 18 5 ]
  
  3.2 消除 `第2行第3列`
      R2 <- R2 - (a23 / a33 = 4) * R3
         -> [ 0 1 4 | 0 1 0 ] - 4 * [ 0 0 1 | -5 4 1 ]
         -> [ 0 1 4 | 0 1 0 ] - [ 0 0 4 | -20 16 4 ]
         -> [ 0 1 0 | 20 -15 -4 ]
  
  矩阵: 
  [ 1 0 0 | -24  18  5 ]
  [ 0 1 0 |  20 -15 -4 ]
  [ 0 0 1 |  -5   4  1 ]
  
  A⁻¹ =  | -24   18   5 |
         |  20  -15  -4 |
         |  -5    4   1 |
  ```
  
  - 举例(线性方程组)
  ```text
    1x + 2y + 3z = 14
    0x +  y + 4z = 13
    5x + 6y + 0z = 17
  
  A = | 1 2 3 |  x = | x |   b = | 14 |
      | 0 1 4 |      | y |       | 13 |
      | 5 6 0 |      | z |       | 17 |
  
  求 A * x = b
    - A: 已知矩阵
    - b: 任意已知向量
    - x: 未知向量
  
    -> A * x = b
    -> A⁻¹ * A * x = A⁻¹ * b -- 必须左乘A⁻¹, 以保证乘法的顺序合法(矩阵乘法不满足交换律)
    -> (A⁻¹ * A) * x = A⁻¹ * b
    -> I * x = A⁻¹ * b  -- A⁻¹ * A = I
    -> x = A⁻¹ * b -- 单位矩阵乘任何向量不变
  
  1. 判断 A 是否是满足逆矩阵:
     det(A) = 1 * (1 * 0 - 4 * 6) - 2 * (0 * 0 - 4 * 5) + 3 * (0 * 6 - 1 * 5)
            = (0 - 24) - 2 * (0 - 20) + 3 * (0 - 5)
            = -24 + 40 - 15
            = 1
  
  2. 求伴随矩阵 `adj(A)`
  2.1 代数余子式矩阵: 
     | +(1 * 0 - 4 * 6) -(0 * 0 - 4 * 5) +(0 * 6 - 1 * 5) |
     | -(2 * 0 - 3 * 6) +(1 * 0 - 3 * 5) -(1 * 6 - 2 * 5) |
     | +(2 * 4 - 3 * 1) -(1 * 4 - 3 * 0) +(1 * 1 - 2 * 0) |
  
  -> | -24  20 -5 |
     |  18 -15  4 |
     |   5  -4  1 |
  
  -> |  (0 - 24) -(0 - 20) (0 -  5) |
     | -(0 - 18) (0 - 15) -(6 - 10) |
     |  (8 -  3) -(4 -  0) (1 -  0) |
  
  -> |  (0 - 24) -(0 - 20)  (0 -  5) |
     | -(0 - 18)  (0 - 15) -(6 - 10) |
     |  (8 -  3) -(4 -  0)  (1 -  0) | 
  
  -> | -24  20 -5 |
     |  18 -15  4 |
     |   5  -4  1 | 
  
  
  2.2 转置(将行列互换)
     | -24   18   5 |
     |  20  -15  -4 |
     |  -5    4   1 |
  
  adj(A) = | -24   18  5 |
           |  20  -15 -4 |
           |  -5    4  1 |
  
  3. 求 A 逆矩阵
     A⁻¹ = (1 / det(A)) * adj(A)
         = | -24   18  5 |
           |  20  -15 -4 |
           |  -5    4  1 |
  
   4. x = A⁻¹ * b
        = | -24  18  5 | * | 14 |
          |  20 -15 -4 |   | 13 |
          |  -5   4  1 |   | 17 |
        = | -24 * 14  18 * 13  5 * 17 |
          |  20 * 14 -15 * 13 -4 * 17 |
          |  -5 * 14   4 * 13  1 * 17 | 
        = | -336 +   234  +    85  |
          |  280 + (-195) +  (-68) |
          |  -70 +    52  +    17  |
        = | -17 |
          |  17 |
          |  -1 |
   最终: x = -17, y = 17, z = -1
  ``` 

  - 使用场景
    - 解线性方程组(A * x = b <-> x = A⁻¹ * b)
      - 解线性方程组用 `高斯-约旦消元法` 是更直接、更高效的办法
    - 图像处理(几何变换)
      - 图像旋转、缩放、镜像等都可以用矩阵表示
      - 反过来还原图像
    - 物理仿真/机器人学
      - 坐标系之间的变换(如机械臂末端 → 基座)
      - 将空间变换 `反向回来`
    - 计算偏导 `Jacobian` 的逆(深度学习 / 优化)
      - 训练神经网络时涉及到雅可比矩阵
      - 有些优化算法中用到了其逆或伪逆
    - 控制系统/状态空间建模
  
- 零矩阵
  `所有元素都是 0` 的矩阵, 记作: `O_m×n`, 其中 `m` 是 `行数`, `n` 是 `列数`
  ```text
   2 * 2 零矩阵:
   | 0 0 |
   | 0 0 |
  
  3 * 1 零矩阵:
   | 0 |
   | 0 |
   | 0 |
  
  1 * 3 零矩阵:
   | 0 0 0 |
  ```
  
  - 特性
    - 加法恒等元
      - 任意矩阵 `A + 0 = A`
    - 乘法吸收元
      - 任意矩阵 `A * 0 = 0`,  `0 * A = 0`(提前是维度合法)
    - 转置还是零矩阵
      - `(O)^T = 0`
    - 对角线元素也是 `0`
      - `不是单位矩阵`
    - 和自身相反
      - `-0 = 0`

- 矩阵的秩
  一个矩阵中 `最多` 有 `多少行或列` 是 `线性无关` 的
  - 矩阵的秩 = 这个矩阵所能 `贡献` 的 `最大独立信息量`
  - 记作: `rank(A)`
  ```text
  矩阵:
      A = | 1 2 |
          | 2 4 |
  发现:
     第 2 行 = 第1行 * 2(完全冗余)
  所以:
     这个矩阵虽然是 `2 * 2`, 但只有 1 行是 `新信息`, 因些:
     rank(A) = 1
  ```
  
  - 求矩阵的秩
    - 高斯消元(行简化阶梯型)
    - 把矩阵通过 `初等行变换` 变成 `阶梯型`, 然后 `统计非零行的个数`, 这个 `个数` 就是 `秩`
  ```text
  设:
        | 1 2 3 |
    A = | 2 4 6 |
        | 1 1 1 |
  
    1. 消元
       使用 `第1行第1列` 消除 `第2行第1列`、`第3行第1列`
    1.1 消除 `第2行第1列`
       R2 <- R2 - (a21 / a11 = 2) * R1
          -> | 2 4 6 | - 2 * | 1 2 3 |
          -> | 2 4 6 | - | 2 4 6 |
          -> | 0 0 0 |
  
    1.2 消除 `第3行第1列` 
        R3 <- R3 - (a31 / a11 = 1) * R1
           -> | 1 1 1 | - 1 * | 1 2 3 |
           -> | 0 -1 -2 |
  
    矩阵:
        | 1  2  3 |
        | 0  0  0 |
        | 0 -1 -2 |
    
    非零行有 `2` 个, 所以:
    rank(A) = 2
  ```
  
   - 行秩 = 列秩
     - 任意矩阵的 `行秩 = 列秩` 都相等
     - 选择按 `行`、或 `列` 来消元, 结果不会变
   
   - 秩的实际意义
     - 判定线性方程组的解
       - 设有 `A * x = b`
         - 如果增广矩阵 `[ A | b ]` 的秩 = 系数矩阵 `A` 的秩
           -> 有解
         - 如果再 = 未知数个数 -> 唯一解
         - 如果 < 未知数个数 -> 无穷解
         - 如果增广矩阵秩 > 系统矩阵秩 -> 无解
     - 判断列/行是否线性相关
       - 如果秩 < 列数 -> 列向量线性相关
       - 如果秩 = 列数 -> 所有列线线无关
     - 矩阵是否可逆
       - rank(A) = n (A 是 n × n 方阵)
         -> A 可逆
       - 否则 -> A 不可逆(退化矩阵)
   - 总结
     - m × n
       - 最大秩: min(m, n)
       - 最小秩: 0(全零矩阵)
     - 全零矩阵
       - 最大秩: 0
     - 单位矩阵In
       - 最大秩: n
     - 有重复行/列
       - 最大秩: 秩会下降
   
   - 举例
     - 线性方程组
     ```text
         →    →
     A * x  = b
     
     A * x = b 是正确的标准表达式;
     加 `→` 是为了强调 `x` 和 `b` 是 `向量`

     其中: A: m × n 系数矩阵
          →
          x: 未知变量列向量(长度为 n)
          →
          b: 常数列向量(长度 m)
 
     1. 构造两个矩阵
        - 系数矩阵 A
                        →      →
        - 增广矩阵 [ A | b ]: 把 b 拼到 A 的最后一列
     
     2. 分别求两个矩阵的秩
        - rank(A): 系数矩阵的秩
                     →
        - rank([ A | b ]): 增广矩阵的秩
     
     3. 根据这两个秩，判断解的情况
        秩关系                              解的类型
                             →
        rank(A) = rank([ A | b ]) = n      唯一解
                             →    
        rank(A) = rank([ A | b ]) < n      无穷多解
                             →       
        rank(A) < rank([ A | b ])          无解
     
     4. 为什么用秩能判断解的情况？
        - 每一行 = 一个 `独立方程`
        - 每一个变量 = 一个 `自由维度`
        如果 `独立方程数量 < 变量数`，就有 `冗余变量`(→ `无限解`)
        如果 `增广矩阵秩 > 系数秩` → 代表 `右边的值矛盾了`(→ `无解`)
     
     ```
     
     ```text
     方程组:
          x + 2y +  z = 4
         2x + 4y + 2z = 8
          x +  y +  z = 5
     
     1. 构造两个矩阵
     1.1 系数矩阵 A
            | 1 2 1 |     →   | 4 |
        A = | 2 4 2 |     b = | 8 |
            | 1 1 1 |         | 5 |
     
     1.2 增广矩阵
              →       →
        [ A | b ]: 把 b 拼到 A 的最后一列   
              
              →     | 1 2 1 4 |
        [ A | b ] = | 2 4 2 8 |
                    | 1 1 1 5 |
     
     2. 分别求两个矩阵的秩
     2.1 rank(A): 系数矩阵的秩
         R2 ← R2 - 2 × R1 → 第 2 行变成 0
         R3 ← R3 - R1 → 第 3 行变成 [0, -1, 0]
     
     得到阶梯型:
            | 1  2 1 |
        A = | 0  0 0 |
            | 0 -1 0 |
     非零行数 = 2, 所以:
        rank(A) = 2
     
                →
     2.2 rank([ A | b ]): 增广矩阵的秩
         R2 ← R2 - 2 × R1 → 第 2 行变成 [0, 0, 0, 0]
         R3 ← R3 - R1 → 第 3 行变成 [0, -1, 0, 1]
     
     得到阶梯型:
              →     | 1  2 1 4 |
        [ A | b ] = | 0  0 0 0 |
                    | 0 -1 - 1 |
     
     非零行数 = 2, 所以:
        rank(A) = 2
     
     3. 根据这两个秩，判断解的情况
                             →
        rank(A) = rank([ A | b ]) = 2  
        变量个数 n = 3
     
     结论: 无穷多解(自由变量 = 3 - 2 = 1 个)
     
     为什么是无穷多解, 使用消元后得到结果
     | 1 0 1  6 |
     | 0 1 0 -1 |
     | 0 0 0  0 |
     
     -> x + z = 6
        y = -1
        自由变量: z = t (任意实数)
     
     -> z = t
        y = -1
        z = 6 - t
     
     -> | x | = | 6 - t |
        | y |   |  -1   |
        | z |   |   t   |
     
     最终: 有无穷多个解, 每个解由自由变量 `𝑡` 决定
     自由变量: 某个变量没有被唯一约束住(消元过程中没有主元)
     ```

- 解线性方程组
  方程式: `A * x = b`
   - A: 已知矩阵
   - b: 任意已知向量
   - x: 未知向量
  
  - 高斯消元法
    - 通过 `初等行变换` 把 `增广矩阵` 化为 `上三角矩阵`, 再从 `最后一行` 开始 `回代` 求出每个未知数
    - 步骤
      - 将 `A` 和 `b` 构建为增广矩阵 `| A | b |`
      - 使用 `初等行变换` 把矩阵变成 `上三角形式`
      - 从 `底部往上` `回代` 求解各变量
    - 优点
      - 通用, 适用于任意线性方程组
      - 空间复杂度低
    - 缺点
      - 需要 `手动回代`(不能一步得到解)
      - 无法直接判断 `自由变量` (但可以结合秩来判断)

  - 高斯-约旦消元法(Gauss-Jordan)
    - 继续消元，`不仅上三角`, 还 `消成单位矩阵`, 这样 `右侧` 就是直接的解
    - 步骤
      - 将 `A` 和 `b` 构建为增广矩阵 `| A | b |`
      - 使用 `初等行变换` 把矩阵 `A` 变成 `单位矩阵I`
      - 增广后的右边 `b` 就是解向量 `x`
    - 优点
      - 直接得到所有变量的值(无回代)
      - 可以直观看出无解、唯一解、无穷多解
      - 自由变量一目了然
    - 缺点
      - 运算量稍大(全部消为单位矩阵)
  
  - 逆矩阵法(Matrix Inversion)
    - 如果矩阵 `A` 是 `可逆的`(`det(A) ≠ 0`), 可以通过求 `A⁻¹` 得到解: `x = A⁻¹ * b`
    - 步骤
      - 计算 `行列式`(`det(A)`), 确认 `矩阵可逆`
      - 计算 `伴随矩阵 adj(A)`, `伴随矩阵` = 所有元素的 `代数余子式矩阵` 的 `转置`
        - 公式
        ```text
           A⁻¹ = (1 / det(A)) * adj(A)
        ```
        - 计算 `代数余子式矩阵`
          - 去掉 `第 i 行` 和 `第 j 列` 后，剩下 `2×2` 矩阵的 `行列式`
          - 加上符号: `(–1)^(i+j)`
          - 符号排列
          ```text
          | +  -  + |
          | -  +  - |
          | +  -  + |
          ```
        - 转置: 把矩阵的 `行变成列`(即 `A[i][j]` -> `A[j][i]`)
      - 计算 `x = A⁻¹ * b`
    - 特别提示
      - 对 `2 * 2` 的逆矩阵有专用公式
      ```text
        `det(A) ≠ 0`: det(A) = a * d - b * c ≠ 0
        A = | a b |  =>  A⁻¹ = (1/ det(A)) * |  d -b |  =>  A⁻¹ = (1 / (ad - bc)) * |  d -b |
            | c d |                          | -c  a |                              | -c  a |
        
       如何记:
       主对角线(a 和 d)交换，副对角线(b 和 c)反号
       | a b |  (主对角线交换)-> | d b |  (副对角线反号)-> | d -b |
       | c d |                 | c a |                 | -c a |
       ```
    - 优点
      - 理论性强, 体现矩阵乘法和线代美感
      - 如果要解多个右端向量（如Ax = b1,b2,...），效率很高
    - 缺点
      - 只有当矩阵 `A` 可逆时才成立
      - 求逆矩阵计算成本高, 对 `n > 3` 不推荐手算
      - 对于稀疏矩阵不友好
    - 场景
      - 手算小规模方程组 (2x2, 3x3): 高斯或高斯-约旦
      - 需要分析解的个数、自由变量: 高斯-约旦最清晰
      - 编程实现通用解法: 高斯消元（+回代）
      - 解多个 b 向量（多个方程组）: 逆矩阵法
      - 稀疏矩阵或高维系统: 避免用逆矩阵法
  
  
  