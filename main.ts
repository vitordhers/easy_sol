import { AvailableProgram } from "./shared/enums/index.ts";
import { run as runHelloSolana } from "./clients/hello_solana/index.ts";
import { run as runCalculator } from "./clients/calculator/index.ts";
import { run as runTransferSol } from "./clients/transfer_sol/index.ts";
import { run as runTokens } from "./clients/tokens/index.ts";
import { SendTransactionError } from "npm:@solana/web3.js";
import { connect } from "./shared/functions/index.ts";

const main = async () => {
  const args = Deno.args;

  const programIndex = args.indexOf("--program");
  if (programIndex) {
    throw new Error(`Param --program must be passed!`);
  }

  const program = args[programIndex + 1];
  const availablePrograms = Object.values(AvailableProgram);
  if (!program) {
    throw new Error(
      `A program name (${availablePrograms.join(", ")}) must be provided!`,
    );
  }
  if (!availablePrograms.includes(program as AvailableProgram)) {
    throw new Error(
      `Program ${program} is not a valid program! Valid programs are: ${
        availablePrograms.join(
          ", ",
        )
      }`,
    );
  }
  const info = await Deno.stat("contracts/target/deploy");
  if (!info.isDirectory) {
    throw new Error(`Make sure contracts are deployed!`);
  }
  console.log(`Running ${program} ...`);

  switch (program) {
    case AvailableProgram.HelloSolana: {
      return await runHelloSolana();
    }
    case AvailableProgram.Calculator: {
      return await runCalculator();
    }
    case AvailableProgram.TransferSol: {
      return await runTransferSol();
    }
    case AvailableProgram.Tokens: {
      return await runTokens();
    }
    default: {
      throw new Error(
        `Program ${program} is not a valid program! Valid programs are: ${
          availablePrograms.join(
            ", ",
          )
        }`,
      );
    }
  }
};

main().then(
  () => process.exit(),
  async (err) => {
    if (err instanceof SendTransactionError) {
      const connection = connect();
      const logs = await err.getLogs(connection);
      console.error(Deno.inspect({ logs }));
      return;
    }
    console.error(err);
    process.exit(-1);
  },
);
