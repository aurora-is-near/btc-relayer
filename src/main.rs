use bitcoincore_rpc::bitcoin::block::Header;
use log::{debug, error, log_enabled, info, Level};
use serde_json::{from_slice, json};

use crate::config::Config;
use crate::near_client::Client as NearClient;
use crate::bitcoin_client::Client as BitcoinClient;

mod near_client;
mod bitcoin_client;
mod config;

struct Synchronizer {
    bitcoin_client: BitcoinClient,
    near_client: NearClient,
}

impl Synchronizer {
    pub fn new(bitcoin_client: BitcoinClient, near_client: NearClient) -> Self {
        Self { bitcoin_client, near_client }
    }
    async fn sync(&mut self) {
        let mut current_height = 0;

        loop {
            // Get the latest block height from the Bitcoin client
            let latest_height = self.bitcoin_client.get_block_count();

            // Check if we have reached the latest block height
            if current_height >= latest_height {
                // Wait for a certain duration before checking for new blocks
                tokio::time::sleep(std::time::Duration::from_secs(60)).await;
                continue;
            }

            let block_hash = self.bitcoin_client.get_block_hash(current_height);
            let block_header = self.bitcoin_client.get_block_header(&block_hash);

            // TODO: It is OK to catch up, but to read everything in this way is not efficient
            // TODO: block confirmation issue (what if we are forking)
            self.near_client
                .submit_block_header(block_header.clone())
                .await
                .expect("to submit a block header successfully");

            if current_height >= 0 {
                // Only do one iteration for testing purpose
                break;
            }

            current_height += 1;
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let config = Config::new().expect("we expect config.toml to be next to executable in `./`");

    debug!("Configuration loaded: {:?}", config);

    let bitcoin_client = BitcoinClient::new(config.clone());
    let near_client = NearClient::new(config.clone());

    let best_block_hash = bitcoin_client.get_best_block_hash();
    debug!("best block hash: {}", best_block_hash);

    info!("run block header sync");
    let mut synchonizer = Synchronizer::new(bitcoin_client, near_client.clone());
    synchonizer.sync().await;
    info!("end block header sync");

    near_client.read_last_block_header().await.expect("read block header succesfully");

    Ok(())
}