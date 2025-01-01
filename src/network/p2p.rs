use crate::{schema, biz::peer_client::PeerClient};

#[derive(Debug, Clone)]
pub struct P2pClient;

impl PeerClient for P2pClient {
   fn known_peers(&self) -> Vec<String> {
       todo!()
   }

   fn get_block_height(&self, peer_id: &str) -> Result<u64, crate::error::Error> {
       todo!()
   }

   fn get_blocks(&self, peer_id: &str, from_height: u64) -> Result<Vec<schema::v1::Block>, crate::error::Error> {
       todo!()
   }

   fn broadcast_block(&self, block: schema::v1::Block) {
       todo!()
   }

   fn broadcast_tx(&self, tx: schema::v1::SignedTx) {
       todo!()
   }
}