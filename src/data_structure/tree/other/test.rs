use crate::data_structure::tree::other::huff::HuffmanTree;
use crate::data_structure::tree::other::merkle::account::Account;
use crate::data_structure::tree::other::merkle::client::Client;
use crate::data_structure::tree::other::merkle::processor::{Block, Processor};
use crate::data_structure::tree::other::segment::SegmentTree;
use crate::data_structure::tree::other::trie::Trie;
use std::collections::HashMap;

/// 测试 `霍夫曼树`
pub fn test_huffman_tree() {
    println!("----- Huffman Tree start ------");
    let mut frequency_map = HashMap::new();
    frequency_map.insert('A', 5);
    frequency_map.insert('B', 9);
    frequency_map.insert('C', 12);
    frequency_map.insert('D', 13);
    frequency_map.insert('E', 16);

    let mut tree = HuffmanTree::new(&frequency_map);

    println!("初始编码表: {:?}", tree.code_map);

    println!("\n🌳 初始树结构:");
    tree.print_tree();

    // 编码/解码
    let encoded_str = format!(
        "{}{}{}",
        tree.get_code('A').unwrap(),
        tree.get_code('B').unwrap(),
        tree.get_code('C').unwrap()
    );
    println!(
        "\n解码二进制串 \"{}\" ➜ {}",
        encoded_str,
        tree.decode(&encoded_str)
    );

    // 添加新符号
    tree.add_symbol(&mut frequency_map, 'F', 7);
    println!("\n添加 F 后编码表: {:?}", tree.code_map);
    tree.print_tree();

    // 修改 D 的频率
    tree.update_frequency(&mut frequency_map, 'D', 20);
    println!("\n修改 D 后编码表: {:?}", tree.code_map);
    tree.print_tree();

    // 删除 B
    tree.remove_symbol(&mut frequency_map, 'B');
    println!("\n删除 B 后编码表: {:?}", tree.code_map);
    tree.print_tree();

    // 带权路径
    let root = tree.get_tree_root();
    if let Some(root) = root {
        let wpl = tree.calculate_wpl(root, 0);
        println!("带权路径长度(WPL): {}", wpl);
    } else {
        println!("带权路径长度(WPL): {}", 0);
    }

    println!("----- Huffman Tree end ------");
}

/// 测试 `霍夫曼树(数据压缩)`
pub fn test_huffman_tree_compress() {
    println!("----- Huffman Tree Compress start ------");
    let json_str = r#"{
        "data": {},
        "localIp": "127.0.0.1",
        "version": "1.0",
        "appVersion": "1.0",
        "opStation": "NA",
        "appId": "AAA",
        "channel": "web",
        "loginAccount": "",
        "loginToken": "",
        "fundAccount": "",
        "fundToken": "",
        "requestId": "5322a66b-7307-f1ab-833a-006209f05592"
    }"#;

    // 1. 统计频率
    let mut frequency_map = HashMap::new();
    for c in json_str.chars() {
        *frequency_map.entry(c).or_insert(0) += 1;
    }

    // 2. 创建 Huffman Tree
    let tree = HuffmanTree::new(&frequency_map);

    // 3. 编码 JSON 字符串
    let mut encoded = String::new();
    for c in json_str.chars() {
        if let Some(code) = tree.get_code(c) {
            encoded.push_str(code);
        }
    }
    println!("编码结果:\n{}", encoded);

    // 4. 解码
    let decoded = tree.decode(&encoded);
    println!("\n解码后:\n{}", decoded);

    println!("----- Huffman Tree Compress end ------");
}

/// 测试 `字典树(敏感词匹配)`
pub fn test_trie() {
    println!("----- Trie Tree start ------");
    let sensitive_words = vec![
        "bad".to_string(),
        "damn".to_string(),
        "hell".to_string(),
        "测试".to_string(),
        "敏感".to_string(),
    ];

    // 原始文本
    let text =
        "This is a bad example. Oh damn, what the hell is going on? 这是一个测试文本，包含敏感词。";
    let replaced = Trie::replace_sensitive_words(&sensitive_words, text);
    println!("原始文本: {}", text);
    println!("替换结果: {}", replaced);
    println!("----- Trie Tree end ------");
}

/// 测试 `线段树`
fn test_segment() {
    println!("----- Segment Tree start ------");
    let nums = vec![2, 1, 5, 3, 4];

    let tree = SegmentTree::build(&nums, 0, nums.len() - 1);

    // 查询区间 `[1, 3]` 的和: 1 + 5 + 3 = 9
    let result = SegmentTree::query_range(&tree, 1, 3);
    println!("Query [1, 3]: {}", result);

    // 将 `[2, 4]` 区间上每个元素 `+2`: [5, 3, 4] + 2 = 7 + 5 + 6 = 18
    SegmentTree::update_range(&tree, 2, 4, 2);

    // 再次查询 `[1, 3]` 的和: 1 + (5+2) + (3+2) = 1 + 7 + 5 = 13
    let result = SegmentTree::query_range(&tree, 1, 3);
    println!("Query [1, 3] after update: {}", result);
    println!("----- Segment Tree End ------");
}

/// 测试 `Merkle 树`
fn test_merkle() {
    println!("----- Merkle Tree start ------");
    println!("=== 模拟链上交易提交系统 ===");
    // 创建账户
    let mut accounts: HashMap<String, Account> = HashMap::new();
    accounts.insert("Alice".into(), Account::new(100));
    accounts.insert("Bob".into(), Account::new(50));
    accounts.insert("Carol".into(), Account::new(30));
    accounts.insert("Dave".into(), Account::new(8));
    accounts.insert("Candy".into(), Account::new(25));
    accounts.insert("Jack".into(), Account::new(30));

    /*
    println!("Alice: {}", 100);
    println!("Bob: {}", 50);
    println!("Carol: {}", 30);
    println!("Dave: {}", 8);
     */

    let mut processor = Processor::new(accounts, Some(3));

    // 模拟发起交易(秒回机制)
    Client::send_transaction("Alice", "Blob", 10, 5, 50, 5, &mut processor);
    Client::send_transaction("Bob", "Carol", 5, 10, 100, 10, &mut processor);
    Client::send_transaction("Carol", "Dave", 2, 30, 300, 11, &mut processor);
    Client::send_transaction("Dave", "Candy", 30, 40, 400, 16, &mut processor);
    Client::send_transaction("Candy", "Jack", 21, 20, 150, 22, &mut processor);

    // 合约调用
    /*
     1. 用户 "Carol" 发起一个调用
     2. 调用的合约是 "MyContract"
     3. 合约内部调用的是函数 "get_value"，即：获取某个值
     4. vec![]：没有参数
    */
    // MyContract.get_value()
    Client::call_contract(
        "Carol",
        "MyContract",
        "get_value",
        vec![],
        15,
        100,
        10,
        &mut processor,
    );

    /*
     1. 用户 "Alice" 发起调用
     2. 合约 "MyContract" 的函数 "increment"（增量函数）
     3. 传入参数 "x"：比如想让变量 x 自增
     4. vec!["x".into()] 表示构造参数向量：字符串 "x" 被转换为 String 类型（用 .into()）
    */
    Client::call_contract(
        "Alice",
        "MyContract",
        "loop_forever",
        vec!["x".into()],
        25,
        250,
        15,
        &mut processor,
    );

    println!("\n=== 批量打包交易 ===");
    let mut blocks: Vec<Block> = Vec::new();
    while let Some(block) = processor.process() {
        println!(
            "== 区块构建完毕，Merkle 根为: {} ==\n",
            hex::encode(&block.root)
        );
        blocks.push(block);
    }

    let mut has_verified: bool = false;
    for block in blocks {
        // 判断某笔交易是否存在
        let verified = processor.verify(Some(block), 1);
        if verified {
            has_verified = true;
            break;
        }
    }

    println!(
        "第 2 笔交易验证结果：{}\n",
        if has_verified {
            "✅ 成功"
        } else {
            "❌ 失败"
        }
    );

    println!("=== 所有交易执行结果如下 ===");
    for (i, res) in processor.get_transaction_results().iter().enumerate() {
        println!(
            "第 {} 笔: [{}] {} (tx_hash: {})",
            i + 1,
            if res.success { "✅" } else { "❌" },
            res.message,
            hex::encode(&res.hash)
        );
    }

    // 查看余额
    println!("");
    processor.get_balances();

    println!("----- Merkle Tree End ------");
}

pub fn test() {
    println!("----- other tree start ------");
    test_huffman_tree();
    test_huffman_tree_compress();
    test_trie();
    test_segment();
    test_merkle();
    println!("----- other tree end ------");
}
