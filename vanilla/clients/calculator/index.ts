import { AccountMeta } from "npm:@solana/web3.js";
import { AvailableProgram } from "../../shared/enums/index.ts";
import {
  configureClientAccount,
  connect,
  getAccountInfo,
  getLocalAccount,
  loadProgram,
  runProgram,
} from "../../shared/functions/index.ts";
import { Operation } from "./enums/index.ts";
import {
  CalculatorInstructionSchema,
  CalculatorSchema,
} from "./models/index.ts";
import { SerializationHelper } from "../../shared/models.ts";

export const run = async () => {
  const connection = connect();
  const localAccount = await getLocalAccount();
  const program = await loadProgram(AvailableProgram.Calculator);
  const calculatorAccountSize = SerializationHelper.getDataSize(
    new CalculatorSchema(),
  );
  const { publicKey: clientPublicKey } = await configureClientAccount(
    connection,
    localAccount,
    program.publicKey,
    calculatorAccountSize,
  );

  const ix = new CalculatorInstructionSchema(Operation.Subtract, 3);
  const ixSerializedData = SerializationHelper.serialize(ix);
  console.log(
    `Dispatching instruction ${Deno.inspect({
      ix,
      ixSerializedData,
    })}...`,
  );
  const ixAccounts: AccountMeta[] = [
    { pubkey: clientPublicKey, isSigner: false, isWritable: true },
  ];
  await runProgram(
    connection,
    localAccount,
    program.publicKey,
    ixAccounts,
    ixSerializedData,
  );
  const updatedAccountInfo = await getAccountInfo(connection, clientPublicKey);
  if (!updatedAccountInfo) {
    throw new Error(`UpdatedAccountInfo is missing!`);
  }
  console.log(
    Deno.inspect({
      decodedData: SerializationHelper.deserialize(
        updatedAccountInfo.data,
        CalculatorSchema,
      ),
    }),
  );
};
