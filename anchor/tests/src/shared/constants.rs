use spl_token::solana_program::pubkey::Pubkey;
use std::{str::FromStr, sync::LazyLock};

pub const METADATA_PROGRAM_ADDR: &str = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s";
pub static METADATA_PROGRAM_ID: LazyLock<Pubkey> =
    LazyLock::new(|| Pubkey::from_str(METADATA_PROGRAM_ADDR).unwrap());
pub const METADATA_ACCOUNT_SEED_PREFIX: &[u8] = b"metadata";
pub const MASTER_EDITION_ACCOUNT_SEED_SUFFIX: &[u8] = b"edition";
