/*!
  选择排序(Selection Sort)
  1. 思想: 每次从未排序部分中选出最小（或最大）元素，放到已排序部分的末尾，数据量较小的场景
  2. 时间复杂度:
     - 最优: O(n2)
     - 最差：O(n2)
     - 平均: O(n2)
  3. 空间复杂度: O(1)
  4. 特点: 数据量小时简单有效；移动次数较少。

  步骤:
   1. 从待排序的数组中，选择最小的元素
   2. 将最小的元素与当前数组的第一个元素交换位置
   3. 将剩余未排序的元素重复上述步骤，直到数组排序完成
*/

pub fn selection_sort<T: Ord + Copy>(arr: &mut [T]) {
    let mut n = arr.len();
    if n == 0 || n == 1 {
        return;
    }

    // 当为 n-1 时, 数组已排序好, 不需要再进行比较
    for i in 0..n - 1 {
        let mut min_index = i;

        // 寻找未排序部分的最小元素
        for j in i + 1..n {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }

        // 如果找到的最小元素不在当前位置，交换它们
        if min_index != i {
            arr.swap(i, min_index);
        }
    }
}
