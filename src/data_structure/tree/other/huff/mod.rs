/*!
霍夫曼树(Huffman Tree)
*/

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::sync::Arc;

pub struct Node {
    frequency: usize,         // 节点频率
    symbol: Option<char>,     // 符号: 叶子节点有,非叶子节点为 None
    left: Option<Arc<Node>>,  // 左子树
    right: Option<Arc<Node>>, // 右子树
}

impl Node {
    // 创建叶子节点
    fn new_leaf(symbol: char, frequency: usize) -> Arc<Node> {
        Arc::new(Node {
            frequency,
            symbol: Some(symbol),
            left: None,
            right: None,
        })
    }

    // 创建内部节点
    fn new_internal(frequency: usize, left: Arc<Node>, right: Arc<Node>) -> Arc<Node> {
        Arc::new(Node {
            frequency,
            symbol: None,
            left: Some(left),
            right: Some(right),
        })
    }
}

// 为 BinaryHeap（最小堆）提供比较器
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.frequency == other.frequency
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.frequency.cmp(&self.frequency)) // 反向（最小堆）
    }
}

// BinaryHeap 默认是最大堆
// 但我们用 Ordering::reverse(在 Ord 里 other.frequency.cmp(&self.frequency)，让它当最小堆用
// 当 heap.push(...) / heap.pop() / heap.peek() 被调用时，BinaryHeap 需要对 Node 进行比较, 这些比较用的就是 Ord / PartialOrd
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // 让小的排在前面
        other.frequency.cmp(&self.frequency)
    }
}

pub struct HuffmanTree {
    root: Option<Arc<Node>>,                    // 根节点
    pub(crate) code_map: HashMap<char, String>, // 符号编码表
}

impl HuffmanTree {
    // 通过频率表构造霍夫曼树
    pub(crate) fn new(frequency_map: &HashMap<char, usize>) -> Self {
        let mut heap = BinaryHeap::new();

        // 每一个符号一个叶子节点
        for (&symbol, &frequency) in frequency_map.iter() {
            heap.push(Node::new_leaf(symbol, frequency));
        }

        // 合并最小的两个节点，直到只剩下根
        while heap.len() > 1 {
            let left = heap.pop().unwrap();
            let right = heap.pop().unwrap();
            let parent = Node::new_internal(left.frequency + right.frequency, left, right);
            heap.push(parent);
        }

        let root = heap.pop();

        // 构建编码表
        let mut code_map = HashMap::new();
        if let Some(ref r) = root {
            HuffmanTree::build_code_map(r, String::new(), &mut code_map);
        }

        HuffmanTree { root, code_map }
    }

    // 递归生成编码表
    fn build_code_map(node: &Arc<Node>, path: String, code_map: &mut HashMap<char, String>) {
        if let Some(symbol) = node.symbol {
            code_map.insert(symbol, path);
        } else {
            if let Some(ref left) = node.left {
                HuffmanTree::build_code_map(&left, format!("{}0", path), code_map);
            }

            if let Some(ref right) = node.right {
                HuffmanTree::build_code_map(&right, format!("{}1", path), code_map);
            }
        }
    }

    // 获取符号编码
    pub(crate) fn get_code(&self, symbol: char) -> Option<&String> {
        self.code_map.get(&symbol)
    }

    // 添加新符号（更新频率表 + 重建树）
    pub(crate) fn add_symbol(
        &mut self,
        frequency_map: &mut HashMap<char, usize>,
        symbol: char,
        frequency: usize,
    ) {
        frequency_map.insert(symbol, frequency);
        *self = HuffmanTree::new(frequency_map);
    }

    // 修改频率（更新频率表 + 重建树）
    pub(crate) fn update_frequency(
        &mut self,
        frequency_map: &mut HashMap<char, usize>,
        symbol: char,
        new_freq: usize,
    ) {
        if frequency_map.contains_key(&symbol) {
            frequency_map.insert(symbol, new_freq);
            *self = HuffmanTree::new(frequency_map);
        }
    }

    // 删除符号（更新频率表 + 重建树）
    pub(crate) fn remove_symbol(&mut self, frequency_map: &mut HashMap<char, usize>, symbol: char) {
        if frequency_map.remove(&symbol).is_some() {
            *self = HuffmanTree::new(frequency_map);
        }
    }

    // 解码二进制 -> 符号串
    pub(crate) fn decode(&self, encoded: &str) -> String {
        let mut result = String::new();
        let mut current = self.root.clone();
        for bit in encoded.chars() {
            if let Some(ref node) = current {
                current = if bit == '0' {
                    node.left.clone()
                } else {
                    node.right.clone()
                };

                if let Some(ref n) = current {
                    if let Some(symbol) = n.symbol {
                        result.push(symbol);
                        current = self.root.clone();
                    }
                }
            }
        }

        result
    }

    // 可视化层序打印树结构
    pub(crate) fn print_tree(&self) {
        if let Some(ref root) = self.root {
            let mut queue = VecDeque::new();
            queue.push_back((root.clone(), 0));

            let mut current_level = 0;
            print!("Level {}: ", current_level);
            while let Some((node, level)) = queue.pop_front() {
                if level != current_level {
                    current_level = level;
                    print!("\nLevel {}: ", current_level);
                }

                if let Some(symbol) = node.symbol {
                    print!("('{}':{}) ", symbol, node.frequency);
                } else {
                    print!("({}) ", node.frequency);
                }

                if let Some(ref left) = node.left {
                    queue.push_back((left.clone(), level + 1));
                }
                if let Some(ref right) = node.right {
                    queue.push_back((right.clone(), level + 1));
                }
            }

            println!();
        } else {
            println!("Tree is empty !");
        }
    }
}
