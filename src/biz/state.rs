use std::{collections::HashMap, fmt::Debug};

use crate::{error::Error, schema::v1::Block, types::Hash};

pub trait State: Debug + Clone + Send + Sync + 'static {
    fn block_height(&self) -> u64;

    fn version(&self) -> u64;

    fn last_block_hash(&self) -> Option<Hash>;

    fn add_block(&self, block: Block) -> Result<(), Error>;

    fn get_block(&self, height: u64) -> Option<Block>;

    fn get_blocks(&self, from_height: u64) -> Vec<Option<Block>>;

    fn get_balance(&self) -> u64;

    fn get_balances(&self) -> HashMap<String, u64>;

    fn get_version(&self) -> HashMap<String, u64>;
}