#![allow(dead_code)]
#![allow(unused_variables)]
use chrono::{DateTime, Utc};

mod frontrunning_bot;

// Highly simplified in reality this would listen directly onchain
// and get the expected vs bought price using the transaction pool
// Also would utilise Rust's async model for max performance

// removed unnecessary properties
// uniswap transaction before being confirmed
#[derive(Debug, Clone)]
pub struct UniswapTransaction {
    id: String,
    timestamp: DateTime<Utc>,
    from_address: String,
    token_a: TokenInfo,
    token_b: TokenInfo,
    slippage: f32,
    expected_price: f64,
}

#[derive(Debug, Clone)]
pub struct ConfirmedUniswapTransaction {
    id: String,
    timestamp: DateTime<Utc>,
    from_address: String,
    token_a: TokenInfo,
    token_b: TokenInfo,
    swap_fee: f64,
    bought_price: f64,
}

#[derive(Debug, Clone)]
pub struct TokenInfo {
    pub address: String,
    pub amount: f64,
    // rice in terms of the other token
    pub price: f64,
}

impl TokenInfo {
    fn new(address: String, amount: f64, price: f64) -> Self {
        Self {
            address,
            amount,
            price,
        }
    }
}

pub struct TokenTransactionsPool {
    // user public wallet key
    pub_key: String,
    token_name: String,
}
impl TokenTransactionsPool {
    async fn detect_any_loss(&self, tx_to_investigate: UniswapTransaction) {
        let client = reqwest::Client::new();

        // etherscan confirmed transaction pool example
        let uniswap_txs = client
            .get(format!(
                "https://etherscan.io/apis/transactions={}",
                self.pub_key
            ))
            .send()
            .await?
            .json::<Vec<ConfirmedUniswapTransaction>>()
            .await?;

        //check if transaction exists
        let transaction = uniswap_txs.iter().find(|x| x.id == tx_to_investigate.id);

        if let Some(transaction) = transaction {
            //slippage has to be over 5% for frontrun (usually)
            if transaction.bought_price > tx_to_investigate.expected_price {
                println!(
                    "Potential frontrun tx id: {:?}, price difference: {:?}",
                    transaction.id,
                    (transaction.bought_price - tx_to_investigate.expected_price)
                );
            }
        }
    }
}

fn main() {}
