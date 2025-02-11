use std::ops::Deref;

use k256::ecdsa;

use crate::error::WalletError;

const SIGNATURE_LENGTH: usize = 65;

#[derive(Debug, Clone, Copy)]
pub struct Signature([u8; SIGNATURE_LENGTH]);

impl From<(ecdsa::Signature, ecdsa::RecoveryId)> for Signature {
    fn from((sig, rec_id): (ecdsa::Signature, ecdsa::RecoveryId)) -> Self {
        let mut buffer = [0u8; SIGNATURE_LENGTH];
        buffer[..64].copy_from_slice(sig.to_bytes().as_ref());
        buffer[64] = rec_id.to_byte();
        Signature(buffer)
    }
}

impl TryFrom<Signature> for (ecdsa::Signature, ecdsa::RecoveryId) {
    type Error = WalletError;
    fn try_from(value: Signature) -> Result<Self, Self::Error> {
        let sig = ecdsa::Signature::from_bytes(value[..64].as_ref().into())
            .map_err(|_|WalletError::InvalidSignature)?;

        let recid = ecdsa::RecoveryId::from_byte(value[64])
            .ok_or(WalletError::InvalidSignature)?;

        Ok((sig, recid))
    }
}

impl Deref for Signature {
    type Target = [u8; SIGNATURE_LENGTH];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<[u8; SIGNATURE_LENGTH]> for Signature {
    fn from(value: [u8; SIGNATURE_LENGTH]) -> Self {
        Signature(value)
    }
}

impl From<Signature> for [u8; SIGNATURE_LENGTH] {
    fn from(value: Signature) -> Self {
        value.0
    }
}