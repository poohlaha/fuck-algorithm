/*!
  Merkle 树, 支持构建、生成证明和验证
*/

use sha2::{Digest, Sha256};

#[derive(Clone)]
pub struct MerkleTree {
    // 多层节点，从底层（叶子）到顶层（根）
    // nodes[0] 是叶子层，nodes[n] 是根所在层
    nodes: Vec<Vec<Vec<u8>>>,
}

impl MerkleTree {
    // 构建 Merkle 树(从底层交易哈希开始), 传入的是交易哈希组成的叶子节点
    // 每个叶子是一个哈希值（Vec<u8>）
    pub fn new(leaves: &Vec<Vec<u8>>) -> Self {
        let mut levels = Vec::new(); // 所有层级，从叶子到根
        levels.push(leaves.clone()); // 第 0 层是叶子

        let mut leafs = leaves.clone();

        // 一直往上构建, 直到根节点
        while leafs.len() > 1 {
            // 如果当前层节点数为奇数，复制最后一个节点使之变偶数（比特币也这样做）
            if leafs.len() % 2 == 1 {
                leafs.push(leafs.last().unwrap().clone());
            }

            let mut next_level = Vec::new();
            // 每两个节点配对，生成父节点
            // 循环每次 `步长为 2`
            // 切片（slice）&[T] 里的内容，追加 到当前 Vec<T> 的尾部
            for i in (0..leafs.len()).step_by(2) {
                let mut combined = Vec::new();
                combined.extend_from_slice(&leafs[i]);
                combined.extend_from_slice(&leafs[i + 1]);

                // 哈希合并后的数据，作为父节点
                let hash = Sha256::digest(&combined);
                next_level.push(hash.to_vec());
            }

            levels.push(next_level.clone());
            leafs = next_level; // 当前层变成下一轮的底层
        }

        MerkleTree { nodes: levels }
    }

    // 获取 Merkle 树的根哈希
    pub fn root(&self) -> Vec<u8> {
        if let Some(ref last) = self.nodes.last() {
            return last[0].clone();
        }

        Vec::new()
    }

    // 为给定的叶子索引生成 Merkle 证明（路径）
    // 返回一组 (兄弟节点哈希, 该兄弟节点是否在右侧)
    pub fn gen_proof(&self, i: usize) -> Vec<(Vec<u8>, bool)> {
        let mut index = i;
        let mut proof = Vec::new();

        // 遍历每一层, 直到只剩下根节点
        for level in &self.nodes {
            if level.len() == 1 {
                break;
            }

            let is_right = index % 2 == 1;
            let sibling_index = if is_right { index - 1 } else { index + 1 };

            // 找到兄弟节点（防止越界时取最后一个）
            let sibling = level
                .get(sibling_index)
                .unwrap_or(level.last().unwrap())
                .clone();

            // 加入证明路径
            proof.push((sibling, is_right));

            // 下一层的索引是当前层除以 2
            index /= 2;
        }

        proof
    }

    // 验证某个哈希值是否存在于 Merkle 树中
    // leaf: 被验证的叶子哈希
    // root: Merkle 根哈希
    pub fn verify(tree: MerkleTree, leaf: Vec<u8>, i: usize) -> bool {
        let mut hash = leaf;
        let proof = tree.gen_proof(i);

        //  逐层组合路径进行哈希
        for (sibling, is_right) in proof {
            let mut combined = Vec::new();
            if is_right {
                // 如果兄弟在右边，顺序是 [sibling][hash]
                combined.extend_from_slice(&sibling);
                combined.extend_from_slice(&hash);
            } else {
                // 如果兄弟在左边，顺序是 [hash][sibling]
                combined.extend_from_slice(&hash);
                combined.extend_from_slice(&sibling);
            }

            // 计算 hash
            let hash_digest = Sha256::digest(&combined);
            // let hash_str = hex::encode(&hash);
            // println!("第 {} 笔交易 hash: {}", i + 1, hash_str);
            hash = hash_digest.to_vec(); // 更新 hash 为上一层
        }

        hash == tree.root()
    }
}
