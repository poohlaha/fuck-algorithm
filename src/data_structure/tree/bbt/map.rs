/*!
 使用红黑树实现 Java 的 Tree Map
 方法:
 - put(key, value)	                  添加键值对, 如果键已存在, 则更新对应的值
 - fix_insert()	                      插入后修复红黑树性质（内部私有）
 - get(&key)	                      根据键获取对应的值
 - remove(&key)	                      删除指定键的键值对
 - fix_delete()	                      删除后修复红黑树性质（内部私有）
 - contains_key(&key)	              检查是否包含指定的键
 - contains_value(value)	          检查是否包含指定的值
 - size(value)	                      返回健值对数量
 - is_empty()	                      检查映射是否为空
 - first_key()	                      返回最小的键(按排序规则)
 - last_key()	                      返回最大的键(按排序规则)
 - head_map(&key)	                  返回小于指定键的子映射
 - tail_map(&to_key)	              返回大于或等于指定键的子映射
 - tail_map(&from_key)	              返回大于或等于指定键的子映射
 - sub_map(from_key, to_key)          返回指定范围内的子映射
 - key_set()                          返回所有键的集合(按排序规则)
 - values()                           返回所有值的集合
 - entry_set()                        返回所有键值对的集合(按排序规则)
 - clear()	                          清空所有值健对
 - floor_key(&key)	                  返回小于或等于指定键的最大键
 - ceiling_key(&key)	              返回大于等于指定键的最小键
 - higher_key(&key)	                  返回大于指定键的最小键
 - lower_key(&key)	                  返回小于指定键的最大键
 - poll_first_entry(&key)	          移除并返回最小键的键值对
 - poll_last_entry(&key)	          移除并返回最大键的键值对
 - len()	                          返回元素个数
 - min() 	                          返回最小的键值对
 - max() 	                          返回最小的键值对
 - iter()	                          中序遍历迭代器（返回有序键值对）
 - range(start, end)	              范围查询指定键在 [start, end) 内的值健对
 - len()	                          获取元素个数
 - clone()                            拷贝
*/

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

enum Color {
    Red,
    Black,
}

struct TreeNode<K, V> {
    key: K,
    value: V,
    color: Color,
    left: Option<Arc<Mutex<TreeNode<K, V>>>>,
    right: Option<Arc<Mutex<TreeNode<K, V>>>>,
    parent: Option<Arc<Mutex<TreeNode<K, V>>>>,
}

struct TreeMap<K, V> {
    root: Option<Arc<Mutex<TreeNode<K, V>>>>,
    size: usize,
    comparator: Option<Arc<dyn Fn(&K, &K) -> std::cmp::Ordering + Send + Sync>>,
}

impl<K: Ord, V> TreeMap<K, V> {
    // 创建一个空的 TreeMap，按照键的自然顺序进行排序
    pub fn new() -> Self {
        Self {
            root: None,
            size: 0,
            comparator: None,
        }
    }

    // 创建一个空的 TreeMap，并指定一个比较器来对键进行排序
    pub fn with_comparator<F>(comparator: F) -> Self
    where
        F: Fn(&K, &K) -> std::cmp::Ordering + Send + Sync + 'static,
    {
        Self {
            root: None,
            size: 0,
            comparator: Some(Arc::new(comparator)),
        }
    }

    // 创建一个 TreeMap，并将给定映射中的所有键值对添加到 TreeMap 中，并按照键的自然顺序进行排序。
    pub fn from_map(map: HashMap<K, V>) -> Self
    where
        K: Clone,
        V: Clone,
    {
        let mut tree = TreeMap::new();
        for (k, v) in map {
            tree.put(k, v);
        }

        tree
    }

    // 创建一个 TreeMap，并将给定有序映射中的所有键值对添加到 TreeMap 中，并按照给定有序映射中的比较器或者自然顺序进行排序
    pub fn from_sorted_map(map: &TreeMap<K, V>) -> Self {
        let mut new_map = if let Some(comparator) = &map.comparator {
            TreeMap {
                root: None,
                size: 0,
                comparator: Some(comparator.clone()),
            }
        } else {
            TreeMap::new()
        };

        /*
        for (k, v) in map.iter() {
            new_map.put(k, v);
        }
         */

        new_map
    }

    // 中序遍历迭代器（返回有序键值对）
    /*
    pub fn iter(&self) -> {

    }
     */

    // 添加键值对, 如果键已存在, 则更新对应的值
    pub fn put(&mut self, key: K, value: V) {}
}
