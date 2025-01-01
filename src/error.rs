use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to decode hash from hex")]
    InvalidHex(#[from] hex::FromHexError),

    #[error("Empty raw transaction")]
    EmptyRawTx,

    #[error("Empty header")]
    EmptyHeader,

    #[error(transparent)]
    InvalidP2pMessage(#[from] prost::DecodeError),

    #[error("Invalid response")]
    InvalidResponse,
}