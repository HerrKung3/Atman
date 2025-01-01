use std::fmt;

use prost::Message;

use crate::error::Error;
use crate::utils::{self, gen_random_number, unix_timestamp};
use crate::schema::v1::{Block, BlockHeader, SignedTx};
use crate::types::{Address, Hash};

const PUZZLE_REWARD: u64 = 25;

impl Block {
    pub fn new(header: Option<BlockHeader>, txs: Vec<SignedTx>) -> Self {
        Block {
            header,
            txs,
        }
    }

    pub fn block_reward(&self) -> u64 {
        let txs_pack_reward: u64 = self.txs.iter().map(|tx|match tx.gas_cost() {
            Ok(cost) => cost,
            Err(_) => 0,
        }).sum();

        txs_pack_reward + PUZZLE_REWARD
    }

    // pub fn parent_hash(&self) -> Result<Hash, Error> {
    //     let header = self.header()?;
    //     Ok(header.parent_hash())
    // }

    // pub fn height(&self) -> Result<u64, Error> {
    //     let header = self.header()?;
    //     Ok(header.height())
    // }

    // pub fn nonce(&self) -> Result<u64, Error> {
    //     let header = self.header()?;
    //     Ok(header.nonce())
    // }

    // pub fn timestamp(&self) -> Result<u64, Error> {
    //     let header = self.header()?;
    //     Ok(header.nonce())
    // }

    // pub fn author(&self) -> Result<Address, Error> {
    //     let header = self.header()?;
    //     Ok(header.author())
    // }
    
    // pub fn hash(&self) -> Result<Hash, Error> {
    //     let header = self.header()?;
    //     Ok(header.hash())
    // }    

    // pub fn update_nonce_and_timestamp(&mut self) -> Result<(), Error> {
    //     let header: &mut BlockHeader = self.header_mut()?;
    //     header.update_nonce_and_timestamp();
    //     Ok(())
    // }

    pub fn header(&self) -> Result<&BlockHeader, Error> {
        match &self.header {
            Some(header) => Ok(header),
            None => Err(Error::EmptyHeader)
        }
    }

    pub fn header_mut(&mut self) -> Result<&mut BlockHeader, Error> {
        match &mut self.header {
            Some(header) => Ok(header),
            None => Err(Error::EmptyHeader)
        }
    }
}

impl BlockHeader {
    pub fn new(
        parent_hash: Hash, 
        height: u64, 
        nonce: u64, 
        //timestamp: u64, 
        author: Address, 
        txs_root: Hash
    ) -> Self {
        let parent_hash: Vec<u8> = parent_hash.to_vec();
        BlockHeader {
            parent_hash,
            height,
            nonce,
            timestamp: unix_timestamp(),
            author: author.into(),
            txs_root: txs_root.into(),
        }
    }

    // pub fn parent_hash(&self) -> Hash {
    //     self.parent_hash.clone().into()
    // }

    // pub fn height(&self) -> u64 {
    //     self.height
    // }

    // pub fn nonce(&self) -> u64 {
    //     self.nonce
    // }

    // pub fn timestamp(&self) -> u64 {
    //     self.timestamp
    // }

    // pub fn author(&self) -> Address {
    //     self.author.clone().into()
    // }

    pub fn hash(&self) -> Hash {
        utils::hash(&self.encode_to_vec())
    }

    // pub fn txs_root(&self) -> Hash {
    //     self.txs_root.clone().into()
    // }

    pub fn update_nonce_and_timestamp(&mut self) {
        self.timestamp = unix_timestamp();
        self.nonce = gen_random_number::<u64>();
    }

}

impl TryFrom<Vec<u8>> for Block {
    type Error = Error;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self::decode(value.as_slice())?)
    }
}

impl From<Block> for Vec<u8> {
    fn from(value: Block) -> Self {
        value.encode_to_vec()
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let header = self.header().map_err(|_|fmt::Error)?;
        write!(f, "Block {{ \n{}, \nTxs: [", header)?;
        for tx in self.txs.iter() {
            write!(f, "{}, ", tx)?;
        };
        write!(f, "] \n}}")
    }
}

impl fmt::Display for BlockHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BlockHeader: {{ \nparent hash: {:?}, \nheight: {}, \nnonce: {}, \ntimestamp: {}, \nauthor: {:?}, \ntxs root: {:?} \n}}", 
        self.parent_hash, self.height, self.nonce, self.timestamp, self.author, self.txs_root)
    }
}

#[cfg(test)]
mod test {
    use crate::{schema::v1::{Block, BlockHeader, SignedTx, Tx}, types::Bytes, utils};

    #[test]
    fn block_test() {
        let sender1 = Bytes::<32>::new_for_test();
        let receiver1 = Bytes::<32>::new_for_test();
        let signature1 = Bytes::<65>::new_for_test();

        let raw_tx1 = Tx::new(sender1, receiver1, 100, 2);
        let signed_tx1 = SignedTx::new(Some(raw_tx1), signature1);

        let sender2 = Bytes::<32>::new_for_test();
        let receiver2 = Bytes::<32>::new_for_test();
        let signature2 = Bytes::<65>::new_for_test();

        let raw_tx2 = Tx::new(sender2, receiver2, 200, 3);
        let signed_tx2 = SignedTx::new(Some(raw_tx2), signature2);

        let txs = vec![signed_tx1, signed_tx2];
        let mut flatten_txs: Vec<u8> = vec![];
        for tx in txs.clone() {
            let serialized: Vec<u8> = tx.into();
            flatten_txs.extend(serialized);
        }

        let txs_root = utils::hash(&flatten_txs);
        let parent_hash = Bytes::<32>::new_for_test();
        let author = Bytes::<32>::new_for_test();

        let block_header = BlockHeader::new(parent_hash, 24, 34, author, txs_root);

        let block = Block::new(Some(block_header), txs);

        println!("block = {}", block);
    }
}