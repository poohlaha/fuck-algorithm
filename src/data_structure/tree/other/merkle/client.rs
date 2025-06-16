/*!
  发起交易
*/

use crate::data_structure::tree::other::merkle::account::TransactionType;
use crate::data_structure::tree::other::merkle::processor::Processor;
use sha2::{Digest, Sha256};
use std::cmp::Ordering;

// 交易结构体
#[derive(Clone, Debug)]
pub struct Transaction {
    pub tx_type: TransactionType,
    pub hash: String,
    pub tx_hash: Vec<u8>,
    pub gas: u64,
    pub gas_limit: u64, // 交易最多可以用多少 gas
    pub priority: u8,   // 优先级, 越大优先级越高
}

// 用于优先级队列排序(大顶堆)
impl PartialEq for Transaction {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority && self.gas == other.gas
    }
}

impl Eq for Transaction {}

// 部分排序(可能不可比)
impl PartialOrd for Transaction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other)) // 复用 Ord 的逻辑
    }
}

// 全排序(必须能比较)
impl Ord for Transaction {
    // .cmp() 是升序(小的排前面), 用 other.cmp(self) 来反过来实现降序
    // other.priority.cmp(&self.priority) <--> self.priority.cmp(&other.priority).reverse()
    fn cmp(&self, other: &Self) -> Ordering {
        // 先比较 priority，再比较 gas（降序）
        println!(
            "Comparing self: priority={}, gas={} with other: priority={}, gas={}",
            self.priority, self.gas, other.priority, other.gas
        );

        let ord = self
            .priority
            .cmp(&other.priority)
            .then_with(|| self.gas.cmp(&other.gas));

        println!("cmp result: {:?}", ord);
        ord
    }
}

pub struct Client;

impl Client {
    pub fn send_transaction(
        from: &str,
        to: &str,
        amount: i64,
        gas: u64,
        gas_limit: u64,
        priority: u8,
        processor: &mut Processor,
    ) -> String {
        // 模拟生成 tx_hash = SHA256(from + to + amount)
        let raw = format!("{}:{}:{}:{}:{}", from, to, amount, gas, priority);
        let hash = Sha256::digest(raw.as_bytes());

        let tx = Transaction {
            tx_type: TransactionType::Transfer {
                from: from.to_string(),
                to: to.to_string(),
                amount,
            },
            hash: hex::encode(&hash),
            tx_hash: hash.to_vec(),
            gas,
            gas_limit,
            priority,
        };

        println!(
            "[前端] 发起交易：{} -> {} ({} BTC, gas: {}, priority: {})，返回哈希: {}",
            from, to, amount, gas, priority, tx.hash
        );

        let hash = tx.hash.clone();
        // 秒回，获得 tx_hash
        processor.enqueue_transaction(tx);
        hash
    }

    pub fn call_contract(
        caller: &str,
        contract: &str,
        method: &str,
        args: Vec<String>,
        gas: u64,
        gas_limit: u64,
        priority: u8,
        processor: &mut Processor,
    ) -> String {
        // 模拟生成 tx_hash = SHA256(from + to + amount)
        let raw = format!(
            "{}:{}:{}:{:?}:{}:{}",
            caller, contract, method, args, gas, priority
        );
        let hash = Sha256::digest(raw.as_bytes());

        let tx = Transaction {
            tx_type: TransactionType::ContractCall {
                caller: caller.into(),
                contract: contract.into(),
                method: method.into(),
                args,
            },
            hash: hex::encode(&hash),
            tx_hash: hash.to_vec(),
            gas,
            gas_limit,
            priority,
        };

        println!(
            "[前端] 发起合约调用：{}.{}() gas={} priority={}，返回哈希: {}",
            contract, method, gas, priority, tx.hash
        );

        let hash = tx.hash.clone();
        processor.enqueue_transaction(tx);
        hash
    }
}
