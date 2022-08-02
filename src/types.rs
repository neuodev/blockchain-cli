use std::fmt;
use crate::utils::{hex_to_decimals, format_label_and_value};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum Method {
    Accounts,
    GasPrice,
    BlockNumber,
    GetBalance,
    TxCount,
    BlockTxCountByHash,
    BlockTxCountByNumber,
    SendTx,
    GetBlockByHash,
    GetBlockByNumber,
    GetTxByHash,
    GetTxReceipt,
}

impl Method {
    pub fn value(self) -> &'static str {
        match self {
            Method::Accounts => "eth_accounts",
            Method::GasPrice => "eth_gasPrice",
            Method::BlockNumber => "eth_blockNumber",
            Method::GetBalance => "eth_getBalance",
            Method::TxCount => "eth_getTransactionCount",
            Method::BlockTxCountByHash => "eth_getBlockTransactionCountByHash",
            Method::BlockTxCountByNumber => "eth_getBlockTransactionCountByNumber",
            Method::SendTx => "eth_sendTransaction",
            Method::GetBlockByHash => "eth_getBlockByHash",
            Method::GetBlockByNumber => "eth_getBlockByNumber",
            Method::GetTxByHash => "eth_getTransactionByHash",
            Method::GetTxReceipt => "eth_getTransactionReceipt",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Body {
    pub jsonrpc: &'static str,
    pub method: &'static str,
    pub params: Vec<String>,
    pub id: u32,
}

impl Body {
    pub fn new(method: Method, params: Vec<String>) -> Body {
        Body {
            jsonrpc: "2.0".into(),
            method: method.value(),
            params,
            id: 1,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockRequest {
    pub jsonrpc: &'static str,
    pub method: &'static str,
    pub params: (String, bool),
    pub id: u32,
}

pub trait ResultParser {
    fn result(&self) -> String;
    fn to_decimals(&self) -> i128 {
        hex_to_decimals(&self.result(), true)
    }

    fn from_wei(&self) -> i128 {
        let base: i128 = 10;
        let num_wei_in_eth: i128 = base.pow(18);
        self.to_decimals() / num_wei_in_eth
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RPCResponse {
    result: String,
}

impl ResultParser for RPCResponse {
    fn result(&self) -> String {
        self.result.clone()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockResponse {
    pub jsonrpc: String,
    pub id: u32,
    pub result: BlockHex,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockHex {
    #[serde(rename = "baseFeePerGas")]
    base_gas_fee: String,
    difficulty: String,
    #[serde(rename = "gasLimit")]
    gas_limit: String,
    #[serde(rename = "gasUsed")]
    gas_used: String,
    hash: String,
    miner: String,
    #[serde(rename = "mixHash")]
    mix_hash: String,
    nonce: String,
    number: String,
    #[serde(rename = "parentHash")]
    parent_hash: String,
    size: String,
    timestamp: String,
    #[serde(rename = "totalDifficulty")]
    total_difficulty: String,
    pub transactions: Vec<TransactionHex>,
}

impl BlockHex {
    pub fn parse(&self) -> Block {
        Block {
            base_gas_fee: hex_to_decimals(&self.base_gas_fee, true),
            difficulty: hex_to_decimals(&self.difficulty, true) as i32,
            gas_limit: hex_to_decimals(&self.gas_limit, true),
            gas_used: hex_to_decimals(&self.gas_used, true),
            hash: self.hash.clone(),
            miner: self.miner.clone(),
            mix_hash: self.mix_hash.clone(),
            nonce: hex_to_decimals(&self.nonce, true) as i32,
            number: hex_to_decimals(&self.number, true) as i32,
            parent_hash: self.parent_hash.clone(),
            size: hex_to_decimals(&self.size, true) as i32,
            timestamp: hex_to_decimals(&self.timestamp, true) as u32,
            total_difficulty: hex_to_decimals(&self.total_difficulty, true) as u32,
            transactions: self
                .transactions
                .clone()
                .into_iter()
                .map(|tx| tx.prase())
                .collect(),
        }
    }
}

#[derive(Debug)]
pub struct Block {
    base_gas_fee: i128,
    difficulty: i32,
    gas_limit: i128,
    gas_used: i128,
    hash: String,
    miner: String,
    mix_hash: String,
    nonce: i32,
    number: i32,
    parent_hash: String,
    size: i32,
    timestamp: u32,
    total_difficulty: u32,
    transactions: Vec<Transaction>,
}


impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lines = [
            ("Hash", self.hash.clone()),
            ("Block Number", self.number.to_string()),
            ("Transactions Count", self.transactions.len().to_string()),
            ("Miner", self.miner.clone()),
            ("Base Gase Fee", self.base_gas_fee.to_string()),
            ("Difficulty", self.difficulty.to_string()),
            ("Gas Limit", self.gas_limit.to_string()),
            ("Gas Used", self.gas_used.to_string()),
            ("Mix Hash", self.mix_hash.to_string()),
            ("Nonce", self.nonce.to_string()),
            ("Parent Hash", self.parent_hash.to_string()),
            ("Size", self.size.to_string()),
            ("Timestamp", self.timestamp.to_string()),
            ("Total Difficulty", self.total_difficulty.to_string())
        ];

        for (label, value) in lines {
            write!(f, "{}", format_label_and_value(label, &value))?;
        }

        println!("Block Transaction: ");
        for tx in &self.transactions {
            println!(">>>>>>>>>>>>>>>>>>>>>>>>><<<<<<<<<<<<<<<<<<<<<<<<<<<<<<");
            println!("{}", tx);
        }

        write!(f, "")
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransactionHex {
    #[serde(rename = "blockHash")]
    block_hash: String,
    #[serde(rename = "blockNumber")]
    block_number: String,
    from: String,
    gas: String,
    #[serde(rename = "gasPrice")]
    gas_price: String,
    #[serde(rename = "maxPriorityFeePerGas")]
    max_priority_fee_per_gas: Option<String>,
    #[serde(rename = "maxFeePerGas")]
    max_fee_per_gas: Option<String>,
    hash: String,
    nonce: String,
    to: Option<String>,
    #[serde(rename = "transactionIndex")]
    tx_idx: String,
    value: String,
}

impl TransactionHex {
    pub fn prase(&self) -> Transaction {
        Transaction {
            block_hash: self.block_hash.clone(),
            block_number: hex_to_decimals(&self.block_number, true) as i32,
            from: self.from.clone(),
            gas: hex_to_decimals(&self.gas, true),
            gas_price: hex_to_decimals(&self.gas_price, true),
            max_priority_fee_per_gas: match self.max_priority_fee_per_gas.clone() {
                Some(val) => Some(hex_to_decimals(&val, true)),
                None => None,
            },
            max_fee_per_gas: match self.max_fee_per_gas.clone() {
                Some(val) => Some(hex_to_decimals(&val, true)),
                None => None,
            },
            hash: self.hash.clone(),
            nonce: hex_to_decimals(&self.nonce, true) as i32,
            to: self.to.clone(),
            tx_idx: hex_to_decimals(&self.tx_idx, true) as i32,
            value: hex_to_decimals(&self.value, true),
        }
    }
}

#[derive(Debug)]
pub struct Transaction {
    block_hash: String,
    block_number: i32,
    from: String,
    gas: i128,
    gas_price: i128,
    max_priority_fee_per_gas: Option<i128>,
    max_fee_per_gas: Option<i128>,
    hash: String,
    nonce: i32,
    to: Option<String>,
    tx_idx: i32,
    value: i128,
}


impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let to= match self.to.clone() {
            Some(val) => val,
            None => "??".to_string()
        };

        let max_priority_fee_per_gas = match self.max_priority_fee_per_gas {
            Some(val) => val.to_string(),
            None => "??".to_string()
        };

        let max_fee_per_gas = match self.max_fee_per_gas {
            Some(val) => val.to_string(),
            None => "??".to_string()
        };


        let lines = [
            ("Hash", self.hash.clone()),
            ("From", self.from.clone()),
            ("To", to),
            ("Value", self.value.to_string()),
            ("Block Number", self.block_number.to_string()),
            ("Block Hash", self.block_hash.clone()),
            ("Gas", self.gas.to_string()),
            ("Gas Price", self.gas_price.to_string()),
            ("Max priority fee per gas", max_priority_fee_per_gas),
            ("Max fee per gas", max_fee_per_gas),
            ("Nonce", self.nonce.to_string()),
            ("Tx Idx", self.tx_idx.to_string()),
        ];

        for (label, value) in lines {
            write!(f, "{}", format_label_and_value(label, &value))?;
        }

        write!(f, "")
    }
}
