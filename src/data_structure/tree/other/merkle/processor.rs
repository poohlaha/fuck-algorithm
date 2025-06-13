/*!
  接收交易 + 模拟秒回 + 存储
*/

use crate::data_structure::tree::other::merkle::account::{
    Account, ContractResult, TransactionType,
};
use crate::data_structure::tree::other::merkle::client::Transaction;
use std::collections::{HashMap, VecDeque};

#[derive(Default, Clone)]
pub struct TransactionResult {
    pub success: bool,
    pub message: String,
    pub hash: Vec<u8>,
}

pub struct Processor {
    accounts: HashMap<String, Account>,
    mempool: VecDeque<Transaction>,
    results: Vec<TransactionResult>,
    tx_hashes: Vec<Vec<u8>>,
}

impl Processor {
    pub fn new(accounts: HashMap<String, Account>) -> Self {
        Self {
            accounts,
            mempool: VecDeque::new(),
            results: Vec::new(),
            tx_hashes: vec![],
        }
    }

    fn apply_transaction(&mut self, tx: Transaction) -> Result<ContractResult, String> {
        match &tx.tx_type {
            TransactionType::Transfer { from, to, amount } => {
                let from_balance = self.accounts.get_mut(from).ok_or("Sender not found")?;
                if from_balance.balance < *amount {
                    return Err("Insufficient balance".into());
                }
                from_balance.balance -= amount;

                let to_balance = self.accounts.entry(to.clone()).or_insert(Account::new(0));
                to_balance.balance += amount;
                Ok(ContractResult::Ok)
            }
            TransactionType::ContractCall {
                caller,
                contract,
                method,
                args,
            } => {
                println!(
                    "模拟调用合约: {} 调用 {}.{}({:?})",
                    caller, contract, method, args
                );
                // 模拟合约返回值
                if method == "get_value" {
                    Ok(ContractResult::ReturnValue("42".to_string()))
                } else {
                    Ok(ContractResult::Ok)
                }
            }
        }
    }

    pub fn process_and_record(&mut self, tx: Transaction) {
        match self.apply_transaction(tx.clone()) {
            Ok(result) => {
                let hash = tx.hash();
                self.mempool.push_back(tx);
                self.tx_hashes.push(hash.clone());
                println!("[链上处理] 交易成功，哈希值: {}", hex::encode(hash));
                if let ContractResult::ReturnValue(val) = result {
                    println!("合约返回值: {}", val);
                }
            }
            Err(err) => {
                println!("[链上处理] 交易失败: {}", err);
            }
        }
    }

    pub fn get_transaction_hashes(&self) -> Vec<Vec<u8>> {
        self.tx_hashes.clone()
    }

    pub fn get_balances(&self) {
        println!("=== 当前账户余额 ===");
        for (addr, acc) in &self.accounts {
            println!("{}: {} BTC", addr, acc.balance);
        }
    }

    pub fn get_transaction_results(&self) -> Vec<TransactionResult> {
        self.results.clone()
    }
}
