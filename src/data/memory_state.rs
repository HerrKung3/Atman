use std::collections::HashMap;

use crate::biz::state::State;
use crate::error::Error;
use crate::schema::v1::Block;
use crate::types::Hash;

#[derive(Debug, Clone)]
pub struct MemoryState {}

impl State for MemoryState {
    fn block_height(&self) -> u64 {
        todo!()
    }

    fn version(&self) -> u64 {
        todo!()
    }

    fn last_block_hash(&self) -> Option<Hash> {
        todo!()
    }

    fn add_block(&self, block: Block) -> Result<(), Error> {
        todo!()
    }

    fn get_version(&self) -> HashMap<String, u64> {
        todo!()
    }

    fn get_block(&self, height: u64) -> Option<Block> {
        todo!()
    }

    fn get_blocks(&self, from_height: u64) -> Vec<Option<Block>> {
        todo!()
    }

    fn get_balance(&self) -> u64 {
        todo!()
    }

    fn get_balances(&self) -> HashMap<String, u64> {
        todo!()
    }
}