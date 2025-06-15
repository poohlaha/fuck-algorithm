/*!
  发起交易
*/

use crate::data_structure::tree::other::merkle::account::TransactionType;
use crate::data_structure::tree::other::merkle::processor::Processor;
use sha2::{Digest, Sha256};
use std::hash::Hash;

// 交易结构体
#[derive(Clone, Debug)]
pub struct Transaction {
    pub tx_type: TransactionType,
    pub hash: String,
    pub tx_hash: Vec<u8>,
}

pub struct Client;

impl Client {
    pub fn send_transaction(
        from: &str,
        to: &str,
        amount: i64,
        processor: &mut Processor,
    ) -> String {
        // 模拟生成 tx_hash = SHA256(from + to + amount)
        let raw = format!("{}:{}:{}", from, to, amount);
        let hash = Sha256::digest(raw.as_bytes());

        let tx = Transaction {
            tx_type: TransactionType::Transfer {
                from: from.to_string(),
                to: to.to_string(),
                amount,
            },
            hash: hex::encode(&hash),
            tx_hash: hash.to_vec(),
        };

        println!(
            "[前端] 发起交易：{} -> {} ({} BTC)... 返回交易哈希: {}",
            from, to, amount, tx.hash
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
        processor: &mut Processor,
    ) -> String {
        // 模拟生成 tx_hash = SHA256(from + to + amount)
        let raw = format!("{}:{}:{}:{:?}", caller, contract, method, args);
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
        };

        println!(
            "[前端] 发起合约调用：{}.{}()... 返回交易哈希: {}",
            contract, method, tx.hash
        );

        let hash = tx.hash.clone();
        processor.enqueue_transaction(tx);
        hash
    }
}
