use std::fmt::Debug;

use crate::{error::Error, schema::v1::{Block, SignedTx}};

pub trait PeerClient: Debug + Clone + Send + Sync + 'static {
    fn known_peers(&self) -> Vec<String>;

    fn get_block_height(&self, peer_id: &str) -> Result<u64, Error>;

    fn get_blocks(&self, peer_id: &str, from_height: u64) -> Result<Vec<Block>, Error>;

    fn broadcast_tx(&self, tx: SignedTx);

    fn broadcast_block(&self, block: Block);
}