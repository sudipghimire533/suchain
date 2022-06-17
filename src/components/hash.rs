use serde::Serialize;
use sha3::Digest;
use sha3::Sha3_256;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Hash([u8; 32]);

impl Hash {
    pub fn new(message: impl AsRef<[u8]>) -> Self {
        let mut hasher = Sha3_256::new();
        hasher.update(message);
        let output = hasher.finalize()[..]
            .to_vec()
            .try_into()
            .expect("Sha3_256's output must be reprsentable as [u8; 32]");

        Hash(output)
    }
}
