/*!
    基数排序(Radix Sort)
    非比较型整数排序算法，通过将数据按位(digit)分解并进行多次排序，最终完成排序的过程。
    它适用于整数和字符串排序，尤其在数据位数较少时效率更高。
    1. 思想: 从最低位到最高位依次对数据进行排序
    2. 时间复杂度:
       - 假设输入数据的数量为 n, 最大数的位数为 d, 每次位排序的复杂度为 O(n + k)(k 是每个位的基数, 通常为常数), 总复杂度为 O(d * (n + k))
       - 当基数 𝑘 是常数时，时间复杂度近似为 O(d * n)
    3. 空间复杂度: 辅助数组的大小为 O(n + k), 因此空间复杂度为 O(n + k)
    4. 特点:
       非比较排序；适合整数排序
       - 优点
         - 稳定排序：相同值的相对顺序不变。
         - 对某些分布范围有限的数据（如整数）表现优异。
         - 避免了比较操作，适合排序大量数据。
       - 缺点
         - 需要额外的内存空间。
         - 只能用于整数或能映射为整数的数据（如字符串）。

    步骤:
    1. 按照数值的 `每一位` 进行排序（从低位到高位，或从高位到低位）
    2. 对每一位的排序通常使用**计数排序（Counting Sort）**或稳定排序算法来保证排序的稳定性

    举例:
     假设有一组整数，使用十进制为例: [170, 45, 75, 90, 802, 24, 2, 66]
     1. 找到最大数，确定排序的轮数：
        - 最大数为 802，它有 3 位数，因此需要排序 3 次（按个位、十位和百位依次排序）
     2. 按个位数排序：
       - 提取每个数的个位：
        170 -> 0
        45 -> 5
        75 -> 5
        90 -> 0
        802 -> 2
        24 -> 4
        2 -> 2
        66 -> 6
       - 排序后：[170, 90, 802, 2, 24, 45, 75, 66]
     3. 按十位数排序
       - 提取每个数的十位
        170 -> 7
        90 -> 9
        802 -> 0
        2 -> 0
        24 -> 2
        45 -> 4
        75 -> 7
        66 -> 6
       - 排序后：[802, 2, 24, 45, 66, 170, 75, 90]
      4. 按百位数排序
       - 提取每个数的百位
        802 -> 8
        2 -> 0
        24 -> 0
        45 -> 0
        66 -> 0
        170 -> 1
        75 -> 0
        90 -> 0
       - 排序后：[2, 24, 45, 66, 75, 90, 170, 802]
      最终结果：[2, 24, 45, 66, 75, 90, 170, 802]
*/
pub fn radix_sort(arr: &mut [u32]) {
    if arr.len() <= 1 {
        return;
    }

    // 1. 找到最大数
    let max = *arr.iter().max().unwrap();
    let mut exp = 1; // 指数，表示当前位：1 -> 个位, 10 -> 十位，依次类推

    while max / exp > 0 {
        counting_sort(arr, exp);
        exp *= 10; // 进入下一位
    }
}

/// 计数统计
fn counting_sort(arr: &mut [u32], exp: u32) {
    // 1. 初始化计数数组
    let mut results = vec![0; arr.len()];
    let mut count = vec![0; 10]; // 每个位的数字范围是 0~9

    // 2. 统计输入数组元素出现次数( + 1)
    for &num in arr.iter() {
        let digit = (num / exp % 10) as usize;
        count[digit] += 1;
    }

    // 3. 计算累计计数数组
    for i in 1..10 {
        count[i] += count[i - 1];
    }

    // 4. 反向遍历输入数组，并将元素放到排序结果数组的正确位置
    for &num in arr.iter().rev() {
        let digit = (num / exp % 10) as usize;
        let index = count[digit] - 1;
        results[index] = num;
        count[digit] -= 1;
    }

    arr.copy_from_slice(&results);
}
