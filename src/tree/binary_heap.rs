/**!
    二叉堆, 一种特殊的二叉树（完全二叉树）, 存储在数组里
    最大堆和最小堆：
        - 最大堆的性质是：每个节点都大于等于它的两个子节点
        - 最小堆的性质是：每个节点都小于等于它的子节点
    上浮和下沉:
        - 上浮
            - 上浮操作通常在插入新元素到堆中后执行，或者在堆中某个元素的值发生变化并可能破坏堆的性质时执行。
            - 上浮的过程是将新插入或变化的元素逐级向上移动，直到它的父节点比它的值小（最小堆）或大（最大堆），或者到达堆的根节点位置。
            - 上浮操作的目标是恢复堆的性质，使得插入的元素被正确地放置在堆中的适当位置
        - 下沉
            - 下沉操作通常在删除根节点后执行，或者在堆中某个元素的值发生变化并可能破坏堆的性质时执行。
            - 下沉的过程是将根节点（或者其他变化的节点）逐级向下移动，直到其子节点的值大于（最小堆）或小于（最大堆）它的值，或者到达堆的叶子节点位置。
            - 下沉操作的目标是恢复堆的性质，使得堆仍然是一个有效的堆。
            - 如果删除的是最大堆，删除并返回的就是最大元素；如果删除的是最小堆，删除并返回的就是最小元素。
*/

pub trait BinaryHeap<R> {
    /**
        在数组表示的完全二叉树中，每个父节点的左子节点总是在其索引的下一个位置。
        例如，父节点的索引为 i，则左子节点的索引为 i + 1。
        而在计算机编程中，数组的索引从 0 开始，
        因此左子节点的索引实际上是父节点索引乘以 2 加上 1，即 2*i + 1。
    **/
    fn left(&self, i: usize) -> usize {
        return 2 * i + 1;
    }

    /**
        与左子节点类似，每个父节点的右子节点总是在其索引的下两个位置。
        例如，父节点的索引为 i，则右子节点的索引为 i + 2。
        同样地，为了得到正确的索引，我们需要将父节点的索引乘以 2，然后再加上 2，即 2*i + 2。
    **/
    fn right(&self, i: usize) -> usize {
        return 2 * i + 2;
    }

    /**
        如果我们有一个子节点的索引 j，我们希望找到它的父节点的索引。
        观察一下，可以发现这样的关系：
            如果将子节点的索引除以 2（整数除法），然后向下取整，就可以得到父节点的索引。
            这是因为在完全二叉树中，每个父节点的左子节点和右子节点都是连续的，索引之间相差为 1 或 2，
            因此反推回去时需要除以 2。
    **/
    fn parent(&self, i: usize) -> usize {
        if i == 0 {
            return 0;
        }

        return (i - 1) / 2;
    }

    /// 插入
    fn push(&mut self, value: R);

    /// 删除, 如果删除的是最大堆，删除并返回的就是最大元素；如果删除的是最小堆，删除并返回的就是最小元素。
    fn delete(&mut self) -> Option<R>;

    /// 下沉, 删除用
    fn sink(&mut self, index: usize);

    /// 上浮, 添加用
    fn swim(&mut self, index: usize);
}

/// 最小堆
#[derive(Debug, Clone)]
pub struct BinaryMinHeap<T> {
    data: Vec<T>,
}

impl<T: PartialOrd> BinaryHeap<T> for BinaryMinHeap<T> {
    /// 插入, 上浮
    fn push(&mut self, value: T) {
        self.data.push(value);
        let last_index = self.data.len() - 1;
        self.swim(last_index);
    }

    /// 删除, 下沉, 返回最小元素
    fn delete(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }

        // 交换第一个和最后一个元素
        let last_index = self.data.len() - 1;
        self.data.swap(0, last_index);

        // 删除交换后的第一个元素
        let result = self.data.pop();

        // 下沉
        self.sink(0);
        result
    }

    /// 下沉, 删除用
    fn sink(&mut self, mut index: usize) {
        let size = self.data.len();
        while index < size {
            let left_child_index = self.left(index);
            let right_child_index = self.right(index);
            let mut min_index = index;

            if left_child_index < size && self.data[left_child_index] < self.data[min_index] {
                min_index = left_child_index
            }

            if right_child_index < size && self.data[right_child_index] < self.data[min_index] {
                min_index = right_child_index
            }

            if min_index == index {
                break;
            }

            self.data.swap(min_index, index);
            index = min_index;
        }
    }

    /// 上浮, 添加用
    fn swim(&mut self, mut index: usize) {
        while index > 0 && index <= self.data.len() - 1 {
            let parent_index = self.parent(index);
            if self.data[parent_index] == self.data[index] {
                // 最小堆中父节点的值小于或等于子节点的值
                break;
            }

            if self.data[parent_index] < self.data[index] {
                // 最小堆中父节点的值小于或等于子节点的值
                break;
            }

            self.data.swap(parent_index, index);
            index = parent_index
        }
    }
}

impl<T: PartialOrd> BinaryMinHeap<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn get_data(&self) -> &Vec<T> {
        return &self.data;
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

/// 最大堆
#[derive(Debug, Clone)]
pub struct BinaryMaxHeap<T> {
    data: Vec<T>,
}

impl<T: PartialOrd> BinaryHeap<T> for BinaryMaxHeap<T> {
    /// 插入, 上浮
    fn push(&mut self, value: T) {
        self.data.push(value);
        let last_index = self.data.len() - 1;
        self.swim(last_index);
    }

    /// 删除, 下沉, 返回最大元素
    fn delete(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }

        // 交换第一个和最后一个元素
        let last_index = self.data.len() - 1;
        self.data.swap(0, last_index);

        // 删除交换后的第一个元素
        let result = self.data.pop();
        self.sink(0);
        result
    }

    /// 下沉, 删除用
    fn sink(&mut self, mut index: usize) {
        let size = self.data.len();
        while index < size {
            let left_child_index = self.left(index);
            let right_child_index = self.right(index);
            let mut max_index = index;

            if left_child_index < size && self.data[left_child_index] > self.data[max_index] {
                max_index = left_child_index
            }

            if right_child_index < size && self.data[right_child_index] > self.data[max_index] {
                max_index = right_child_index
            }

            if max_index == index {
                break;
            }

            self.data.swap(max_index, index);
            index = max_index;
        }
    }

    /// 上浮, 添加用
    fn swim(&mut self, mut index: usize) {
        while index > 0 && index <= self.data.len() - 1 {
            let parent_index = self.parent(index);
            if self.data[parent_index] >= self.data[index] {
                // 最大堆中父节点的值小于或等于子节点的值
                break;
            }

            self.data.swap(parent_index, index);
            index = parent_index
        }
    }
}

impl<T: PartialOrd> BinaryMaxHeap<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn get_data(&self) -> &Vec<T> {
        return &self.data;
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
