use crate::shared::constants::{METADATA_ACCOUNT_SEED_PREFIX, METADATA_PROGRAM_ID};
use anchor_client::{
    anchor_lang::AccountDeserialize,
    solana_sdk::{
        account::ReadableAccount,
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair},
        signer::Signer,
        system_program::ID as SYSTEM_PROGRAM_ID,
        sysvar::rent::ID as RENT_ID,
    },
    Client, Cluster, Program,
};
use anchor_spl::{
    associated_token::{get_associated_token_address, ID as ASSOCIATED_TOKEN_PROGRAM_ID},
    metadata::MetadataAccount,
    token::{Mint, TokenAccount},
};
use spl_token::ID as TOKEN_PROGRAM_ID;
use std::{str::FromStr, sync::LazyLock};
use tokens::data::FungibleTokenParams;

const PROGRAM_ID: &str = "GCrqR8NeXX7j4ho8nPoJxojSS7deokj7W5mNBEdGSaqC";
static PAYER: LazyLock<Keypair> = LazyLock::new(|| {
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    read_keypair_file(&anchor_wallet).unwrap()
});
static PROGRAM: LazyLock<Program<&Keypair>> = LazyLock::new(|| -> Program<&Keypair> {
    let client =
        Client::new_with_options(Cluster::Localnet, &*PAYER, CommitmentConfig::confirmed());
    let program_id = Pubkey::from_str(PROGRAM_ID).unwrap();
    client.program(program_id).unwrap()
});

#[tokio::test]
async fn test_fungible_token_mint() {
    let mint_keypair = Keypair::new();
    let mint_pubkey = mint_keypair.pubkey();
    let token_pubkey = get_associated_token_address(&PAYER.pubkey(), &mint_keypair.pubkey());
    let (metadata_pda, _bump) = Pubkey::find_program_address(
        &[
            METADATA_ACCOUNT_SEED_PREFIX,
            METADATA_PROGRAM_ID.as_ref(),
            mint_keypair.pubkey().as_ref(),
        ],
        &METADATA_PROGRAM_ID,
    );
    let accounts = tokens::accounts::MintFungible {
        mint: mint_keypair.pubkey(),
        token: token_pubkey,
        mint_authority: PAYER.pubkey(),
        metadata: metadata_pda,
        metadata_program: *METADATA_PROGRAM_ID,
        system_program: SYSTEM_PROGRAM_ID,
        rent: RENT_ID,
        token_program: TOKEN_PROGRAM_ID,
        associated_token_program: ASSOCIATED_TOKEN_PROGRAM_ID,
    };
    let symbol = String::from("JBC");
    let name = String::from("Jogo do Bicho Coin");
    let uri = String::from("https://gateway.pinata.cloud/ipfs/bafkreicrswd7o45wtlkkvqijr7w7ksvugjjww5ylwopb32wqvs5cihp4lm");
    let token_metadata = tokens::data::FungibleTokenMetadata {
        symbol: symbol.clone(),
        name: name.clone(),
        uri: uri.clone(),
    };
    let token_params = FungibleTokenParams {
        metadata: token_metadata,
        decimals: 9,
        initial_supply: 1_000_000,
        should_freeze_after_mint: true,
    };
    let token_data = tokens::data::TokenData::Fungible(token_params);
    let ix_data = tokens::instruction::MintFungibleToken { data: token_data };
    let tx = {
        PROGRAM
            .request()
            .accounts(accounts)
            .args(ix_data)
            .signer(&PAYER)
            .signer(mint_keypair)
            .send()
            .await
            .expect("request to be valid")
    };

    println!("Your transaction signature {}", tx);

    let mint_account = PROGRAM
        .rpc()
        .get_account(&mint_pubkey)
        .await
        .expect("Failed to fetch mint account");
    let mint_account = Mint::try_deserialize(&mut mint_account.data())
        .expect("mint data to be deserialized correctly");

    assert_eq!(mint_account.decimals, 9);
    assert_eq!(mint_account.supply, 1_000_000 * 10_u64.pow(9_u32));
    assert_eq!(mint_account.mint_authority.unwrap(), PAYER.pubkey());
    let token_account = PROGRAM
        .rpc()
        .get_account(&token_pubkey)
        .await
        .expect("Failed to fetch mint account");
    let token_account = TokenAccount::try_deserialize(&mut token_account.data())
        .expect("token data to be deserialized correctly");

    assert_eq!(token_account.amount, 1_000_000 * 10_u64.pow(9_u32));
    assert_eq!(token_account.mint, mint_pubkey);
    assert_eq!(token_account.owner, PAYER.pubkey());

    let metadata_account = PROGRAM
        .rpc()
        .get_account(&metadata_pda)
        .await
        .expect("Failed to fetch mint account");
    let metadata_account = MetadataAccount::try_deserialize(&mut metadata_account.data()).unwrap();

    assert_eq!(metadata_account.symbol.trim_matches(char::from(0)), symbol);
    assert_eq!(metadata_account.name.trim_matches(char::from(0)), name);
    assert_eq!(metadata_account.uri.trim_matches(char::from(0)), uri);
}


