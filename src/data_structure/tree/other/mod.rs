/*!
    树类型       特点 & 应用
    二叉树	    基础树结构
    多叉树	    目录、JSON、组织结构
    B 树	    数据库索引（MySQL）
    B+ 树	    文件系统、数据库索引
    Trie 树	    搜索引擎、前缀匹配
    红黑树	    std::map，Linux epoll
    AVL 树	    快速查找但插入慢
    并查集	    连通性检测（社交网络）
    线段树	    区间统计查询
    BIT 树	    前缀和计算
    哈夫曼树	    文件压缩
    K-D 树	    2D / 3D 近邻搜索
    Merkle 树	区块链验证
    四叉树	    2D 空间划分
    Splay 树	LRU 缓存
*/
mod bit;
mod dsu;
pub mod huff;
mod kd;
mod merkle;
mod more;
pub mod segment;
pub mod trie;

pub mod test;
