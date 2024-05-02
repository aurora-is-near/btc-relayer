use near_jsonrpc_client::methods;

use bitcoincore_rpc::{Auth, Client as BitcoinClient, RpcApi};
use bitcoincore_rpc::bitcoin::block::Header;

mod utils;

// Keep the BitcoinRelay and Synchronizer structs the same as before

struct BitcoinRelay {
}

impl BitcoinRelay {
    fn save_block_header(&self, block_header: Header) {
        // Save the block header
        println!("Saving block header: {:?}", block_header);
    }
}

struct Synchronizer {
    rpc: BitcoinClient,
    relay: BitcoinRelay,
}

impl Synchronizer {
    pub fn new(rpc: BitcoinClient, relay: BitcoinRelay) -> Self {
        Self { rpc, relay }
    }
    fn sync(&mut self) {
        let mut current_height = 0;

        loop {
            let block_hash = self.rpc.get_block_hash(current_height).unwrap();
            let block_header = self.rpc.get_block_header(&block_hash).unwrap();

            self.relay.save_block_header(block_header);

            current_height += 1;

            // TODO: How to stop properly?
            if current_height == 1_000 {
                break;
            }
        }
    }
}

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

    let rpc = BitcoinClient::new(
        "http://127.0.0.1:8332",
        Auth::UserPass(
            "raven".to_string(),
            "raven_is_here_for_your_bitcoins".to_string()
        )
    ).expect("failed to create a bitcoin client");

    let best_block_hash = rpc.get_best_block_hash().unwrap();
    println!("best block hash: {}", best_block_hash);

    println!("run block reader");
    let relay = BitcoinRelay {};
    let mut synchonizer = Synchronizer::new(rpc, relay);

    synchonizer.sync();
    println!("end running block reader");

    return Ok(());

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