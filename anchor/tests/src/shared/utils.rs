use anchor_client::solana_sdk::{signature::Keypair, signer::SeedDerivable};
use sha2::{Digest, Sha256};
use std::error;

pub fn seed_from_str(label: &str) -> [u8; 32] {
    let hash = Sha256::digest(label.as_bytes());
    let mut seed = [0u8; 32];
    seed.copy_from_slice(&hash);
    seed
}

pub fn keypair_from_seed(seed: &[u8]) -> Result<Keypair, Box<dyn error::Error>> {
    Keypair::from_seed(seed)
}
