/*!
  顺序存储
  - 定义
    - 根节点的索引为 `0`
    - 对于任意节点 `i`, 其左子节点的索引是 `2 * i + 1`, 右子节点的索引是 `2* i + 2`
    - 给定节点 `i`, 其父节点的索引是 `(i - 1) / 2`

  - 举例
         1
        / \
       2   3
      / \   \
     4   5   6
   顺序存储方式在数组中表示为: [1, 2, 3, 4, 5, 6]
*/

use std::fmt::{Debug, Display};

// 二叉搜索树
// 左子树的所有节点值都小于根节点的值
// 右子树的所有节点值都大于根节点的值
#[derive(Debug, Clone, PartialEq)]
pub struct TreeNode<T> {
    data: Vec<T>,
}

impl<T: Clone + Display + Debug + PartialOrd + Default> TreeNode<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    // 插入
    pub fn insert(&mut self, value: T) {
        if self.data.is_empty() {
            self.data.push(value);
            return;
        }
        self.insert_recursive(0, value);
    }

    // 调整树的顺序, 保证完全二叉树的性质
    fn insert_recursive(&mut self, index: usize, value: T) {
        // 获取当前节点的值
        if index >= self.data.len() {
            return;
        }

        // 计算左右子节点的索引
        let left_index = 2 * index + 1;
        let right_index = 2 * index + 2;

        if index > 0 && (left_index >= self.data.len() || right_index >= self.data.len()) {
            self.data.insert(index + 1, value.clone());
            return;
        }

        // 如果新值小于当前节点的值，插入到左子树
        if value < self.data[index] {
            // 如果左子树位置空，则插入
            if left_index < self.data.len() {
                self.insert_recursive(left_index, value);
            } else {
                self.data.push(value);
            }
        }
        // 如果新值大于等于当前节点的值，插入到右子树
        else {
            // 如果右子树位置空，则插入
            if right_index < self.data.len() {
                self.insert_recursive(right_index, value);
            } else {
                self.data.push(value);
            }
        }
    }

    // 查找
    pub fn search(&self, value: T) -> Option<usize> {
        self.search_recursive(0, value)
    }

    fn search_recursive(&self, index: usize, value: T) -> Option<usize> {
        if index >= self.data.len() {
            return None;
        }

        if self.data[index] == value {
            return Some(index);
        }

        if value < self.data[index] {
            // 从左子树搜索
            // 对于任意节点 `i`, 其左子节点的索引是 `2 * i + 1`
            let left_index = 2 * index + 1;
            self.search_recursive(left_index, value)
        } else {
            // 从右子树搜索
            // 对于任意节点 `i`, 其右子节点的索引是 `2* i + 2`
            let right_index = 2 * index + 2;
            self.search_recursive(right_index, value)
        }
    }

    // 删除
    pub fn remove(&mut self, value: T) -> bool {
        if let Some(index) = self.search(value) {
            self.remove_recursive(index);
            return true;
        }

        false
    }

    fn remove_recursive(&mut self, index: usize) {
        // 对于任意节点 `i`, 其左子节点的索引是 `2 * i + 1`
        let left_index = 2 * index + 1;
        // 对于任意节点 `i`, 其右子节点的索引是 `2* i + 2`
        let right_index = 2 * index + 2;

        // 从左子树上删除
        if left_index < self.data.len() {
            self.data.remove(left_index);
            return;
        }

        // 从右子树上删除
        self.data.remove(right_index);
    }

    // 深度
    pub fn depth(&self) -> usize {
        self.calculate_depth(0)
    }

    fn calculate_depth(&self, index: usize) -> usize {
        // 当前节点不存在, 深度为 0
        if index >= self.data.len() {
            return 0;
        }

        // 计算左子树的深度
        // 对于任意节点 `i`, 其左子节点的索引是 `2 * i + 1`
        let left_index = 2 * index + 1;
        let left_depth = self.calculate_depth(left_index);

        // 计算右子树的深度
        // 对于任意节点 `i`, 其右子节点的索引是 `2* i + 2`
        let right_index = 2 * index + 2;
        let right_depth = self.calculate_depth(right_index);

        // 当前节点深度为左右子树最大深度 + 1
        std::cmp::max(left_depth, right_depth) + 1
    }

    pub fn print(&mut self) {
        println!("{:?}", self.data)
    }

    // 打印树
    pub fn print_tree(&mut self) {
        let spec: Vec<String> = Vec::new();
        let value: Vec<String> = Vec::new();

        let mut level = 0;
        let mut node_in_level = 1;
        for (index, v) in self.data.iter().enumerate() {
            if index >= node_in_level {
                level += 1;
                node_in_level = 2_usize.pow(level as u32);
                println!();
            }

            print!("{:4} ", v);
            /*
            println!("{} {}", index, v);
            // let indent = " ".repeat((max_depth - index) * 4);
            let indent = "";
            if index == 0 {
                println!("{}{}", indent, v);
                continue;
            }

            // 对于任意节点 `i`, 其左子节点的索引是 `2 * i + 1`
            let left_index = 2 * index + 1;
            // 对于任意节点 `i`, 其右子节点的索引是 `2* i + 2`
            let right_index = 2 * index + 2;
            if left_index < self.data.len() {
                spec.push(format!("{}/", indent));
                value.push(format!("{}{}", indent, self.data[left_index].clone()));
            }

            if right_index < self.data.len() {
                spec.push(format!("{}\\", indent));
                value.push(format!("{}{}", indent, self.data[right_index].clone()));
            }

            println!("{:?}", spec);
            println!("{:?}", value);
             */
        }

        if !spec.is_empty() {
            println!("{}", spec.join(""));
        }

        if !value.is_empty() {
            println!("{}", value.join(""));
        }
    }
}
