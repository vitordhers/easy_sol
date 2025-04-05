#!/usr/bin/env bash

SOLANA_PROGRAMS=("hello_solana" "calculator" "transfer_sol" "tokens")
CLONE_UPGRADEABLE_PROGRAMS=("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")
IS_RUNNING=false
SELECTED_PROGRAM=""

if pgrep -f solana-test-validator > /dev/null; then
  IS_RUNNING=true
fi

for arg in "$@"; do
  case $arg in
    --program=*)
      SELECTED_PROGRAM="${arg#*=}"
      ;;
  esac
done

contains() {
    local match="$1"
    shift
    for item in "$@"; do
        if [[ "$item" == "$match" ]]; then
            return 0  
        fi
    done
    return 1  
}

if [[ -n "$SELECTED_PROGRAM" ]]; then
    if contains "$SELECTED_PROGRAM" "${SOLANA_PROGRAMS[@]}"; then
        SOLANA_PROGRAMS=("$SELECTED_PROGRAM")
    else
        echo "❌ Error: Program '$SELECTED_PROGRAM' is not in the list of available programs."
        echo "📜 Available programs: ${SOLANA_PROGRAMS[*]}"
        exit 1
    fi
fi

case $1 in
  "continue"|"start")
    if [[ "$IS_RUNNING" == true ]]; then
      echo "Validator is already running!"
      exit 1
    fi
    CMD="solana-test-validator"
    for arg in "$@"; do
      case $arg in
        --clone-programs)
          echo "Cloning programs:"
          for program in "${CLONE_UPGRADEABLE_PROGRAMS[@]}"; do
            echo " - $program"
            CMD+=" --clone-upgradeable-program $program"
          done
          CMD+=" --url https://api.mainnet-beta.solana.com"
          ;;
      esac
    done
    [[ "$1" == "start" ]] && CMD+=" --reset"
    eval "$CMD"
    ;;

  "reset")
    if [[ "$IS_RUNNING" == true ]]; then
      for x in $(solana program-v4 show | awk 'RP==0 {print $1}'); do 
          if [[ $x != "Program" ]]; then 
              solana program-v4 close --program-id $x
          fi
      done
    fi
    cargo clean --manifest-path=./contracts/Cargo.toml
    rm -rf contracts/target/deploy
    ;;

  "build")
    for program in "${SOLANA_PROGRAMS[@]}"; do
      cargo build-sbf --manifest-path=./contracts/$program/Cargo.toml
    done
    ;;

  "deploy")
    if [[ "$IS_RUNNING" == false ]]; then
      echo "Validator must be running before deploying contracts!"
      exit 1
    fi
    for program in "${SOLANA_PROGRAMS[@]}"; do
      deploy_output=$(solana program-v4 deploy \
        --program-keypair ./contracts/target/deploy/$program-keypair.json \
        ./contracts/target/deploy/$program.so)
      program_id=$(echo "$deploy_output" | awk '/Program Id:/ {print $NF}')
      echo "$program id: $program_id"
    done
    solana program-v4 show
    ;;

  "redeploy")
    "$0" clean 
    "$0" build
    "$0" deploy
    ;;

  "clean")
    for program in "${SOLANA_PROGRAMS[@]}"; do
      cargo clean --manifest-path=./contracts/$program/Cargo.toml 
      rm -f contracts/target/deploy/$program-keypair.json
      rm -f contracts/target/deploy/$program.so
    done
    ;;

  "logs")
    if [[ "$IS_RUNNING" == false ]]; then
      echo "Validator must be running before logging!"
      exit 1
    fi

    if [[ ${#SOLANA_PROGRAMS[@]} -ne 1 ]]; then
      echo "Error: logs can only be streamed when exactly one program is selected."
      echo "Current programs: ${SOLANA_PROGRAMS[*]}"
      exit 1
    fi

    SELECTED_PROGRAM="${SOLANA_PROGRAMS[0]}"
    KEYPAIR_PATH="./contracts/target/deploy/${SELECTED_PROGRAM}-keypair.json"

    if [[ ! -f "$KEYPAIR_PATH" ]]; then
      echo "Error: Keypair file not found at $KEYPAIR_PATH"
      exit 1
    fi

    PROGRAM_ID=$(solana address -k "$KEYPAIR_PATH" 2>&1)
    if echo "$PROGRAM_ID" | grep -qi "error"; then
      echo "Error while fetching program ID:"
      echo "$PROGRAM_ID"
      exit 1
    fi

    LINES=10
    for arg in "$@"; do
      case $arg in
        --lines=*)
          LINES="${arg#*=}"
          ;;
      esac
    done
    
    echo "Streaming logs for Program ID: $PROGRAM_ID (Showing last $LINES lines per match)"
    solana logs --commitment processed | grep --line-buffered "$PROGRAM_ID invoke" -A "$LINES"
    ;;
esac
