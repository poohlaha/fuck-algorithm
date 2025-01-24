/*!
 冒泡排序(Bubble Sort):
 1. 思想: 重复地遍历要排序的列表，比较相邻的元素并交换它们的位置，如果它们的顺序错误。这个过程会被重复进行，直到不需要更多的交换为止，也就是说列表已经被排序
 2. 时间复杂度:
    - 最优: O(n) (已排序)
    - 最差：O(n2)
    - 平均: O(n2)
 3. 空间复杂度: O(1)(原地排序)
 4. 特点: 简单易懂, 适合小规模数据; 效率较低
*/
pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    let mut n = arr.len();
    if n == 0 || n == 1 {
        return;
    }

    let mut swapped = false;
    let mut end = n - 1;

    loop {
        // 是否有交换, 如果没有交换，说明列表已经是有序的
        for i in 0..end {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }

        // 如果本次遍历没有交换，说明数组已经有序
        if !swapped {
            break;
        }

        // 每次遍历，最大的元素会到正确的位置
        end -= 1;
        swapped = false;
    }
}
