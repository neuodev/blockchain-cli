use crate::types::{
    Block, BlockHex, BlockRequest, Body, Method, RPCResponse, Transaction, TransactionHex,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

//Todo: Should be an env var or set by using CLI
const RPC: &str = "https://rpc.ankr.com/eth_ropsten";
pub struct RpcCalls;

impl RpcCalls {
    pub async fn get_accounts() -> Result<Vec<String>, reqwest::Error> {
        let body = Body::new(Method::Accounts, vec![]);
        let res = RpcCalls::call::<Value, Body>(body).await?;
        let result = res.get("result").unwrap().clone();
        let accounts: Vec<String> = serde_json::from_value(result).unwrap();
        Ok(accounts)
    }

    pub async fn gas_price() -> Result<RPCResponse, reqwest::Error> {
        let body = Body::new(Method::GasPrice, vec![]);
        let res = RpcCalls::call::<RPCResponse, Body>(body).await?;
        Ok(res)
    }

    pub async fn block_number() -> Result<RPCResponse, reqwest::Error> {
        let body = Body::new(Method::BlockNumber, vec![]);
        let res = RpcCalls::call::<RPCResponse, Body>(body).await?;
        Ok(res)
    }

    pub async fn get_balance(
        addr: &str,
        block: Option<i32>,
    ) -> Result<RPCResponse, reqwest::Error> {
        let block = match block {
            Some(val) => val.to_string(),
            None => String::from("latest"),
        };
        let body = Body::new(Method::GetBalance, vec![addr.to_string(), block]);
        let res = RpcCalls::call::<RPCResponse, Body>(body).await?;
        Ok(res)
    }

    pub async fn get_tx_count(
        addr: &str,
        block: Option<i32>,
    ) -> Result<RPCResponse, reqwest::Error> {
        let block = match block {
            Some(val) => val.to_string(),
            None => String::from("latest"),
        };
        let body = Body::new(Method::TxCount, vec![addr.to_string(), block]);
        let res = RpcCalls::call::<RPCResponse, Body>(body).await?;
        Ok(res)
    }

    pub async fn block_tx_count(
        block_hash: Option<&str>,
        block_number: Option<i32>,
    ) -> Result<RPCResponse, reqwest::Error> {
        let body: Body;

        if let Some(val) = block_hash {
            body = Body::new(Method::BlockTxCountByHash, vec![val.to_string()]);
        } else {
            let block_number = match block_number {
                Some(b) => b.to_string(),
                None => String::from("latest"),
            };

            body = Body::new(Method::BlockTxCountByNumber, vec![block_number]);
        }

        let res = RpcCalls::call::<RPCResponse, Body>(body).await?;

        Ok(res)
    }

    pub async fn get_block(
        block_hash: Option<&str>,
        block_number: Option<i32>,
    ) -> Result<Block, reqwest::Error> {
        let body: BlockRequest;

        if let Some(val) = block_hash {
            body = BlockRequest {
                id: 1,
                jsonrpc: "2.0".into(),
                method: Method::GetBlockByHash.value(),
                params: (val.to_string(), true),
            };
        } else {
            let block_number = match block_number {
                Some(b) => b.to_string(),
                None => String::from("latest"),
            };

            body = BlockRequest {
                id: 1,
                jsonrpc: "2.0".into(),
                method: Method::GetBlockByNumber.value(),
                params: (block_number, true),
            };
        }

        let res = RpcCalls::call::<Value, BlockRequest>(body).await?;
        let result = res.get("result").unwrap().clone();
        let block_hex: BlockHex = serde_json::from_value(result).unwrap();
        Ok(block_hex.parse())
    }

    pub async fn get_tx(hash: &str) -> Result<Transaction, reqwest::Error> {
        let body = Body::new(Method::GetTxByHash, vec![hash.to_string()]);
        let res = RpcCalls::call::<Value, Body>(body).await?;
        let result = res.get("result").unwrap().clone();
        let tx: TransactionHex = serde_json::from_value(result).unwrap();
        Ok(tx.prase())
    }

    pub async fn call<'a, T: for<'de> Deserialize<'de>, B: Serialize>(
        body: B,
    ) -> Result<T, reqwest::Error> {
        let client = reqwest::Client::new();
        let res = client
            .post(RPC)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .json(&body)
            .send()
            .await?;
        let body = res.text().await?;
        let res: T = serde_json::from_str(body.as_str()).unwrap();
        Ok(res)
    }
}
