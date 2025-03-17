import { AvailableProgram } from "./shared/enums";
import { run as runHelloSolana } from "./clients/hello_solana/index";

const main = async () => {
  const args = process.argv.slice(2);

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
      `Program ${program} is not a valid program! Valid programs are: ${availablePrograms.join(", ")}`,
    );
  }
  console.log(`Running ${program} ...`);

  switch (program) {
    case AvailableProgram.HelloSolana: {
      return await runHelloSolana();
    }
    case AvailableProgram.Calculator: {
      return;
    }
    default: {
      throw new Error(
        `Program ${program} is not a valid program! Valid programs are: ${availablePrograms.join(", ")}`,
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
