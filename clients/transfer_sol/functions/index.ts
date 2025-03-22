import { Buffer } from "node:buffer";
import {
  AccountMeta,
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  sendAndConfirmTransaction,
  SystemProgram,
  Transaction,
  TransactionInstruction,
} from "npm:@solana/web3.js";
import { TransferSchema } from "../models/index.ts";
import { SerializationHelper } from "../../../shared/models.ts";

export const sendLamports = async (
  connection: Connection,
  programId: PublicKey,
  from: Keypair,
  to: PublicKey,
  amountInLamports: bigint,
) => {
  const ixData = new TransferSchema(amountInLamports);
  const ixSerializedData = SerializationHelper.serialize(ixData);
  const accountsMetadata: AccountMeta[] = [
    { pubkey: from.publicKey, isSigner: true, isWritable: true },
    { pubkey: to, isSigner: false, isWritable: true },
    { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
  ];
  const ix = new TransactionInstruction({
    keys: accountsMetadata,
    programId,
    data: ixSerializedData,
  });
  return await sendAndConfirmTransaction(
    connection,
    new Transaction().add(ix),
    [from],
  );
};

export const lamportsToSol = (lamports: number) =>
  (lamports / LAMPORTS_PER_SOL).toFixed(4);
