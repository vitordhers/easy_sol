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
import { join } from "@std/path";
import { parse as parseYaml } from "@std/yaml";
import { Buffer } from "node:buffer";
import { AvailableProgram, SolanaRpcUrl } from "../enums/index.ts";
import { CONFIG_FILE_PATH, PROGRAMS_PATH } from "../constants/index.ts";
import { CliConfig } from "../interfaces/index.ts";

export const { inspect, readFile } = Deno;

export const connect = (url = SolanaRpcUrl.Local) => {
  return new Connection(url, "confirmed");
};

export const createKeypairFromFile = async (path: string) => {
  const secretKey = await Deno.readFile(path);
  const secretKeyUtf8 = new TextDecoder("utf-8").decode(secretKey);
  const secretKeyUintArray = new Uint8Array(JSON.parse(secretKeyUtf8));
  return Keypair.fromSecretKey(secretKeyUintArray);
};

export const getLocalAccount = async (connectionForAirdrop?: Connection) => {
  const yamlConfigUintArray = await Deno.readFile(CONFIG_FILE_PATH);
  const yamlConfig = new TextDecoder("utf-8").decode(yamlConfigUintArray);
  const keypairConfig = (await parseYaml(yamlConfig)) as CliConfig;
  const keypairPath = keypairConfig.keypair_path;
  const localKeypair = await createKeypairFromFile(keypairPath);
  console.log(
    `Local account loaded successfully (${localKeypair.publicKey.toBase58()})`,
  );
  if (connectionForAirdrop) {
    console.log(`Requesting airdrop of 2 SOL to local account...`);
    const airdropRequest = await connectionForAirdrop.requestAirdrop(
      localKeypair.publicKey,
      2 * LAMPORTS_PER_SOL,
    );
    await connectionForAirdrop.confirmTransaction(airdropRequest, "confirmed");
    console.log(`airdrop confirmed successfully!`);
  }
  return localKeypair;
};

export const loadProgram = async (programName: AvailableProgram) => {
  const programKeypairPath = join(PROGRAMS_PATH, `${programName}-keypair.json`);
  const programKeypair = await createKeypairFromFile(programKeypairPath);
  const address = programKeypair.publicKey.toBase58();
  console.log(`Program ${programName} (${address}) loaded successfully!`);
  return { name: programName, publicKey: programKeypair.publicKey, address };
};

export const getAccountInfo = async (
  connection: Connection,
  publicKey: PublicKey,
) => {
  return (await connection.getAccountInfo(publicKey)) || undefined;
};
export const decodeBuffer = (buffer: Buffer, encoding = "utf-8") => {
  return new TextDecoder(encoding).decode(buffer);
};

export const configureClientAccount = async (
  connection: Connection,
  localKeypair: Keypair,
  programPublicKey: PublicKey,
  accountSpaceSize: number,
  seed = "test1",
) => {
  const clientPublicKey = await PublicKey.createWithSeed(
    localKeypair.publicKey,
    seed,
    programPublicKey,
  );

  const clientPublicKeyAddress = clientPublicKey.toBase58();

  console.log(`Generated address: ${clientPublicKeyAddress}`);

  let clientAccount = await getAccountInfo(connection, clientPublicKey);
  if (clientAccount) {
    return { account: clientAccount, publicKey: clientPublicKey };
  }
  console.log(
    `Account for address ${clientPublicKeyAddress} doesn't exist yet, creating it...`,
  );
  const ix = SystemProgram.createAccountWithSeed({
    fromPubkey: localKeypair.publicKey,
    basePubkey: localKeypair.publicKey,
    seed,
    newAccountPubkey: clientPublicKey,
    lamports: LAMPORTS_PER_SOL,
    space: accountSpaceSize,
    programId: programPublicKey,
  });
  const tx = new Transaction().add(ix);
  const result = await sendAndConfirmTransaction(connection, tx, [
    localKeypair,
  ]);
  console.log(`Client account created successfully.`, result);
  clientAccount = (await connection.getAccountInfo(clientPublicKey)) ||
    undefined;
  if (!clientAccount) {
    throw new Error(`Client account is missing`);
  }
  return { account: clientAccount, publicKey: clientPublicKey };
};

//  fields: Layout<T[keyof T]>[],
//  payload: T,
//) => {
//  const layout = struct(fields);
//  const buffer = Buffer.alloc(layout.span);
//  layout.encode(payload, buffer);
//  return buffer;
//};

export const runProgram = async (
  connection: Connection,
  localKeypair: Keypair,
  programId: PublicKey,
  accountsMeta: AccountMeta[],
  ixData?: Uint8Array,
) => {
  if (!ixData) {
    ixData = Buffer.alloc(0);
  }
  const ix = new TransactionInstruction({
    keys: accountsMeta,
    programId,
    data: ixData instanceof Buffer ? ixData : Buffer.from(ixData),
  });

  return await sendAndConfirmTransaction(
    connection,
    new Transaction().add(ix),
    [localKeypair],
  );
};
