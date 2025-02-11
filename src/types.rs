use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;
use serde::{Deserialize, Serialize};

use crate::error::Error;


const HASH_LENGTH: usize = 32;

const SIGNATURE_LENGTH: usize = 65;

const ADDRESS_LENGTH: usize = 32;

pub type Hash = Bytes<HASH_LENGTH>;

pub type Signature = Bytes<SIGNATURE_LENGTH>;

pub type Address = Bytes<ADDRESS_LENGTH>;

#[derive(Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct Bytes<const T: usize> (pub [u8; T]);

impl <const T: usize>Bytes<T> {
    pub fn is_empty(&self) -> bool {
        self.0.iter().all(|&c| c == 0)
    }

    pub fn fmt_as_hex(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "0x")?;
        for byte in self.0.iter() {
            write!(f, "{:02x}", byte)?;
        };
        Ok(())
    }

    #[cfg(test)]
    pub fn new_for_test() -> Self {
        use crate::utils;
        let mut inner = [0u8; T];
        for i in 0..T {
            let hex_num = utils::gen_random_number::<u8>();
            inner[i] = hex_num;
        };
        Bytes(inner)
    }
}

impl <const T: usize>Deref for Bytes<T> {
    type Target = [u8; T];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl <const T: usize>From<Vec<u8>> for Bytes<T> {
    fn from(value: Vec<u8>) -> Self {
        let mut inner = [0u8; T];
        inner.copy_from_slice(value.as_slice());
        Self(inner)

    }
}

impl <const T: usize>From<[u8; T]> for Bytes<T> {
    fn from(value: [u8; T]) -> Self {
        Self(value)
    }
}

impl <const T: usize>TryFrom<String> for Bytes<T> {
    type Error = Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = if let Some(value) = value.strip_prefix("0x") {
            value
        }else {
            &value
        };
        Ok(hex::decode(value).map(Self::from)?)
    }
}

impl <const T: usize>From<Bytes<T>> for Vec<u8> {
    fn from(value: Bytes<T>) -> Self {
        value.0.to_vec()
    }
}

impl <const T: usize>From<Bytes<T>> for [u8; T] {
    fn from(value: Bytes<T>) -> Self {
        value.0
    }
}

impl <const T: usize>From<Bytes<T>> for String {
    fn from(value: Bytes<T>) -> Self {
        String::from("0x") + &hex::encode(value.0)
    }
}

impl <const T: usize>Default for Bytes<T> {
    fn default() -> Self {
        Bytes([0u8; T])
    }
}

impl <const T: usize>Debug for Bytes<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_as_hex(f)
    }
}

impl <const T: usize>Display for Bytes<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_as_hex(f)
    }
}

impl From<wallet::signature::Signature> for Signature {
    fn from(value: wallet::signature::Signature) -> Self {
        Self(value.into())
    }
}

impl From<Signature> for wallet::signature::Signature {
    fn from(value: Signature) -> Self {
        Self::from(value.0)
    }
}

#[cfg(test)]
mod test{

    use super::*;

    #[test]
    fn default_value_test() {
        let default_value = Bytes::<32>::default();
        assert!(default_value.is_empty())
    }

    #[test]
    fn bytes_from_test() {
        let s = "0x000036755a024ef491b6710fe765e06e33a616f83b8a33c6a1963ab20f6e5bdb";
        let v = hex::decode(&s[2..]).unwrap();
        let arr: [u8; 32] = v.as_slice().try_into().unwrap();

        let b1 = Bytes::<32>::from(arr);
        assert_eq!(b1[..], arr[..]);

        let b2 = Bytes::<32>::from(v);
        assert_eq!(b1, b2);

        let b3 = Bytes::<32>::try_from(s.to_string()).unwrap();
        assert_eq!(b1, b3);
    }

    #[test]
    fn bytes_into_test() {
        let s1 = "0x000036755a024ef491b6710fe765e06e33a616f83b8a33c6a1963ab20f6e5bdb";
        let b1: Bytes<32> = Bytes::<32>::try_from(s1.to_string()).unwrap();

        let s2: String  = b1.into();
        assert_eq!(s1, &s2);

        let v: Vec<u8> = b1.into();
        assert_eq!(v.as_slice(), b1.as_slice());

        let arr: [u8; 32] = b1.into();
        assert_eq!(arr, b1.0);
    }

    #[test]
    fn bytes_logging_format_test() {
        let s = "0x000036755a024ef491b6710fe765e06e33a616f83b8a33c6a1963ab20f6e5bdb";
        let b: Bytes<32> = Bytes::<32>::try_from(s.to_string()).unwrap();

        assert_eq!(s, format!("{}", b));
        assert_eq!(s, format!("{:?}", b));
    }
}