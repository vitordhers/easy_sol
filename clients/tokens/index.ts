import {
  AccountMeta,
  Keypair,
  PublicKey,
  sendAndConfirmTransaction,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
  Transaction,
  TransactionInstruction,
} from "npm:@solana/web3.js";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddress,
  TOKEN_PROGRAM_ID,
} from "npm:@solana/spl-token";
import { AvailableProgram } from "../../shared/enums/index.ts";
import {
  connect,
  findProgramAddress,
  getLocalAccount,
  loadProgram,
} from "../../shared/functions/index.ts";
import {
  FungibleAssetData,
  FungibleAssetMetadata,
  FungibleTokenData,
  FungibleTokenMetadata,
  NonFungibleTokenData,
  NonFungibleTokenMetadata,
  TokenData,
} from "./models/index.ts";
import { SerializationHelper } from "../../shared/models.ts";
import { Buffer } from "node:buffer";
import { TOKEN_METADATA_PROGRAM_ID } from "../../shared/constants/index.ts";
export const run = async () => {
  const connection = connect();

  const wallet = await getLocalAccount();
  const programKeypair = await loadProgram(AvailableProgram.Tokens);
  const fungibleTokenMetadata = new FungibleTokenMetadata(
    "Jogo do Bicho Coin",
    "JBC",
    "https://gateway.pinata.cloud/ipfs/bafkreiavttmvulnb2cagvpb4iwyeoeetohvq5bbqeqw4kvedbfb25wha5e",
  );
  const fungibleAssetMetadata = new FungibleAssetMetadata(
    "Food",
    "Food",
    "https://gateway.pinata.cloud/ipfs/bafkreicrtnhb7ec6b2glosuclyqm3yvlhonqhvew763vl7mo5ktd3m7erq",
    1000n,
  );
  const nonFungibleTokenMetadata = new NonFungibleTokenMetadata(
    "Ferris, the Memory Guardian",
    "Dts#001",
    "https://gateway.pinata.cloud/ipfs/bafkreiewvggcg23sci5jq3qqruhlcwmyunwd557ev2yuc4d65eyrvlzfre",
    500,
  );
  const tokensData: TokenData[] = [
    new FungibleTokenData(9, 1000000000n, true, fungibleTokenMetadata),
    new FungibleAssetData(0, 1000n, fungibleAssetMetadata),
    new NonFungibleTokenData(nonFungibleTokenMetadata),
  ];

  const metadataProgramPublicKey = new PublicKey(TOKEN_METADATA_PROGRAM_ID);
  const baseProgramsAccountInfos = [
    {
      // Rent account
      pubkey: SYSVAR_RENT_PUBKEY,
      isSigner: false,
      isWritable: false,
    },
    {
      // System program
      pubkey: SystemProgram.programId,
      isSigner: false,
      isWritable: false,
    },
    {
      // Token program
      pubkey: TOKEN_PROGRAM_ID,
      isSigner: false,
      isWritable: false,
    },
    {
      // Associated token program
      pubkey: ASSOCIATED_TOKEN_PROGRAM_ID,
      isSigner: false,
      isWritable: false,
    },
    {
      // Metadata program
      pubkey: metadataProgramPublicKey,
      isSigner: false,
      isWritable: false,
    },
  ];

  for (const tokenData of tokensData) {
    const mintKeypair = Keypair.generate();
    const tokenAddress = await getAssociatedTokenAddress(
      mintKeypair.publicKey,
      wallet.publicKey,
    );

    console.log(
      `Minting token with data ${Deno.inspect({ data: tokenData })} at ${
        Deno.inspect(
          {
            mintAddress: mintKeypair.publicKey.toBase58(),
            tokenAddress: tokenAddress,
          },
        )
      } ...`,
    );

    const metadataAccounts: AccountMeta[] = [
      {
        pubkey: findProgramAddress(
          [
            Buffer.from("metadata"),
            TOKEN_METADATA_PROGRAM_ID.toBuffer(),
            mintKeypair.publicKey.toBuffer(),
          ],
          TOKEN_METADATA_PROGRAM_ID,
        ).pda,
        isSigner: false,
        isWritable: true,
      },
    ];

    if (tokenData instanceof NonFungibleTokenData) {
      const masterEditionAccountData: AccountMeta = {
        pubkey: findProgramAddress(
          [
            Buffer.from("metadata"),
            TOKEN_METADATA_PROGRAM_ID.toBuffer(),
            mintKeypair.publicKey.toBuffer(),
            Buffer.from("edition"),
          ],
          TOKEN_METADATA_PROGRAM_ID,
        ).pda,
        isSigner: false,
        isWritable: true,
      };
      metadataAccounts.push(masterEditionAccountData);
    }

    const accountsMeta: AccountMeta[] = [
      {
        pubkey: mintKeypair.publicKey,
        isSigner: true,
        isWritable: true,
      },
      { pubkey: tokenAddress, isSigner: false, isWritable: true },
      {
        // Mint authority
        pubkey: wallet.publicKey,
        isSigner: true,
        isWritable: false,
      },
      ...metadataAccounts,
      ...baseProgramsAccountInfos,
    ];
    const serializedIxData = SerializationHelper.serialize(tokenData);
    const ix = new TransactionInstruction({
      keys: accountsMeta,
      programId: programKeypair.publicKey,
      data: serializedIxData,
    });
    const signature = await sendAndConfirmTransaction(
      connection,
      new Transaction().add(ix),
      [wallet, mintKeypair],
    );

    console.log("Minted successfully! Signature:", signature);
  }
};
