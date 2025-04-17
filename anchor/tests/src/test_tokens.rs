use crate::shared::constants::{METADATA_ACCOUNT_SEED_PREFIX, METADATA_PROGRAM_ID};
use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair},
        signer::{SeedDerivable, Signer},
        system_program::ID as SYSTEM_PROGRAM_ID,
        sysvar::rent::ID as RENT_ID,
    },
    Client, Cluster,
};
use sha2::{Digest, Sha256};
use spl_associated_token_account::{
    get_associated_token_address, ID as ASSOCIATED_TOKEN_PROGRAM_ID,
};
use spl_token::ID as TOKEN_PROGRAM_ID;
use std::str::FromStr;
use tokens::data::FungibleTokenParams;

fn seed_from_str(label: &str) -> [u8; 32] {
    let hash = Sha256::digest(label.as_bytes());
    let mut seed = [0u8; 32];
    seed.copy_from_slice(&hash);
    seed
}

#[test]
fn test_fungible_token_mint() {
    let program_id = "5HYopUu3aUZ3CQC9AMq3iKa5Q9awF6aEm18jvHfKQ45r";
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = read_keypair_file(&anchor_wallet).unwrap();
    let wallet_pubkey = payer.pubkey();
    let client = Client::new_with_options(Cluster::Localnet, &payer, CommitmentConfig::confirmed());
    let program_id = Pubkey::from_str(program_id).unwrap();
    let program = client.program(program_id).unwrap();
    let seed = seed_from_str("test_seed");
    let mint_keypair = Keypair::from_seed(&seed).unwrap();
    let token_pubkey = get_associated_token_address(&wallet_pubkey, &mint_keypair.pubkey());
    let metadata_program_pubkey = Pubkey::from_str(METADATA_PROGRAM_ID).unwrap();
    let (metadata_pda, _bump) = Pubkey::find_program_address(
        &[
            b"metadata",
            metadata_program_pubkey.as_ref(),
            mint_keypair.pubkey().as_ref(),
        ],
        &metadata_program_pubkey,
    );
    let accounts = tokens::accounts::MintFungible {
        mint: mint_keypair.pubkey(),
        token: token_pubkey,
        mint_authority: payer.pubkey(),
        metadata: metadata_pda,
        metadata_program: metadata_program_pubkey,
        system_program: SYSTEM_PROGRAM_ID,
        rent: RENT_ID,
        token_program: TOKEN_PROGRAM_ID,
        associated_token_program: ASSOCIATED_TOKEN_PROGRAM_ID,
    };
    let token_metadata = tokens::data::FungibleTokenMetadata {
            symbol: String::from("JBC"),
            name:  String::from("Jogo do Bicho Coin"),
            uri: String::from("https://gateway.pinata.cloud/ipfs/bafkreicrswd7o45wtlkkvqijr7w7ksvugjjww5ylwopb32wqvs5cihp4lm"),
    };
    let params = FungibleTokenParams {
        metadata: token_metadata,
        decimals: 9,
        initial_supply: 1_000_000,
        should_freeze_after_mint: true,
    };
    let tx_data = tokens::data::TokenData::Fungible(params);
    let ix_data = tokens::instruction::MintFungibleToken { data: tx_data };
    let tx = program
        .request()
        .accounts(accounts)
        .args(ix_data)
        .signer(&payer)
        .signer(&mint_keypair)
        .send()
        .expect("");

    println!("Your transaction signature {}", tx);
}

#[test]
fn test_fungible_asset_mint() {}

#[test]
fn test_non_fungible_token_mint() {}
