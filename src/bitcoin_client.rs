use bitcoincore_rpc::bitcoin::block::Header;
use bitcoincore_rpc::RpcApi;
use bitcoincore_rpc::bitcoin::BlockHash;

use crate::config::Config;

pub struct Client {
    config: Config,
    inner: bitcoincore_rpc::Client
}

impl Client {
    pub fn new(config: Config) -> Self {
        let inner = bitcoincore_rpc::Client::new(
            &config.bitcoin.endpoint,
            bitcoincore_rpc::Auth::UserPass(
                config.bitcoin.node_user.clone(),
                config.bitcoin.node_password.clone()
            )
        ).expect("failed to create a bitcoin client");

        Self {
            config,
            inner
        }
    }

    pub fn get_best_block_hash(&self) -> BlockHash {
        self.inner.get_best_block_hash().unwrap()
    }

    pub fn get_block_count(&self) -> u64 {
        self.inner.get_block_count().unwrap()
    }

    pub fn get_block_hash(&self, height: u64) -> BlockHash {
        self.inner.get_block_hash(height).unwrap()
    }

    pub fn get_block_header(&self, block_hash: &BlockHash) -> Header {
        self.inner.get_block_header(block_hash).unwrap()
    }
}