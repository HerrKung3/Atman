//! Data Transfer Object for HTTP requests and responses.

use serde::{Deserialize, Serialize};

use crate::{schema, types::{Address, Hash, Signature}};

#[derive(Debug, Serialize)]
pub struct Tx{
    pub from: Address,
    pub to: Address,
    pub amount: u64,
    pub version: u64,
    pub gas: u64,
    pub gas_price: u64,
    pub timestamp: u64,
}

#[derive(Debug, Serialize)]
pub struct SignedTx {
    pub tx: Tx,
    pub signature: Signature,
}

#[derive(Debug, Serialize)]
pub struct BlockHeader {
    parent_hash: Hash,
    height: u64,
    nonce: u64,
    timestamp: u64,
    author: Address,
    txs_root: Hash,
}

#[derive(Debug, Serialize)]
pub struct Block {
    pub header: BlockHeader,
    pub txs: Vec<SignedTx>,
}

#[derive(Debug, Serialize)]
pub struct BlockResp {
    pub hash: Hash,
    pub block: Block,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBlocksReq {
    pub from_height: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionReq {
    pub account: Address,
}

//TODO(Haydn): Use TxID to identify a transaction
#[derive(Debug, Serialize, Deserialize)]
pub struct TxReq {
    pub from: Address,
    pub to: Address,
    pub version: u64,
    pub amount: u64,
}

impl From<schema::v1::Tx> for Tx {
    fn from(tx: schema::v1::Tx) -> Self {
        Tx { 
            from: tx.receiver.into(), 
            to: tx.sender.into(), 
            amount: tx.amount,
            version: tx.version, 
            gas: tx.gas, 
            gas_price: tx.gas_price, 
            timestamp: tx.timestamp, 
        }
    }
}

impl From<schema::v1::SignedTx> for SignedTx {
    fn from(signed_tx: schema::v1::SignedTx) -> Self {
        SignedTx { 
            tx: signed_tx.tx.unwrap().into(), 
            signature: Signature::from(signed_tx.signature),
        }
    }
}

impl From<schema::v1::BlockHeader> for BlockHeader {
    fn from(header: schema::v1::BlockHeader) -> Self {
        BlockHeader {
            parent_hash: header.parent_hash.into(),
            height: header.height,
            nonce: header.nonce,
            timestamp: header.timestamp,
            author: header.author.into(),
            txs_root: header.txs_root.into(),
        }
    }
}

impl From<schema::v1::Block> for Block {
    fn from(block: schema::v1::Block) -> Self {
        Block { 
            header: block.header.unwrap().into(), 
            txs: block.txs.into_iter().map(|tx|tx.into()).collect(), 
        }
    }
}
