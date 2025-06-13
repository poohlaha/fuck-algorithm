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
}

impl Transaction {
    pub fn hash(&self) -> Vec<u8> {
        let tx_str = format!("{:?}", self);
        Sha256::digest(tx_str.as_bytes()).to_vec()
    }
}

pub struct Client;

impl Client {
    pub fn send_transaction(from: &str, to: &str, amount: i64, processor: &mut Processor) {
        // 模拟生成 tx_hash = SHA256(from + to + amount)
        // let raw = format!("{}:{}:{}", from, to, amount);
        // let hash = Sha256::digest(raw.as_bytes());

        let tx = Transaction {
            tx_type: TransactionType::Transfer {
                from: from.to_string(),
                to: to.to_string(),
                amount,
            },
        };

        // let tx_hash = hex::encode(tx.hash().clone());
        println!("[前端] 发起交易：{} -> {} ({} BTC)...", from, to, amount);

        // 秒回，获得 tx_hash
        processor.process_and_record(tx);
        // println!("[前端] 秒回成功，交易哈希：{}\n", tx_hash);
        println!("");
    }

    pub fn call_contract(
        caller: &str,
        contract: &str,
        method: &str,
        args: Vec<String>,
        processor: &mut Processor,
    ) {
        let tx = Transaction {
            tx_type: TransactionType::ContractCall {
                caller: caller.into(),
                contract: contract.into(),
                method: method.into(),
                args,
            },
        };
        processor.process_and_record(tx);
    }
}
