import { LAMPORTS_PER_SOL } from "npm:@solana/web3.js";
import { AvailableProgram } from "../../shared/enums/index.ts";
import {
  configureNewWallet,
  connect,
  loadProgram,
} from "../../shared/functions/index.ts";
import { lamportsToSol, sendLamports } from "./functions/index.ts";

export const run = async () => {
  const connection = connect();
  const program = await loadProgram(AvailableProgram.TransferSol);

  const donRamonKeypair = await configureNewWallet("don ramon", connection);
  const drChatubaKeypair = await configureNewWallet("dr. chatuba");
  const gloriaKeypair = await configureNewWallet("gloria", connection);
  const donSantiagoKeypair = await configureNewWallet("don santiago");
  const donRamonInitialBalance =
    (await connection.getAccountInfo(donRamonKeypair.publicKey))?.lamports || 0;
  const drChatubaInitialBalance =
    (await connection.getAccountInfo(drChatubaKeypair.publicKey))?.lamports ||
    0;
  const gloriaInitialBalance =
    (await connection.getAccountInfo(gloriaKeypair.publicKey))?.lamports || 0;
  const donSantiagoInitialBalance =
    (await connection.getAccountInfo(donSantiagoKeypair.publicKey))?.lamports ||
    0;

  console.log(
    `Wallets initial state: ${
      Deno.inspect({
        donRamon: {
          address: donRamonKeypair.publicKey.toBase58(),
          initialBalance: lamportsToSol(donRamonInitialBalance),
        },
        drChatuba: {
          address: drChatubaKeypair.publicKey.toBase58(),
          initialBalance: lamportsToSol(drChatubaInitialBalance),
        },
        gloria: {
          address: gloriaKeypair.publicKey.toBase58(),
          initialBalance: lamportsToSol(gloriaInitialBalance),
        },
        donSantiagoKeypair: {
          address: donSantiagoKeypair.publicKey.toBase58(),
          initialBalance: lamportsToSol(donSantiagoInitialBalance),
        },
      })
    }`,
  );
  console.log("Don Ramon sends 0.5 SOL to Gloria...");
  const result0 = await sendLamports(
    connection,
    program.publicKey,
    donRamonKeypair,
    gloriaKeypair.publicKey,
    BigInt(LAMPORTS_PER_SOL * 0.5),
  );
  console.log(`First transfer signature ${result0}.`);
  console.log("Gloria sends Don Santiago 0.3 SOL...");
  const result1 = await sendLamports(
    connection,
    program.publicKey,
    gloriaKeypair,
    donSantiagoKeypair.publicKey,
    BigInt(LAMPORTS_PER_SOL * 0.3),
  );
  console.log(`Second transfer signature ${result1}.`);
  console.log("Don Ramon sends Dr. Chatuba 1 SOL...");
  const result2 = await sendLamports(
    connection,
    program.publicKey,
    donRamonKeypair,
    drChatubaKeypair.publicKey,
    BigInt(LAMPORTS_PER_SOL),
  );
  console.log(`Third transfer signature ${result2}.`);
  console.log("Dr. Chatuba sends Gloria 0.7 SOL...");
  const result3 = await sendLamports(
    connection,
    program.publicKey,
    drChatubaKeypair,
    gloriaKeypair.publicKey,
    BigInt(LAMPORTS_PER_SOL * 0.7),
  );
  console.log(`Fourth transfer signature ${result3}.`);
  console.log("Gloria sends Don Santiago 0.5 SOL...");
  const result4 = await sendLamports(
    connection,
    program.publicKey,
    gloriaKeypair,
    donSantiagoKeypair.publicKey,
    BigInt(LAMPORTS_PER_SOL * 0.5),
  );
  console.log(`Fifth transfer signature ${result4}.`);

  const donRamonFinalBalance =
    (await connection.getAccountInfo(donRamonKeypair.publicKey))?.lamports || 0;
  const drChatubaFinalBalance =
    (await connection.getAccountInfo(drChatubaKeypair.publicKey))?.lamports ||
    0;
  const gloriaFinalBalance =
    (await connection.getAccountInfo(gloriaKeypair.publicKey))?.lamports || 0;
  const donSantiagoFinalBalance =
    (await connection.getAccountInfo(donSantiagoKeypair.publicKey))?.lamports ||
    0;

  console.log(
    `Wallets final state: ${
      Deno.inspect({
        donRamon: {
          finalBalance: lamportsToSol(donRamonFinalBalance),
          diff: lamportsToSol(donRamonFinalBalance - donRamonInitialBalance),
        },
        drChatuba: {
          finalBalance: lamportsToSol(drChatubaFinalBalance),
          diff: lamportsToSol(drChatubaFinalBalance - drChatubaInitialBalance),
        },
        gloria: {
          finalBalance: lamportsToSol(gloriaFinalBalance),
          diff: lamportsToSol(gloriaFinalBalance - gloriaInitialBalance),
        },
        donSantiagoKeypair: {
          finalBalance: lamportsToSol(donSantiagoFinalBalance),
          diff: lamportsToSol(
            donSantiagoFinalBalance - donSantiagoInitialBalance,
          ),
        },
      })
    }`,
  );
};
