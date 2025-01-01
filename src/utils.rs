use std::time::SystemTime;
use rand::distributions::{Distribution, Standard};
use rand::random;
use tiny_keccak::{Hasher, Sha3};

use crate::types::Hash;

pub fn gen_random_number<T>() -> T
where Standard: Distribution<T>
{
    random::<T>()
}

pub fn unix_timestamp() -> u64 {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
}

pub fn hash(data: &[u8]) -> Hash {
    let mut sha3 = Sha3::v256();
    let mut output = [0u8; 32];
    sha3.update(data);
    sha3.finalize(&mut output);
    output.into()
}