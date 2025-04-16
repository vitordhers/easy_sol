use std::str::FromStr;

use crate::shared::constants::{METADATA_ACCOUNT_SEED_PREFIX, METADATA_PROGRAM_ID};
use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair},
        signer::Signer,
        system_program::ID as SYSTEM_PROGRAM_ID,
        sysvar::rent::ID as RENT_ID,
    },
    Client, Cluster,
};
use spl_associated_token_account::{
    get_associated_token_address, ID as ASSOCIATED_TOKEN_PROGRAM_ID,
};
use spl_token::ID as TOKEN_PROGRAM_ID;
use tokens::data::FungibleTokenParams;

#[test]
fn test_fungible_token_mint() {
    let program_id = "GCrqR8NeXX7j4ho8nPoJxojSS7deokj7W5mNBEdGSaqC";
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = read_keypair_file(&anchor_wallet).unwrap();
    let wallet_pubkey = payer.pubkey();
    let client = Client::new_with_options(Cluster::Localnet, &payer, CommitmentConfig::confirmed());
    let program_id = Pubkey::from_str(program_id).unwrap();
    let program = client.program(program_id).unwrap();

    let mint_keypair = Keypair::new();

    let token_pubkey = get_associated_token_address(&wallet_pubkey, &mint_keypair.pubkey());
    let metdata_program_pubkey = Pubkey::from_str_const(METADATA_PROGRAM_ID);
    let (metadata_pda, _bump) = Pubkey::find_program_address(
        &[
            METADATA_ACCOUNT_SEED_PREFIX.as_bytes(),
            METADATA_PROGRAM_ID.as_bytes(),
            &mint_keypair.pubkey().to_bytes(),
        ],
        &metdata_program_pubkey,
    );
    let accounts = tokens::accounts::MintFungible {
        mint: mint_keypair.pubkey(),
        token: token_pubkey,
        mint_authority: payer.pubkey(),
        metadata: metadata_pda,
        metadata_program: metdata_program_pubkey,
        system_program: SYSTEM_PROGRAM_ID,
        rent: RENT_ID,
        token_program: TOKEN_PROGRAM_ID,
        associated_token_program: ASSOCIATED_TOKEN_PROGRAM_ID,
    };
    let token_metadata = tokens::data::FungibleTokenMetadata { uri };
    let params = FungibleTokenParams {
        metadata: token_metadata,
        decimals: 9,
        initial_supply: 100_000,
        should_freeze_after_mint: true,
    };
    let tx_data = tokens::data::TokenData::Fungible(params);
    let ix_data = tokens::instruction::MintFungibleToken { data: tx_data };
    let tx = program
        .request()
        .accounts(accounts)
        .args(ix_data)
        .send()
        .expect("");

    println!("Your transaction signature {}", tx);
}

#[test]
fn test_fungible_asset_mint() {}

#[test]
fn test_non_fungible_token_mint() {}
