use near_jsonrpc_client::{methods, JsonRpcClient};
use near_jsonrpc_primitives::types::transactions::TransactionInfo;

use bitcoincore_rpc::{Auth, Client, RpcApi};

mod utils;

pub fn specify_block_reference() -> std::io::Result<near_primitives::types::BlockReference> {
    println!("=========[Block Reference]=========");
    let block_reference = utils::select(
        || {
            println!(" [1] final        \x1b[38;5;244m(alias: f, fin)\x1b[0m");
            println!(" [2] optimistic   \x1b[38;5;244m(alias: o, opt)\x1b[0m");
            println!(" [3] block hash   \x1b[38;5;244m(alias: s, hash)\x1b[0m");
            println!(" [4] block height \x1b[38;5;244m(alias: h, height)\x1b[0m");
        },
        "\x1b[33m(enter a selection)\x1b[0m> ",
        |selection| match (selection, selection.parse()) {
            ("f" | "fin" | "final", _) | (_, Ok(1)) => {
                Some(near_primitives::types::BlockReference::Finality(
                    near_primitives::types::Finality::Final,
                ))
            }
            ("o" | "opt" | "optimistic", _) | (_, Ok(2)) => {
                Some(near_primitives::types::BlockReference::Finality(
                    near_primitives::types::Finality::None,
                ))
            }
            ("s" | "hash" | "block hash", _) | (_, Ok(3)) => loop {
                match utils::input("What block hash should we query? ")
                    .unwrap()
                    .parse()
                {
                    Ok(block_hash) => {
                        break Some(near_primitives::types::BlockReference::BlockId(
                            near_primitives::types::BlockId::Hash(block_hash),
                        ))
                    }
                    _ => println!("(i) Invalid block hash, please reenter!"),
                }
            },
            ("h" | "height" | "block height", _) | (_, Ok(4)) => loop {
                match utils::input("What block height should we query? ")
                    .unwrap()
                    .parse()
                {
                    Ok(block_height) => {
                        break Some(near_primitives::types::BlockReference::BlockId(
                            near_primitives::types::BlockId::Height(block_height),
                        ))
                    }
                    _ => println!("(i) Invalid block height, please reenter!"),
                }
            },
            _ => None,
        },
    )?;
    println!("===================================");

    Ok(block_reference)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let client = utils::select_network()?;

    // tolerate only 3 retries
    for _ in 1..=3 {
        let block_reference = specify_block_reference()?;

        match client
            .call(methods::block::RpcBlockRequest { block_reference })
            .await
        {
            Ok(block_details) => println!("{:#?}", block_details),
            Err(err) => match err.handler_error() {
                Some(methods::block::RpcBlockError::UnknownBlock { .. }) => {
                    println!("(i) Unknown block!");
                    continue;
                }
                Some(err) => {
                    println!("(i) An error occurred `{:#?}`", err);
                    continue;
                }
                _ => println!("(i) A non-handler error ocurred `{:#?}`", err),
            },
        };
        break;
    }

    Ok(())
}

/*fn main() {
    let mainnet_client = JsonRpcClient::connect("https://archival-rpc.mainnet.near.org");

    let tx_status_request = methods::tx::RpcTransactionStatusRequest {
        transaction_info: TransactionInfo::TransactionId {
            hash: "9FtHUFBQsZ2MG77K3x3MJ9wjX3UT8zE1TczCrhZEcG8U".parse()?,
            account_id: "miraclx.near".parse()?,
        },
    };

    // call a method on the server via the connected client
    let tx_status = mainnet_client.call(tx_status_request).await?;

    println!("{:?}", tx_status);


    /*let rpc = Client::new("http://localhost:8332",
                          Auth::UserPass("<FILL RPC USERNAME>".to_string(),
                                         "<FILL RPC PASSWORD>".to_string())).unwrap();

    let best_block_hash = rpc.get_best_block_hash().unwrap();
    println!("best block hash: {}", best_block_hash);*/
}*/