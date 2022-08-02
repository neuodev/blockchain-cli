use colored::*;
use std::{io::{self, Write}};

use crate::{rpc_calls::RpcCalls, types::ResultParser};

pub enum Option {
    GetAccounts,
    GetGasPrice,
    GetBlockNumber,
    GetBalance,
    GetTxCount,
    GetBlockTxCount,
    GetBlock,
    GetTx,
    None,
}

impl Option {
    pub fn value(&self) -> &'static str {
        match self {
            Option::GetAccounts => "1) Get node accounts",
            Option::GetGasPrice => "2) Get gase price",
            Option::GetBlockNumber => "3) Get Block Number",
            Option::GetBalance => "4) Get balance",
            Option::GetTxCount => "5) Get tx count for an address",
            Option::GetBlockTxCount => "6) Get block tx count",
            Option::GetBlock => "7) Get block info",
            Option::GetTx => "8) Get transaction",
            Option::None => "Invalid input",
        }
    }

    pub fn get_option_by_idx(idx: u32) -> Option {
        match idx {
            1 => Option::GetAccounts,
            2 => Option::GetGasPrice,
            3 => Option::GetBlockNumber,
            4 => Option::GetBalance,
            5 => Option::GetTxCount,
            6 => Option::GetBlockTxCount,
            7 => Option::GetBlock,
            8 => Option::GetTx,
            _ => Option::None,
        }
    }
}

pub struct CommandLine;
impl CommandLine {
    pub async fn select_option() -> Result<bool, reqwest::Error>{
        let options = [
            Option::GetAccounts,
            Option::GetGasPrice,
            Option::GetBlockNumber,
            Option::GetBalance,
            Option::GetTxCount,
            Option::GetBlockTxCount,
            Option::GetBlock,
            Option::GetTx,
        ];

        for option in &options {
            println!(
                "{}",
                format!("{}", option.value()).underline().bold().cyan()
            )
        }

        let input = CommandLine::user_input("Select an option: ");
        let option_idx = input.parse::<u32>().unwrap_or_else(|_| 0);
        let option = Option::get_option_by_idx(option_idx);
        CommandLine::execute_option(option).await?;
        let should_continue = match CommandLine::user_input("Continue?(Y/N): ").as_str() {
            "Y" | "y" => true,
            "N" | "n" => false,
            _ => true,
        };

        Ok(should_continue)
    }

    fn user_input(prefix: &str) -> String {
        print!("{}", format!("{}", prefix).bold().on_cyan());
        io::stdout().flush().unwrap();
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        let input = buf.trim();

        input.to_string()
    }

    async fn execute_option(option: Option) -> Result<(), reqwest::Error> {
        match option {
            Option::GetBalance => {
                println!("Get user balance.");
                let addr = CommandLine::user_input("Address: ");
                CommandLine::loading();
                let balance = RpcCalls::get_balance(addr.as_str(), None).await.unwrap();
                CommandLine::display_label_and_value("ETH", balance.from_wei().to_string().as_str());
                CommandLine::display_label_and_value("Wei", balance.to_decimals().to_string().as_str());
                Ok(())
            }, 
            Option::GetAccounts => {
                println!("Get node accounts...");
                CommandLine::loading();
                let accounts = RpcCalls::get_accounts().await?;
                println!("{:#?}", accounts);
                Ok(())
            },
            Option::GetGasPrice => {
                println!("Get Gas price...");
                CommandLine::loading();
                let gas_price = RpcCalls::gas_price().await?;
                let gas_in_wei = gas_price.to_decimals();
                CommandLine::display_label_and_value("Wei", gas_in_wei.to_string().as_str());
                Ok(())
            },
            Option::GetBlockNumber => {
                println!("Get latest block number...");
                CommandLine::loading();
                let block_number = RpcCalls::block_number().await?;
                CommandLine::display_label_and_value("block", block_number.to_decimals().to_string().as_str());
                Ok(())
            },
            Option::GetTxCount => {
                println!("Get tx counts...");
                let addr = CommandLine::user_input("Address: ");
                CommandLine::loading();
                let tx_count = RpcCalls::get_tx_count(addr.as_str(), None).await?;
                CommandLine::display_label_and_value("Transaction", tx_count.to_decimals().to_string().as_str());
                Ok(())
            },
            Option::GetBlockTxCount => {
                println!("Get tx counts...");
                let number = match CommandLine::user_input("Block Number: ").parse::<i32>() {
                    Ok(val) => match val {
                            0 => None,
                            _ => Some(val)
                        }
                    ,
                    Err(e) => {
                        println!("Invalid block number: {:#?}", e.kind());
                        return Ok(())
                    }
                };
                CommandLine::loading();
                let tx_count = RpcCalls::block_tx_count(None, number).await?;
                CommandLine::display_label_and_value("Transaction", tx_count.to_decimals().to_string().as_str());
                Ok(())
            },
            Option::GetBlock => {
                println!("Get block info...");
                let number = match CommandLine::user_input("Block Number: ").parse::<i32>() {
                    Ok(val) => match val {
                            0 => None,
                            _ => Some(val)
                        }
                    ,
                    Err(e) => {
                        println!("Invalid block number: {:#?}", e.kind());
                        return Ok(())
                    }
                };
                CommandLine::loading();
                let block = RpcCalls::get_block(None, number).await?;
                println!("{}", block);
                Ok(())
            },
            Option::GetTx => {
                println!("Get Tx info...");
                let tx_hash = CommandLine::user_input("Tx Hash: ");
                CommandLine::loading();
                let tx = RpcCalls::get_tx(tx_hash.as_str()).await?;
                println!("{}", tx);
                Ok(())
            },
            _ => {
                println!("In progress");
                Ok(())
            }
        }
    }

    fn loading() {
        println!("{}", format!("Fetching...").bold().green())
    }

    fn display_label_and_value(label: &str, value: &str) {
        println!("> {}", format!("{} {}", value, label).bold().underline().bright_blue());
    }
}

// addr1 = 0xf1a9e8f520b3427b6326356731a5cb4389337516
// addr2 = 0x4d684f86ed2084484c6547975533151128b0c8bd
// Block number 12710481