/*!
    归并排序(Merge Sort)
    1. 思想: 将数组分成两部分，分别排序后合并(分治法的一种)
    2. 时间复杂度:
       - 最优: O(n log n)
       - 最差：O(n log n)
       - 平均: O(n log n)
    3. 空间复杂度: O(n) (额外的临时数组)
    4. 特点: 稳定排序，适合链表，适合外部排序(大数据)

     步骤:
     1. 分解(Divide)
       将数组递归分成两半，直到每个子数组只有一个元素。
       - 找到数组的中间位置 mid，将数组分为左半部分和右半部分。
       - 对左半部分递归执行分解操作。
       - 对右半部分递归执行分解操作。

     2. 合并(Merge)
       将两个已经排序的子数组合并成一个有序数组。
       - 准备两个指针分别指向左、右子数组的开头。
       - 比较两个指针所指的元素，将较小的元素放入结果数组。
       - 移动对应的指针，继续比较。
       - 如果某一子数组的元素全部放入结果数组，则将另一个子数组的剩余部分直接追加到结果数组。

     3. 排序(Sort and Merge)
        在递归返回的过程中，逐层合并排序好的子数组，最终得到排序后的完整数组。
        - 在最底层的递归中，两个只有一个元素的子数组（天然有序）合并成一个有序数组。
        - 返回到上一层递归时，合并两个已经排序的子数组。
        - 继续合并直到处理完整个数组。

      示例:
      假设数组为 [38, 27, 43, 3, 9, 82, 10]:

      Step 1: 分解
      1. 初始数组分成两部分
         Left: [38, 27, 43], Right: [3, 9, 82, 10]
      2. 对 Left: [38, 27, 43] 继续分解:
         - Left: [38], Right: [27, 43]
         - Left: [27], Right: [43]
      2. 对 Right: [3, 9, 82, 10] 继续分解:
         - Left: [3, 9], Right: [82, 10]
         - Left [3], Right: [9]
         - Left: [82], Right: [10]

      Step 2: 合并
      1. Left: [38, 27, 43]
         - 从 `最底层` 开始合并 [27] 和 [43]
           Merge: [27] + [43] => [27, 43]
         - 合并 [38] 和 [27, 43]
           Merge: [38] + [27, 43] => [27, 38, 43]
      2. Right: [3, 9, 82, 10]
         - 从 `最底层` 开始合并 [3] 和 [9]
           Merge: [9] + [9] => [3, 9]
         - 从 `最底层` 开始合并 [82] 和 [10]
           Merge: [82] + [10] => [10, 82]
         - 合并 [3, 9] 和 [10, 82]
           Merge: [3, 9]] + [10, 82] => [3, 9, 10, 82]

      Step 3: 整合
         最后将 [27, 38, 43] 和 [3, 9, 10, 82] 合并
         Merge: [27, 38, 43] + [3, 9, 10, 82] => [3, 9, 10, 27, 38, 43, 82]

      归并排序流程图(可视化)
      1. 分解阶段:
         Original Array: [38, 27, 43, 3, 9, 82, 10]
         Split -> [38, 27, 43] | [3, 9, 82, 10]
         Split -> [38] | [27, 43] | [3, 9] | [82, 10]
         Split -> [27] | [43] | [3] | [9] | [82] | [10]

      2. 合并阶段：
         Merge: [27] + [43] -> [27, 43]
         Merge: [38] + [27, 43] -> [27, 38, 43]
         Merge: [3] + [9] -> [3, 9]
         Merge: [82] + [10] -> [10, 82]
         Merge: [3, 9] + [10, 82] -> [3, 9, 10, 82]
         Merge: [27, 38, 43] + [3, 9, 10, 82] -> [3, 9, 10, 27, 38, 43, 82]
*/
pub fn merge_sort<T: Ord + Clone>(arr: &mut [T]) -> Vec<T> {
    let n = arr.len();
    if n == 0 || n == 1 {
        return arr.to_vec();
    }

    // Step 1: 分解
    let mid = n / 2;
    let left = merge_sort(&mut arr[..mid]);
    let right = merge_sort(&mut arr[mid..]);

    // Step 2: 合并
    merge(left, right)
}

fn merge<T: Ord>(left: Vec<T>, right: Vec<T>) -> Vec<T> {
    let mut results = Vec::with_capacity(left.len() + right.len());
    let mut left_iter = left.into_iter();
    let mut right_iter = right.into_iter();

    let mut left_val = left_iter.next();
    let mut right_val = right_iter.next();

    // 当前两段有序子数组的比较与合并
    while left_val.is_some() && right_val.is_some() {
        if left_val <= right_val {
            results.push(left_val.unwrap());
            left_val = left_iter.next();
        } else {
            results.push(right_val.unwrap());
            right_val = right_iter.next();
        }
    }

    // 将剩余部分加入结果数组, 只会是单值
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
