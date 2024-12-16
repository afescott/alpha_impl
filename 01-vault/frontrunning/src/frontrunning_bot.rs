use tungstenite::connect;

pub enum TokenType {
    Eth,
    Btc,
    Usdt,
}

pub enum GasFeeLevel {
    Standard,
    High,
    //e.g. frontrunning
    Extreme,
}

pub struct PendingTransactions<'a, P> {
    pending_transactions: Vec<PendingTransaction<'a, P>>,
}

pub struct PendingTransaction<'a, P> {
    tx_hash: TxHash,
    confirmations: usize,
    provider: &'a Provider<P>,
    state: PendingTxState<'a>,
    interval: Box<dyn Stream<Item = ()> + Send + Unpin>,
    retries_remaining: usize,
}

pub struct FrontRunningBot {
    //bot wallet private key
    priv_key: String,
    //e.g. ETH:USDT_Pairs
    token_targets: (TokenType, TokenType),
    target_contract: String,
    gas_level: GasFeeLevel,
}

impl FrontRunningBot {
    fn new(
        token_1: TokenType,
        token_2: TokenType,
        target_contract: String,
        priv_key: String,
    ) -> Self {
        Self {
            //e.g. ETH_USDT pairs
            token_targets: (TokenType::Eth, TokenType::Usdt),
            target_contract,
            gas_level: GasFeeLevel::Extreme,
            priv_key,
        }
    }

    // run on async thrad
    fn frontrun_loop() {
        //some kind of websocket to listen onchain to mempool
        let (mut socket, response) = connect(String::from(
            "https://ethereum.mainnet.parity_pending_transactions.com",
        ))
        .expect("Can't connect");

        loop {
            let pending_transactions = socket
                .read_message()
                .expect("Error reading message")
                .try_into::<PendingTransactions>()
                .unwrap();

            for ele in pending_transactions {
                //1) let = if msg.struct_properties match contract requirements
                //2 Get transaction info e.g..
                // let get_transaction = reqwest::get("www.etherscan.io/{transaction_info}").await?;
                //3) if transaction causes price fluctuation with high slippage and likely to succeed
                //4) Sign and submit tx
            }
        }
    }
}
