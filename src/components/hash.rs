use sha3::Digest;
use crate::components::SuHasher;
use crate::components::SU_HASHER_LEN;

use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
struct PrettyHash(String);
impl From<Hash> for PrettyHash {
     fn from(src: Hash) -> Self {
        PrettyHash(
            format!("0x{}", hex::encode(src.0.to_vec()))
        )
     }
}

impl From<PrettyHash> for Hash {
    fn from(src: PrettyHash) -> Self {
        let hash_string = {
            if src.0.starts_with("0x") {
                &src.0[2..]
            } else {
                &src.0[..]
            }
        };

        let bytes = hex::decode(hash_string).expect("Invalid hex value");
        Hash(bytes.try_into().expect("Unexpected hash length"))
    }
}

#[derive(Clone, PartialEq, Eq, Serialize, Hash, Deserialize)]
#[serde(into = "PrettyHash")]
#[serde(from = "PrettyHash")]
pub struct Hash([u8; SU_HASHER_LEN]);

impl core::fmt::Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let to_string = serde_json::to_string_pretty(self)
            .map_err(|_e| std::fmt::Error)?;
        write!(f, "{to_string}")
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
