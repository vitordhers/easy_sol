import { AvailableProgram } from "./shared/enums/index.ts";
import { run as runHelloSolana } from "./clients/hello_solana/index.ts";
import { run as runCalculator } from "./clients/calculator/index.ts";
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
  (err) => {
    console.error(err);
    process.exit(-1);
  },
);
