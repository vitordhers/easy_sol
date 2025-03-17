import { AccountMeta } from "npm:@solana/web3.js";
import { AvailableProgram } from "../../shared/enums/index.ts";
import {
  configureClientAccount,
  connect,
  createBufferInstruction,
  getAccountInfo,
  getLocalAccount,
  loadProgram,
  runProgram,
} from "../../shared/functions/index.ts";
import { CALCULATOR_SIZE } from "./constants/index.ts";
import { Operation } from "./enums/index.ts";
import { CalculatorInstruction, CalculatorSchema } from "./models/index.ts";
import { serialize } from "node:v8";

export const run = async () => {
  const connection = await connect();
  const localAccount = await getLocalAccount();
  const program = await loadProgram(AvailableProgram.Calculator);
  const { publicKey: clientPublicKey, account } = await configureClientAccount(
    connection,
    localAccount,
    program.publicKey,
    CALCULATOR_SIZE,
  );

  const ix = new CalculatorInstruction(Operation.Subtract, 3);

  const ixFields = CalculatorInstruction.getFields();
  const ixData = createBufferInstruction(ixFields, ix);
  console.log(
    `Dispatching instruction ${
      Deno.inspect({
        ix,
        serialized: ix.serialize(),
        ixData,
      })
    }...`,
  );
  const ixAccounts: AccountMeta[] = [
    { pubkey: clientPublicKey, isSigner: false, isWritable: true },
  ];
  await runProgram(
    connection,
    localAccount,
    program.publicKey,
    ixAccounts,
    ix.serialize(),
  );
  const updatedAccountInfo = await getAccountInfo(connection, clientPublicKey);
  if (!updatedAccountInfo) {
    throw new Error(`UpdatedAccountInfo is missing!`);
  }
  console.log(
    Deno.inspect({
      decodedData: CalculatorSchema.deserialize(updatedAccountInfo.data),
    }),
  );
};
