use serde::{Deserialize, Serialize};
use sha2::digest::Output;
use sha2::{Digest, Sha256};

use self::block::Block;

pub mod block;

// Easy limit, first two hex digits must be 0
const PROOF_OF_WORK_LIMIT: &[u8] = &[0];

fn is_under_limits(byte_array: Output<sha2::Sha256>) -> bool {
    PROOF_OF_WORK_LIMIT == &byte_array[..1]
}

#[derive(Serialize, Deserialize)]
pub struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut bc = Self { chain: Vec::new() };
        // Genesis block
        bc.create_block(0, String::from("0"));
        bc
    }

    pub fn create_block(&mut self, proof: i64, previous_hash: String) -> &Block {
        let block = Block::new(self.chain.len(), proof, previous_hash);
        self.chain.push(block);
        self.get_last_block()
    }

    pub fn get_last_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    /// Our PoW:
    /// (new_proof ^ 2) - (previous_proof ^ 2)
    /// This result must have two leading zeros
    pub fn proof_of_work(&self) -> i64 {
        let mut proof = 1;
        let previous_proof = self.get_last_block().get_proof();
        loop {
            let hash = self.calculate_proof(proof, previous_proof);
            if is_under_limits(hash) {
                return proof;
            }
            proof += 1;
        }
    }

    fn calculate_proof(&self, new_proof: i64, prev_proof: i64) -> Output<Sha256> {
        let mut hasher = Sha256::new();
        let to_hash = new_proof.saturating_pow(2) - prev_proof.saturating_pow(2);
        hasher.update(to_hash.to_be_bytes());
        hasher.finalize()
    }

    pub fn is_chain_valid(&self) -> bool {
        self.chain.windows(2).all(|blocks| {
            let (first, second) = (blocks.first().unwrap(), blocks.last().unwrap());
            let current_hash = first.get_hash()[..]
                .iter()
                .map(|x| format!("{:02x?}", x))
                .collect::<String>();
            let (prev_proof, proof) = (first.get_proof(), second.get_proof());
            let proof = self.calculate_proof(proof, prev_proof);
            current_hash == second.get_previous_hash() && is_under_limits(proof)
        })
    }
}
