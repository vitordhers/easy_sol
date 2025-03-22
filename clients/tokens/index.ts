import {
  AccountMeta,
  Keypair,
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
  getLocalAccount,
  loadProgram,
} from "../../shared/functions/index.ts";
import {
  FungibleAssetData,
  FungibleTokenData,
  NonFungibleTokenData,
  TokenData,
} from "./models/index.ts";
import { SerializationHelper } from "../../shared/models.ts";

export const run = async () => {
  const connection = connect();

  const wallet = await getLocalAccount();
  const programKeypair = await loadProgram(AvailableProgram.Tokens);

  const tokensData: TokenData[] = [
    new FungibleTokenData(9, 1000000000n, true),
    new FungibleTokenData(9, 1000000000n, false),
    new FungibleAssetData(0, 1000n),
    new NonFungibleTokenData(),
  ];
  const baseAccountsInfos = [
    {
      pubkey: wallet.publicKey,
      isSigner: true,
      isWritable: false,
    }, // Rent account
    {
      pubkey: SYSVAR_RENT_PUBKEY,
      isSigner: false,
      isWritable: false,
    }, // System program
    {
      pubkey: SystemProgram.programId,
      isSigner: false,
      isWritable: false,
    },
    {
      pubkey: TOKEN_PROGRAM_ID,
      isSigner: false,
      isWritable: false,
    }, // Associated token program
    {
      pubkey: ASSOCIATED_TOKEN_PROGRAM_ID,
      isSigner: false,
      isWritable: false,
    },
  ];
  for (const tokenData in tokensData) {
    const mintKeypair = Keypair.generate();
    const tokenAddress = await getAssociatedTokenAddress(
      mintKeypair.publicKey,
      wallet.publicKey,
    );

    console.log(
      `Minting token with data ${Deno.inspect({ data: tokensData })} at ${Deno.inspect(
        {
          mintAddress: mintKeypair.publicKey.toBase58(),
          tokenAddress: tokenAddress,
        },
      )} ...`,
    );
    const accountsMeta: AccountMeta[] = [
      {
        pubkey: mintKeypair.publicKey,
        isSigner: true,
        isWritable: true,
      },
      { pubkey: tokenAddress, isSigner: false, isWritable: true },
      ...baseAccountsInfos,
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
