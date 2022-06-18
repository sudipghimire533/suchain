use serde::Serialize;
use sha3::Digest;
use crate::components::SuHasher;
use crate::components::SU_HASHER_LEN;

#[derive(Clone, PartialEq, Eq, Serialize, Hash)]
pub struct Hash([u8; SU_HASHER_LEN]);

impl core::fmt::Debug for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hex_rep = hex::encode(&self.0);
        write!(f, "0x{hex_rep}")
    }
}

impl Hash {
    pub fn raw(hash_value: [u8; SU_HASHER_LEN]) -> Self {
        Hash(hash_value)
    }

    pub fn new(message: impl AsRef<[u8]>) -> Self {
        let mut hasher = SuHasher::new();
        hasher.update(message);
        let output = hasher.finalize()[..]
            .to_vec()
            .try_into()
            .expect(&format!("SuHasher's output must be reprsentable as [u8; {SU_HASHER_LEN}]"));

        Hash(output)
    }

    pub fn difficulty_verified(&self, difficulty_level: usize) -> bool {
        self.0.starts_with(&vec![0u8; difficulty_level])
    }
}
