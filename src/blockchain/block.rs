use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::digest::Output;
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize)]
pub struct Block {
    index: usize,
    timestamp: i64,
    proof: i64,
    previous_hash: String,
}

impl Block {
    pub fn new(index: usize, proof: i64, previous_hash: String) -> Self {
        // We use chrono instead of std::time
        // timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
        // because we want to get the current UTC timestamp
        Self {
            index,
            timestamp: Utc::now().timestamp(),
            proof,
            previous_hash,
        }
    }

    pub fn get_previous_hash(&self) -> &str {
        self.previous_hash.as_str()
    }

    pub fn get_proof(&self) -> i64 {
        self.proof
    }

    pub fn get_hash(&self) -> Output<Sha256> {
        let mut hasher = Sha256::new();
        hasher.update(serde_json::to_string(self).unwrap());
        hasher.finalize()
    }
}
