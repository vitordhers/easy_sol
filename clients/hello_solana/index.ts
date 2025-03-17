import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  sendAndConfirmTransaction,
  Transaction,
  TransactionInstruction,
} from "npm:@solana/web3.js";
import { Buffer } from "node:buffer";

const LOCAL_RPC_URL = "http://127.0.0.1:8899";

export const run = async () => {
  try {
    console.log("Checking solana_program artifacts...", Deno.cwd());
    const keypairFileInfo = await Deno.stat(
      "contracts/target/deploy/hello_solana-keypair.json",
    );
    if (!keypairFileInfo.isFile) {
      throw new Error(`hello_solana-keypair.json file is missing!`);
    }
    console.log("Setting up connection...");
    const connection = new Connection(LOCAL_RPC_URL, "confirmed");

    console.log(`Loading program secret key string...`);

    const secretKeyStringData = await Deno.readFile(
      "contracts/target/deploy/hello_solana-keypair.json",
    );

    const secretKeyString = new TextDecoder("utf-8").decode(
      secretKeyStringData,
    );

    const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
    const programKeypair = Keypair.fromSecretKey(secretKey);
    const programId = programKeypair.publicKey;
    console.log(`Program keypair loaded! Program id: ${programId.toBase58()}`);
    console.log(`Generating a new keypair for calling the program...`);
    const callerKeypair = Keypair.generate();
    console.log(`Keypair generated! Pubkey ${callerKeypair.publicKey}`);
    console.log(`Requesting airdrop for new keyipair...`);
    const airdropRequest = await connection.requestAirdrop(
      callerKeypair.publicKey,
      LAMPORTS_PER_SOL,
    );
    const airdorpResult = await connection.confirmTransaction(
      airdropRequest,
      "finalized",
    );
    console.log(`Airdrop result: ${Deno.inspect({ airdorpResult })}`);
    console.log(`Pinging program: ${programId.toBase58()} ...`);
    const ix = new TransactionInstruction({
      keys: [
        { pubkey: callerKeypair.publicKey, isSigner: false, isWritable: true },
      ],
      programId,
      data: Buffer.alloc(0),
    });

    const result = await sendAndConfirmTransaction(
      connection,
      new Transaction().add(ix),
      [callerKeypair],
    );
    console.log(`Transaction result ${Deno.inspect({ result })}`);
  } catch (error) {
    console.error(`Error: ${Deno.inspect({ error })}`);
    throw error;
  }
};
