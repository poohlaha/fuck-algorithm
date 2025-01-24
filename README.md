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

### 算法
分为 `基本排序算法`、`高效排序算法`、`分布式排序算法`、`特殊排序算法` 等

### 基本排序算法
- 冒泡排序(Bubble Sort)
1. 时间复杂度: O(n2), 空间复杂度 O(1)(排序在原数组上进行，不需要额外的辅助空间)
2. 适合小规模数据; 效率较低
3. 步骤
   - 初始化
     - 定义 `start = 0`, `end = len - 1`
     - 定义 `loop` 循环结束条件 `swapped = false`
   - 冒泡
     - 比较从 `start` 到 `end` 是否交换
     - 如果没有交换, 则循环结束
     - 如果有交换
       - 尾部再缩短一位 `end -= 1`
       - 重置循环结束条件 `swapped = false`
4. 代码    
```rust
  fn bubble_sort() {
    let arr = vec![4, 2, 2, 2, 8, 8, 3, 3, 1, 9];
    
    // 1. 初始化
    let mut start = 0;
    let end = arr.len() - 1;
    let mut swapped = false;
  
    loop {
        // 2. 冒泡
        for i in start .. end {
          // 2.1 交换
          if arr[i] > arr[i + 1] {
            arr.swap(i, i+ 1);
            swapped = true
          }
        }
      
        // 2.3 如果没有交换, 则循环结束
        if !swapped {
          break;
        }
      
        // 2.2 如果有交换
        end -= 1; // 2.2.1 尾部再缩短一位 `end -= 1`
        swapped = false // 2.2.2 重置循环结束条件 `swapped = false`
    }
  }
```

- 插入排序(Insertion Sort)
1. 时间复杂度: O(n2), 空间复杂度 O(1)(排序在原数组上进行，不需要额外的辅助空间)
2. 对接近有序的数据效率高；实现简单
3. 步骤
   - 定义索引 `i` 从 `1` 开始
   - 把 `i` 赋值给 `j`, 每次比较 `j` 之前的元素
4. 代码
```rust
   fn insertion_sort() {
      let arr = vec![4, 2, 2, 2, 8, 8, 3, 3, 1, 9];
    
      for i in 1 .. arr.len() {
          let mut j = i ;

          while j > 0 && arr[j - 1] > arr[j] {
              arr.swap(j - 1, j);
              j -= 1;
          }
      }
   }
```

- 选择排序(Selection Sort)
1. 时间复杂度: O(n2), 空间复杂度 O(1)(排序在原数组上进行，不需要额外的辅助空间)
2. 数据量小时简单有效；移动次数较少
3. 步骤
    - 定义索引 `i` 从 `0` 开始
    - 把 `i` 赋值给 `j`, 每次选取= `j` 之后的最小或最大的元素
4. 代码
```rust
  fn selection_sort() {
     let arr = vec![4, 2, 2, 2, 8, 8, 3, 3, 1, 9];
    
     let n = arr.len();
    
     // 当为 n-1 时, 数组已排序好, 不需要再进行比较
     for i in 0 .. n - 1 {
         let mut min_index = i;
         
         for j in i + 1 .. n {
             if arr[j] < arr[min_index] {
                 min_index = j
             }
         }
         
         if min_index != i {
             arr.swap(i, min_index)
         }
     }
  }
```
    
### 高效排序算法
- 快速排序(Quick Sort)
1. 时间复杂度: O(n log n), 空间复杂度 O(log n)(递归栈空间)
2. 实现复杂，但一般是最快的排序算法
3. 步骤
    - 选取基准(Pivot)
      - 获取 `随机索引`
      - 将基准移到数组末尾, 交换 `随机索引值` 和 `数组最后一值`
    - 分区(Partition)
      - 定义边界指针 `i = 0`, 表示 `当前小于基准的部分的右边界, 下一个小于基准的元素应该放置的位置`
      - 遍历数组中的每个元素(除最后的基准)，与基准(此时为最后一个值)比较
      - 基准回到正确位置
        - 最后将 `基准值` 与 `arr[i]` 交换
      此时分区完成，数组分为两部分:
      - 左侧部分, 小于基准
      - 基准位置
      - 右侧部分, 大于基准
    - 递归排序
      - 对基准两侧的子数组分别重复上述过程，直到数组长度为 1 或 0
    - 合并结果  
4. 代码
```rust
  use rand::Rng;

  fn main() {
      let arr = vec![4, 2, 2, 2, 8, 8, 3, 3, 1, 9];
      quick_sort(arr);
  }

  fn quick_sort(arr: &mut[usize]) {
    let n = arr.len(); 
      
    let index = partition(arr); // // 分区
    let left = quick_sort(&mut arr[..index]); // 分区后基准左边部分
    let right = quick_sort(&mut arr[(index + 1)..]); // 分区后基准右边部分
  }

  fn partition(arr: &mut [usize]) -> usize {
      let n = arr.len();
      
      // 1. 获取基准(Pivot)
      // 1.1 获取 `随机索引` 
      let index = rand::thread_rng().gen_range(0..n);

      // 1.2 将基准移到数组末尾, 交换 `随机索引值` 和 `数组最后一值`
      arr.swap(index, n - 1);

      // 2 分区(Partition)
      // 2.1  定义边界指针 `i = 0`, 表示 `当前小于基准的部分的右边界, 下一个小于基准的元素应该放置的位置`
      let mut i = 0;

      // 2.2 遍历数组中的每个元素(除最后的基准)，与基准(此时为最后一个值)比较
      for j in 0 .. n - 1 {
          if arr[j] < arr[n - 1] {
              // 交换位置
              arr.swap(i, j);
              i += 1;
          }
      }

      // 2.3 基准回到正确位置
      arr.swap(i, index);
      i
  }
```

- 归并排序(Merge Sort)
1. 时间复杂度: O(n log n), 空间复杂度 O(n)(额外的临时数组)
2. 稳定排序，适合链表，适合外部排序(大数据)
3. 步骤
    - 分解(Divide)
      - 将数组递归分成两半，直到每个子数组只有一个元素。
        - 找到数组的中间位置 mid，将数组分为左半部分和右半部分。
        - 对左半部分递归执行分解操作。
        - 对右半部分递归执行分解操作。
    - 合并(Merge)
      - 将两个已经排序的子数组合并成一个有序数组
        - 准备两个指针分别指向左、右子数组的开头。
        - 比较两个指针所指的元素，将较小的元素放入结果数组。
        - 移动对应的指针，继续比较。
        - 如果某一子数组的元素全部放入结果数组，则将另一个子数组的剩余部分直接追加到结果数组。
    - 排序(Sort and Merge)
        - 在递归返回的过程中，逐层合并排序好的子数组，最终得到排序后的完整数组。
          - 在最底层的递归中，两个只有一个元素的子数组（天然有序）合并成一个有序数组。
          - 返回到上一层递归时，合并两个已经排序的子数组。
          - 继续合并直到处理完整个数组。
4. 代码
```rust
   fn main() {
      let arr = vec![4, 2, 2, 2, 8, 8, 3, 3, 1, 9];
      merge_sort(arr);
   }

   fn merge_sort(arr: &mut [usize]) -> Vec<usize> {
       let n = arr.len();

       // 1. 分解(Divide)
       // 1.1 将数组递归分成两半，直到每个子数组只有一个元素。
       // 1.1.1 找到数组的中间位置 mid，将数组分为左半部分和右半部分
       let mid = n / 2;
       let left = merge_sort(&mut arr[..mid]);
       let right = merge_sort(&mut arr[(mid + 1)..]);

       // 2. 合并(Merge)
       merge(left, right)
   }

    fn merge(left: Vec<usize>, right: Vec<usize>) -> Vec<T> {
       let results = Vec::with_capacity(left.len() + right.len());
       let left_iter = left.into_iter(); 
       let right_iter = right.into_iter();

       // 2.1 将两个已经排序的子数组合并成一个有序数组
       // 2.1.1 准备两个指针分别指向左、右子数组的开头 
       let mut left_val = left_iter.next();
       let mut right_val = right_iter.next();
       
       // 2.1.2 比较两个指针所指的元素，将较小的元素放入结果数组
       while left_val.is_some() && right_val.is_some() {
           if left_val <= right_val {
               results.push(left_val.unwrap());
               left_val = left_iter.next();
           } else {
               results.push(right_val.unwrap());
               right_val = right_iter.next();
           }
       } 
        
       // 2.1.3 如果某一子数组的元素全部放入结果数组，则将另一个子数组的剩余部分直接追加到结果数组
        if let Some(lv) = left_val {
            results.push(lv);
            results.extend(left_iter);
        }

        if let Some(rv) = right_val {
            results.push(rv);
            results.extend(right_iter);
        }
        
        results
   }
```

- 堆排序(Heap Sort)
1. 时间复杂度: O(n log n), 空间复杂度 O(1)(排序在原数组上进行，不需要额外的辅助空间)
2. 堆是一棵 `完全二叉树`, 不稳定排序，常用于需要高效排序的场景
    - 大顶堆：每个节点的值都 `>=` 其子节点的值
    - 小顶堆：每个节点的值都 `<=` 其子节点的值
    - 堆排序从 `堆尾` 逐步 `向前收缩`，最终排序完成
    - 从 `1` 开始是因为我们交换堆顶(索引 `0`)和堆尾时，堆的大小变为 `len - 1`，所以从 `1..len` 开始处理
      - 当只剩下一个元素时(`i == 0`)，它已经是最小值，不需要再排序了
3. 步骤
    - 构建初始堆
      - 对数组元素进行调整，使其满足堆的性质(大顶堆或小顶堆)
        - 创建堆从 `heap_size / 2` 开始, 因为叶子节点的索引范围从 `heap_size / 2` 到 `heap_size - 1`
        - 从最后一个非叶子节点开始逐层向上调整堆
          - 为什么从 `heap_size / 2` 开始?
            - 因为二叉堆的叶子节点本身已经满足堆的性质（没有子节点需要比较）。
            - 从最后一个非叶子节点（索引为 `heap_size / 2 - 1`）开始调整堆，确保整个堆满足大顶堆的性质。
    - 堆排序(堆顶元素(0 索引处)始终是当前堆中最大的值(对于大顶堆))
      - 交换 `堆顶(最大值)` 与`堆尾` 元素
        - 将当前堆中的最大值(堆顶元素)放到数组末尾，并将该位置视为已排序部分，剩下的部分继续调整为堆
        - 将最大值（堆顶）移到未排序区域的末尾位置 `i`，从而逐步生成一个从小到大的有序数组
        - 重复上述过程，直到堆中只剩下一个元素
      - 长度 `n - 1`
      - 下沉, 始终从 `0` (最顶端) 开始 
        - 传入 `0` 是因为每次交换后，堆顶元素(索引 `0` 的元素)可能不再是剩余堆的最大值，需要从堆顶开始重新调整堆  
    - 所有元素都排序完成
4. 代码
```rust
   fn heap_sort() {
      let arr = vec![4, 2, 2, 2, 8, 8, 3, 3, 1, 9];
      let mut n = arr.len();
    
      // 1. 构建初始堆(大顶堆: 每个节点的值都 `>=` 其子节点的值)
      // 1.1 创建堆从 `heap_size / 2` 开始, 因为叶子节点的索引范围从 `heap_size / 2` 到 `heap_size - 1`
      for i in (0.. n / 2).rev() {
          sink(&mut arr, n, i) // 1.1.2 从最后一个非叶子节点开始逐层向上调整堆
      }
    
      // 2. 堆排序
      for i in (1..n).rev() {
          arr.swap(0, i); // 2.1 交换 `堆顶(最大值)` 与`堆尾` 元素
          n -= 1; // 2.2 长度 `n - 1`
          sink(&mut arr, n, 0); // 2.3 下沉, 始终从 `0` (最顶端) 开始 
      }
   }

    // 左子节点
    fn left(i: usize) -> usize {
        2 * i + 1
    }

    // 右子节点
    fn right(i: usize) -> usize {
        2 * i + 2
    }

   // 上浮
   fn sink(arr: &mut [usize], size: usize, mut index: usize) {
       while index <= size - 1 {
           let left_index = left(index);
           let right_index = right(index);

           let mut max_index = index;
           if left < size && arr[left_index] > arr[max_index] {
               max_index = left_index;
           }

           if right < size && arr[right_index] > arr[max_index] {
               max_index = right_index;
           }

           if max_index == index {
               break;
           }
           
           // 2. 交换堆顶与末尾元素
           arr.swap(max_index, index);
           index = max_index;
       }
   }
```
    
### 分布式排序算法
- 桶排序(Bucket Sort)
- 计数排序(Counting Sort)
- 基数排序(Radix Sort)

### 特殊排序算法
- 鸡尾酒排序(Cocktail Shaker Sort)
  又称 `冒泡改进算法` 或 `双向冒泡排序`, 是冒泡排序的改进版。
1. 时间复杂度: O(n2), 空间复杂度 O(1)(排序在原数组上进行，不需要额外的辅助空间)
2. 比经典冒泡排序效率更高; 每轮需要双向扫描，增加了操作复杂度
3. 步骤
  - 初始化
    - 定义 `start = 0`, `end = len - 1`
    - 定义 `while` 循环结束条件 `swapped = false`
  - 冒泡
    - 正向冒泡
      - 比较从 `start` 到 `end` 是否交换
      - 如果没有交换, 则循环结束
      - 如果有交换
        - 尾部再缩短一位 `end -= 1`
        - 重置循环结束条件 `swapped = false`
    - 反向冒泡
      - 比较从 `end` 到 `start` 是否交换
      - 尾部(此时是 `start`)再增加一位 `start += 1`
4. 代码
```rust
  fn cocktail_sort() {
    let arr = vec![4, 2, 2, 2, 8, 8, 3, 3, 1, 9];
    
    // 1. 初始化
    let mut start = 0;
    let end = arr.len() - 1;
    let mut swapped = true;
  
    // 2. 冒泡
    while swapped {
      
      // 2.1 正向冒泡, 比较从 `start` 到 `end` 是否交换
      for i in start .. end {
        // 2.1 交换
        if arr[i] > arr[i + 1] {
          arr.swap(i, i+ 1);
          swapped = true
        }
      }

      // 2.1.1 如果没有交换, 则循环结束
      if !swapped {
        break;
      }
      
      // 2.1.2 如果有交换
      end -= 1; // 尾部再缩短一位 `end -= 1`
      swapped = false; // 重置循环结束条件 `swapped = false`
      
      // 2.2 反向冒泡, 比较从 `end` 到 `start` 是否交换
      for i in (start .. end).rev() {
        if arr[i] > arr[i + 1] {
          arr.swap(i, i+ 1);
          swapped = true
        }
      }
      
      // 2.2.1 尾部(此时是 `start`)再增加一位 `start += 1`
      start += 1;
    }
  }
```

- 希尔排序(Shell Sort)