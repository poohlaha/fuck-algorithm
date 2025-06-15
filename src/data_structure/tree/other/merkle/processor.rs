/*!
  接收交易 + 模拟秒回 + 存储
*/

use crate::data_structure::tree::other::merkle::account::{
    Account, ContractResult, TransactionType,
};
use crate::data_structure::tree::other::merkle::client::Transaction;
use crate::data_structure::tree::other::merkle::merkle::MerkleTree;
use std::collections::{HashMap, VecDeque};

#[derive(Default, Clone)]
pub struct TransactionResult {
    pub success: bool,
    pub message: String,
    pub hash: Vec<u8>,
}

pub struct Block {
    pub transactions: Vec<Transaction>,
    pub root: Vec<u8>,
    pub merkle: MerkleTree,
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

    // 加入交到到 mempool
    pub fn enqueue_transaction(&mut self, tx: Transaction) -> String {
        let hash = tx.hash.clone();
        self.mempool.push_back(tx);
        hash
    }

    pub fn get_transaction_hashes(&self) -> Vec<Vec<u8>> {
        self.tx_hashes.clone()
    }

    // 打包交易
    pub fn process(&mut self) -> Option<Block> {
        if self.mempool.is_empty() {
            println!("没有待处理交易");
            return None;
        }

        let txs: Vec<_> = self.mempool.drain(..).collect();
        let hashes: Vec<_> = txs.iter().map(|tx| tx.tx_hash.clone()).collect();
        self.tx_hashes = hashes.clone();
        let merkle_root = MerkleTree::new(&hashes);

        let block = Block {
            transactions: txs.clone(),
            root: merkle_root.root(),
            merkle: merkle_root.clone(),
        };

        println!(
            "[区块] 构建完成，Merkle 根: {}",
            hex::encode(&merkle_root.root())
        );

        for tx in txs {
            let hash = tx.hash.clone();
            let tx_hash = tx.tx_hash.clone();
            let result = match self.apply_transaction(tx) {
                Ok(res) => {
                    println!("[交易成功] {}", hash);
                    if let ContractResult::ReturnValue(val) = &res {
                        println!("[返回值] {}", val);
                    }
                    TransactionResult {
                        success: true,
                        message: "OK".into(),
                        hash: tx_hash,
                    }
                }
                Err(err) => {
                    println!("[交易失败] {} 错误: {}", hash, err);
                    TransactionResult {
                        success: false,
                        message: err,
                        hash: tx_hash,
                    }
                }
            };
            self.results.push(result);
        }

        Some(block)
    }

    // 验证交易
    pub fn verify(&self, block: Option<Block>, index: usize) -> bool {
        if let Some(block) = block {
            let tree = block.merkle;
            let tx_hashes = self.get_transaction_hashes();

            let leaf = tx_hashes[index].clone();
            return MerkleTree::verify(tree.clone(), leaf, index);
        }

        false
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
