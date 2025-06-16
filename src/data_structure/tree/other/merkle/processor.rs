/*!
  接收交易 + 模拟秒回 + 存储
*/

use crate::data_structure::tree::other::merkle::account::{
    Account, ContractResult, TransactionType,
};
use crate::data_structure::tree::other::merkle::client::Transaction;
use crate::data_structure::tree::other::merkle::gas::GasMeter;
use crate::data_structure::tree::other::merkle::merkle::MerkleTree;
use std::collections::{BinaryHeap, HashMap};

const MAX_SIZE: usize = 10;

#[derive(Default, Clone, Debug)]
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
    mempool: BinaryHeap<Transaction>, // VecDeque<Transaction>
    results: Vec<TransactionResult>,
    tx_hashes: Vec<Vec<u8>>,
    max_block_size: usize,
}

impl Processor {
    pub fn new(accounts: HashMap<String, Account>, max_block_size: Option<usize>) -> Self {
        let max_size = if let Some(size) = max_block_size {
            size
        } else {
            MAX_SIZE
        };

        Self {
            accounts,
            mempool: BinaryHeap::new(),
            results: Vec::new(),
            tx_hashes: vec![],
            max_block_size: max_size,
        }
    }

    fn apply_transaction(&mut self, tx: &Transaction) -> Result<ContractResult, String> {
        match &tx.tx_type {
            TransactionType::Transfer { from, to, amount } => {
                let mut meter = GasMeter::new(tx.gas_limit);
                meter.consume(5)?;
                let from_balance = self.accounts.get_mut(from).ok_or("Sender not found")?;
                if from_balance.balance < *amount {
                    let error_msg = format!(
                        "[交易失败: {} -> {} ({} BTC, gas: {}, priority: {})]: {}",
                        from, to, amount, tx.gas, tx.priority, "Insufficient balance"
                    );
                    return Err(error_msg.into());
                }
                from_balance.balance -= amount;

                let to_balance = self.accounts.entry(to.clone()).or_insert(Account::new(0));
                to_balance.balance += amount;
                println!("=======================");
                println!(
                    "[{} -> {} ({} BTC, gas: {}, priority: {})]: {}",
                    from, to, amount, tx.gas, tx.priority, tx.hash
                );
                println!("[Gas 使用]: 已用: {}, 限制: {}", meter.used(), tx.gas_limit);
                println!("[交易成功]: {}", tx.hash);
                println!("=======================");
                println!("");
                Ok(ContractResult::Ok)
            }
            TransactionType::ContractCall {
                caller,
                contract,
                method,
                args,
            } => {
                println!("=======================");
                println!(
                    "模拟调用合约: {} 调用 {}.{}({:?}), {}",
                    caller, contract, method, args, tx.hash
                );

                let mut meter = GasMeter::new(tx.gas_limit);

                let result = self.simulate_contract_execution(method, args, &mut meter);

                println!("[Gas 使用]: 已用: {}, 限制: {}", meter.used(), tx.gas_limit);
                println!("[交易成功]: {}", tx.hash);
                println!("=======================");
                println!("");
                result
            }
        }
    }

    // 加入交到到 mempool
    pub fn enqueue_transaction(&mut self, tx: Transaction) -> String {
        let hash = tx.hash.clone();
        self.mempool.push(tx);
        hash
    }

    pub fn get_transaction_hashes(&self) -> Vec<Vec<u8>> {
        self.tx_hashes.clone()
    }

    pub fn simulate_contract_execution(
        &self,
        method: &str,
        args: &[String],
        meter: &mut GasMeter,
    ) -> Result<ContractResult, String> {
        match method {
            "get_value" => {
                meter.consume(10)?; // 模拟读取指令
                Ok(ContractResult::ReturnValue("42".into()))
            }
            "set_value" => {
                meter.consume(10)?; // 校验权限
                meter.consume(50)?; // 写入存储
                Ok(ContractResult::Ok)
            }
            "loop_forever" => {
                for _ in 0..10_000 {
                    meter.consume(1)?; // 每次循环消耗 1 gas
                }
                Ok(ContractResult::Ok)
            }
            _ => {
                meter.consume(5)?; // 默认执行
                Ok(ContractResult::Ok)
            }
        }
    }

    // 打包交易
    pub fn process(&mut self) -> Option<Block> {
        if self.mempool.is_empty() {
            println!("没有待处理交易");
            return None;
        }

        let mut txs = Vec::with_capacity(self.max_block_size);

        // let txs: Vec<_> = self.mempool.drain(..).collect();
        // let hashes: Vec<_> = txs.iter().map(|tx| tx.tx_hash.clone()).collect();
        let mut hashes = Vec::new();

        while txs.len() < self.max_block_size {
            if let Some(tx) = self.mempool.pop() {
                println!("Pop tx priority={}, gas={}", tx.priority, tx.gas);
                let hash = tx.hash.clone();
                let tx_hash = tx.tx_hash.clone();
                let result = match self.apply_transaction(&tx) {
                    Ok(res) => {
                        if let ContractResult::ReturnValue(val) = &res {
                            println!("[返回值] {}", val);
                        }
                        TransactionResult {
                            success: true,
                            message: "OK".into(),
                            hash: tx_hash.clone(),
                        }
                    }
                    Err(err) => {
                        println!("[交易失败] {} 错误: {}", hash, err);
                        TransactionResult {
                            success: false,
                            message: err,
                            hash: tx_hash.clone(),
                        }
                    }
                };

                txs.push(tx);
                self.results.push(result);
                self.tx_hashes.push(tx_hash.clone());
                hashes.push(tx_hash.clone());
            } else {
                break;
            }
        }

        if txs.is_empty() {
            return None;
        }

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
