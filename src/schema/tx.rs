use core::fmt;

use prost::Message;

use crate::{error::Error, utils, types::{Address, Hash, Signature}};

use super::v1::{SignedTx, Tx};

const GAS: u64 = 21;

const GAS_PRICE: u64 = 1;

impl SignedTx {
    pub fn new(tx: Option<Tx>, signature: Signature) -> Self {
        let signature = signature.to_vec();
        SignedTx {
            tx,
            signature,
        }
    }

    pub fn signature(&self) -> Signature {
        self.signature.clone().into()
    }

    // pub fn sender(&self) -> Result<Vec<u8>, Error> {
    //     let raw_tx = self.raw_tx()?;
    //     Ok(raw_tx.sender())
    // }

    // pub fn receiver(&self) -> Result<Vec<u8>, Error> {
    //     let raw_tx = self.raw_tx()?;
    //     Ok(raw_tx.receiver())
    // }

    // pub fn amount(&self) -> Result<u64, Error> {
    //     let raw_tx = self.raw_tx()?;
    //     Ok(raw_tx.amount())
    // }

    // pub fn version(&self) -> Result<u64, Error> {
    //     let raw_tx = self.raw_tx()?;
    //     Ok(raw_tx.version())
    // }

    // pub fn gas(&self) -> Result<u64, Error> {
    //     let raw_tx = self.raw_tx()?;
    //     Ok(raw_tx.gas())
    // }

    // pub fn gas_price(&self) -> Result<u64, Error> {
    //     let raw_tx = self.raw_tx()?;
    //     Ok(raw_tx.gas_price())
    // }

    pub fn gas_cost(&self) -> Result<u64, Error> {
        let raw_tx = self.raw_tx()?;
        Ok(raw_tx.gas_cost())
    }

    // pub fn timestamp(&self) -> Result<u64, Error> {
    //     let raw_tx = self.raw_tx()?;
    //     Ok(raw_tx.timestamp())
    // } 

    pub fn total_cost(&self) -> Result<u64, Error> {
        let raw_tx = self.raw_tx()?;
        Ok(raw_tx.total_cost())
    }

    pub fn raw_tx_digest(&self) -> Result<Hash, Error> {
        let raw_tx = self.raw_tx()?;
        Ok(raw_tx.id())
    }

    pub fn raw_tx(&self) -> Result<&Tx, Error> {
        match &self.tx {
            Some(tx) => Ok(tx),
            None => Err(Error::EmptyRawTx)
        }
    }
}

impl TryFrom<Vec<u8>> for SignedTx {
    type Error = Error;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self::decode(value.as_slice())?)
    }
}

impl From<SignedTx> for Vec<u8> {
    fn from(value: SignedTx) -> Self {
        value.encode_to_vec()
    }
} 

impl fmt::Display for SignedTx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SignedTx: {{ Raw TX: {:?}, \nSignature: {:?}}}", self.tx, self.signature)
    }
}

impl Tx {
    pub fn new(
        sender: Address, 
        receiver: Address, 
        amount: u64, 
        version: u64, 
        //gas: u64, 
        //gas_price: u64,
        //timestamp: u64,
    ) -> Self {
        Tx { 
            sender: sender.into(), 
            receiver: receiver.into(), 
            amount, 
            version, 
            gas: GAS,
            gas_price: GAS_PRICE,
            timestamp: utils::unix_timestamp(), 
        }
    }

    // pub fn receiver(&self) -> Vec<u8> {
    //     self.receiver.clone()
    // }

    // pub fn sender(&self) -> Vec<u8> {
    //     self.sender.clone()
    // }

    // pub fn amount(&self) -> u64 {
    //     self.amount
    // }

    // pub fn version(&self) -> u64 {
    //     self.amount
    // }

    // pub fn gas(&self) -> u64 {
    //     self.gas
    // }

    // pub fn gas_price(&self) -> u64 {
    //     self.gas_price
    // }

    // pub fn timestamp(&self) -> u64 {
    //     self.timestamp
    // }

    pub fn gas_cost(&self) -> u64 {
        self.gas * self.gas_price
    }

    pub fn total_cost(&self) -> u64 {
        self.amount + self.gas_cost()
    }

    pub fn id(&self) -> Hash {
        utils::hash(&self.as_bytes())
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.encode_to_vec()
    }
}

impl TryFrom<Vec<u8>> for Tx {
    type Error = Error;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self::decode(value.as_slice())?)
    }
}

impl From<Tx> for Vec<u8> {
    fn from(value: Tx) -> Self {
        value.encode_to_vec()
    }
}

mod test {
    #[test]
    fn tx_format_test() {
        use crate::types::Bytes;
        use super::*;
        
        let sender = Bytes::<32>::try_from(
            "0x000036755a024ef491b6710fe765e06e33a616f83b8a33c6a1963ab20f6e5bdb".to_string()
        ).unwrap();
        let receiver = Bytes::<32>::try_from(
            "0x000036755a024ef491b6710fe765e06e33a616f83b8a33c6a1963ab20f6e5bef".to_string()
        ).unwrap();
        // let signature = Bytes::<65>::try_from(
        //     "0x000036755a024ef491b6710fe765e06e33a616f83b8a33c6a1963ab20f6e5bdb".to_string() + 
        //     "000036755a024ef491b6710fe765e06e33a616f83b8a33c6a1963ab20f6e5befff"
        // ).unwrap();

        let raw_tx = Tx::new(sender, receiver, 100, 1);
        //let signed_tx = SignedTx::new(Some(raw_tx), signature);

        //assert_eq!(signed_tx.amount().unwrap(), 100);
        assert_eq!(raw_tx.amount, 100);
        assert_eq!(raw_tx.gas_cost(), 21);
        assert_eq!(raw_tx.total_cost(), 121);
    }
}