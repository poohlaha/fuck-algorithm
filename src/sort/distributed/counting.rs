/*!
    计数排序(Counting Sort)
    一种非比较排序算法，适用于已知范围内的整数排序，尤其是当数据范围较小且数据量较大时，它能够高效地排序。
    1. 思想: 根据数据范围创建计数数组，统计每个元素出现的次数，然后输出有序数据
    2. 时间复杂度: O(n + k)
       - n: 输入数组的元素个数
       - k: 数组元素的范围（最大值）
       - 由于计数排序是基于计数的，因此时间复杂度主要由输入数组大小 n 和数据范围 k 决定
    3. 空间复杂度: O(k + n)
       - count 数组需要 O(k) 空间，输出数组需要 O(n) 空间
    4. 特点:
       - 稳定性：计数排序是稳定的，意味着如果两个元素相等，它们在排序后的顺序与排序前相同
       - 适用场景：当数据范围较小且是整数时，计数排序比其他基于比较的排序算法（如快速排序、归并排序）更高效
       - 限制：不能处理浮动数或负数，对于大范围的数字，空间和时间复杂度都会变得不经济
         非比较排序，适合整数范围小的数据；不适合小数或范围大的数据

    步骤:
     1. 初始化计数数组: 假设输入数组的元素范围是从 0 到 k，则创建一个计数数组 count，长度为 k + 1，用来记录每个元素出现的次数
     2. 统计元素出现次数: 遍历输入数组，对于每个元素 x，将 count[x] 的值加 1，表示该元素 x 出现的次数
     3. 计算累计计数: 计算计数数组的累计值，即使得 count[i] 存储的是小于等于 i 的元素总数。这是为了将每个元素正确地放置到排序后的数组中
     4. 排序结果: 通过累计计数数组，反向遍历输入数组，将每个元素放到排序后的结果数组的正确位置
     5. 输出排序结果: 最后，输出排序后的数组

    举例:
    假设有数组: [4, 2, 2, 8, 3, 3, 1]
    范围 `k` 是 `8`，即 `元素最大值` 为 `8`。
    - 初始化计数数组 `count`，大小为 `k + 1 = 9`，初始值都为 `0`,
      - count = [0, 0, 0, 0, 0, 0, 0, 0, 0]
    - 统计输入数组元素出现次数
      - count[4] += 1  → count = [0, 0, 0, 0, 1, 0, 0, 0, 0]
      - count[2] += 1  → count = [0, 0, 1, 0, 1, 0, 0, 0, 0]
      - count[2] += 1  → count = [0, 0, 2, 0, 1, 0, 0, 0, 0]
      - count[8] += 1  → count = [0, 0, 2, 0, 1, 0, 0, 0, 1]
      - count[3] += 1  → count = [0, 0, 2, 1, 1, 0, 0, 0, 1]
      - count[3] += 1  → count = [0, 0, 2, 2, 1, 0, 0, 0, 1]
      - count[1] += 1  → count = [0, 1, 2, 2, 1, 0, 0, 0, 1]
     - 计算累计计数数组
      - count[1] = count[0] + count[1] → count = [0, 1, 2, 2, 1, 0, 0, 0, 1]
      - count[2] = count[1] + count[2] → count = [0, 1, 3, 2, 1, 0, 0, 0, 1]
      - count[3] = count[2] + count[3] → count = [0, 1, 3, 5, 1, 0, 0, 0, 1]
      - count[4] = count[3] + count[4] → count = [0, 1, 3, 5, 6, 0, 0, 0, 1]
      - count[5] = count[4] + count[5] → count = [0, 1, 3, 5, 6, 6, 0, 0, 1]
      - count[6] = count[5] + count[6] → count = [0, 1, 3, 5, 6, 6, 6, 0, 1]
      - count[7] = count[6] + count[7] → count = [0, 1, 3, 5, 6, 6, 6, 6, 1]
      - count[8] = count[7] + count[8] → count = [0, 1, 3, 5, 6, 6, 6, 6, 7]
     - 反向遍历输入数组，并将元素放到排序结果数组的正确位置
       - result = [1, 2, 2, 3, 3, 4, 8]
     - 输出排序结果
       - [1, 2, 2, 3, 3, 4, 8]

     1. 为什么要反向遍历数组？
        在计数排序中，反向遍历输入数组的原因与 `稳定性` 有关。
          - 稳定性：在计数排序中，保证相同值的元素在排序后能保持原有的相对顺序。这意味着如果有多个相同的元素，它们在排序后应出现在相同的位置，并且它们的相对顺序不变。
          - 当我们按从后往前的顺序遍历输入数组时，确保在每次放入排序结果数组时，已经处理过的相同元素的顺序不会被打乱。这是因为，当多个相同元素的值被放入结果数组时，它们会按照输入数组中原来的顺序放入（即，先放入的元素会在结果数组的较低索引位置）。
          - 如果我们从前往后遍历数组，每次放入的元素可能会覆盖已经放入的相同元素，从而打乱相同元素的相对顺序。因此，反向遍历保证了稳定排序。

     2. 当反向遍历数组时，如何计算其值？(以 arr[0] = 4 为例)
        - 统计元素出现次数：首先，我们统计每个元素出现的次数
          - 假设我们已经创建了 count 数组并计算了每个元素的累计计数
        - 反向遍历数组并放入结果数组
          - 通过 count 数组，我们可以知道每个元素应该放到结果数组的哪个位置
          - 假设我们反向遍历输入数组中的元素，当前处理的元素是 4
          - count[4] 存储的是小于等于 4 的元素个数，即元素 1, 2, 3 和 4 的总数
          - count[4] = 6，这意味着在排序后，元素 4 应该放到结果数组中索引为 5 的位置(因为数组是从 0 开始的，所以位置是 6 - 1 = 5)
        - 更新计数数组
          - 当我们将 4 放入结果数组中的正确位置后，我们需要更新 count[4]，减 1，表示已经处理了一个 4，这样在处理下一个相同的 4 时，它会被放到前一个位置，确保相同的元素保持顺
      举例:
       假设 arr = [4, 2, 2, 8, 3, 3, 1]，count 数组的累积值如下
         - count = [0, 1, 3, 5, 6, 6, 6, 6, 7]
       当我们开始反向遍历 arr:
         - 第一次遍历到元素 4 时，count[4] = 6，我们把 4 放到结果数组的索引位置 5
           - 计算位置：count[4] - 1 = 6 - 1 = 5
           - output[5] = 4
           - 更新 count[4]：count[4] -= 1，变为 count[4] = 5
         - 接下来，处理下一个 4 时
           - count[4] = 5，将其放置在位置 4（count[4] - 1）
           - 更新 count[4]：count[4] -= 1，变为 count[4] = 4
           依此类推，所有的元素都会根据 count 数组确定它们的正确位置
       ps:
          反向遍历的关键
           - 确保稳定性：通过反向遍历，我们能保证元素相对顺序不变
           - 计算位置：count[x] 表示小于等于 x 的元素总数，通过 count[x] - 1 得到当前元素应该放入结果数组的位置
         次数的计算
           - 在反向遍历时，我们并不需要直接通过 count[x] 获取元素的出现次数，而是通过 count[x] 计算出元素放置的位置，然后每次放置该元素时，count[x] 会减 1，确保同样的元素按照原始顺序放置。

     累计计数数组如何计算？
      在 `计数排序` 中，我们通过以下步骤计算累计计数数组(count 数组):
      - 统计每个元素出现的次数：首先，我们统计每个元素在输入数组中出现的次数，构建一个 count 数组，其中 count[i] 表示元素 i 在输入数组中出现的次数。
      - 计算累计计数：累计计数的目的是，计算 `小于或等于某个元素的所有元素的总数`。也就是说，count[i] 表示的是 `小于等于 i 的元素的个数` 。为了能够将元素放置到正确的位置，我们需要将 count[i] 变成元素 i 在最终排序数组中的位置。

     为什么累计计数值对应索引位置？
       - 通过累计计数，我们能够知道一个元素在排序数组中的“范围”或“位置”。通过累计计数后得到的 count 数组，其每个元素表示的是：
         - count[i] 表示元素 i 在最终排序数组中的“最后一个位置”的索引。
         - count[i] - 1 就是元素 i 应该放置的第一个位置。
      - 为什么累计后的值是“索引位置”？
        - 累计计数数组通过加和的方式，把元素的 `个数信息` 转化为 `元素在排序后数组中的位置`。简单来说，累计计数帮助我们了解在排序过程中，元素 i 应该被放置在哪个“位置”上，以确保排序的正确性。
      - 累计数组如何转化为索引位置？
        - 初始的 count 数组：表示每个元素出现的次数。
          假设我们有一个数组 [4, 2, 2, 8, 3, 3, 1]，初始统计的 count 数组如下：
          - count = [0, 1, 2, 2, 1, 0, 0, 0, 1]
          这里 count[i] 表示值 i 在输入数组中出现的次数。例如，count[2] = 2 表示 2 出现了两次
        - 累计计数：通过将 count[i] 加上 count[i-1]，得到 `每个元素小于等于当前元素的总数`，进而得到该元素应该放置 `在排序数组中的最后位置`。
          累计后的 count 数组：
          - count = [0, 1, 3, 5, 6, 6, 6, 6, 7]
          现在:
          - count[1] = 1：表示 1 应该放置在排序数组的第一个位置。
          - count[2] = 3：表示 2 应该放置在排序数组中的索引位置 2 到 3（count[2] - 1 之前的位置）
          - count[3] = 5：表示 3 应该放置在排序数组中的索引位置 4 到 5
          因此，count[i] 存储的是 元素 i 在最终排序数组中的 “最后一个位置”，而 count[i] - 1 就是元素 i 的 “第一个位置”。

      为什么在反向遍历时需要使用 count[i] - 1？
        - 反向遍历：在反向遍历数组时，我们可以确保稳定排序（即相同的元素保持它们原来的相对顺序）。反向遍历的目的是尽量保证同一元素的多个实例按出现顺序排放，避免覆盖掉前面相同元素的排序位置。
        - count[i] - 1：由于 count[i] 表示该元素应放置的最后一个位置，所以我们用 count[i] - 1 来确定该元素在排序数组中的第一个位置。然后，每次放置完一个元素后，count[i] 会减少 1，确保下一个相同元素被放置在正确的位置。

      举例:
         假设我们有以下输入数组：
         - let arr = [4, 2, 2, 8, 3, 3, 1];
         1. 统计元素的出现次数： 初始 count 数组：
           - count = [0, 1, 2, 2, 1, 0, 0, 0, 1]
         2. 计算累计计数： 累计 count 数组：
           - count = [0, 1, 3, 5, 6, 6, 6, 6, 7]
         3. 反向遍历并放置元素： 反向遍历时，我们会根据 count[i] - 1 计算出每个元素应该放置的索引位置。每次放置一个元素后，更新 count[i]
           - 对于元素 1，count[1] = 1，所以它应该放在 result[0]，然后更新 count[1] = 0
           - 对于元素 2，count[2] = 3，所以它应该放在 result[2]，然后更新 count[2] = 2
           - 依此类推，最终得到排序后的数组：
             - result = [1, 2, 2, 3, 3, 4, 8]
       总结:
        累计计数数组中的每个元素表示该值的 `最后一个位置`。通过反向遍历，并将元素放置在 `count[i] - 1` 的位置，我们能够确保每个元素按照正确的顺序被放置到排序结果中。
*/

use std::fmt::{Debug, Display};

pub fn counting_sort<T>(arr: &mut [T])
where
    T: Ord + Debug + Clone + Copy + Into<isize> + Display,
{
    let n = arr.len();
    if n == 0 || n == 1 {
        return;
    }

    // 1. 初始化计数数组
    let max = *arr.iter().max().unwrap();
    let max: isize = max.into();
    let k = (max + 1) as usize;
    let mut count = vec![0; k];

    // 2. 统计输入数组元素出现次数( + 1)
    println!("统计输入数组元素出现次数:");
    for num in arr.iter() {
        let i_num: isize = (*num).into();
        count[i_num as usize] += 1;
        println!("{:?}", count)
    }

    println!();
    // 3. 计算累计计数数组
    println!("计算累计计数数组:");
    for i in 1..k {
        count[i] += count[i - 1];
        println!("{:?}", count);
    }

    // 4. 反向遍历输入数组，并将元素放到排序结果数组的正确位置
    println!();
    let mut results: Vec<T> = vec![arr[0].clone(); n];
    println!("init result: {:?}, count: {:?}", results, count);
    println!(
        "arr: {:?}, arr rev: {:?}",
        arr,
        arr.iter().rev().collect::<Vec<&T>>()
    );

    println!();
    println!("反向遍历输入数组，并将元素放到排序结果数组的正确位置:");
    for &num in arr.iter().rev() {
        let i_num: isize = num.into();
        let count_index_value = count[i_num as usize];
        let count_value = count_index_value - 1;
        let index = count_value;
        results[index] = num;
        count[i_num as usize] -= 1;
        println!(
            "num: {}, count[{}]: {}, count[{}] - 1: {}, result[{}]: {}, result: {:?}, count: {:?}",
            i_num,
            i_num,
            count_index_value,
            i_num,
            count_value,
            index,
            results[index],
            results,
            count
        )
    }

    // 5. 输出排序结果
    arr.copy_from_slice(&results); // 将一个切片（slice）的内容复制到另一个切片中
}
