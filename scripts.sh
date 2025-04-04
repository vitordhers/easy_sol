#! /bin/bash

SOLANA_PROGRAMS=("hello_solana" "calculator" "transfer_sol" "tokens")
CLONE_UPGRADEABLE_PROGRAMS=("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")


SELECTED_PROGRAM=$(echo "$@" | awk 'match($0, /--program[[:space:]]+([^[:space:]]+)/, a) {print a[1]}')

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

case $1 in
    "start")
        CMD="solana-test-validator"
        if [[ "$2" == "--clone-programs" ]]; then
          echo "Cloning programs:"
            for program in "${CLONE_UPGRADEABLE_PROGRAMS[@]}"; do
                echo " - $program"
                CMD+=" --clone-upgradeable-program $program"
            done
            CMD+=" --url https://api.mainnet-beta.solana.com"
        fi
        eval "$CMD"
        ;;
    "start-new")
        CMD="solana-test-validator"
        if [[ "$2" == "--clone-programs" ]]; then
          echo "Cloning programs:"
            for program in "${CLONE_UPGRADEABLE_PROGRAMS[@]}"; do
                echo " - $program"
                CMD+=" --clone-upgradeable-program $program"
            done
            CMD+=" --url https://api.mainnet-beta.solana.com"
        fi
        CMD+=" --reset"
        eval "$CMD"
        ;;
    "reset")
        for x in $(solana program-v4 show | awk 'RP==0 {print $1}'); do 
            if [[ $x != "Program" ]]; 
            then 
                solana program-v4 close --program-id $x;
            fi
        done
        cargo clean --manifest-path=./contracts/Cargo.toml
        rm -rf contracts/target/deploy
        ;;
    "clean")
        cargo clean --manifest-path=./contracts/Cargo.toml 
        rm -rf contracts/target/deploy
        ;;
    "build")
        cargo build-sbf --manifest-path=./contracts/Cargo.toml
        ;;
    "deploy")
        if [[ -n "$SELECTED_PROGRAM" ]]; then
          if ! contains "$SELECTED_PROGRAM" "${SOLANA_PROGRAMS[@]}"; then
                echo "Error: Program '$SELECTED_PROGRAM' is not in the list of available programs."
                echo "Available programs: ${SOLANA_PROGRAMS[*]}"
                exit 1
          fi
          cargo build-sbf --manifest-path=./contracts/$SELECTED_PROGRAM/Cargo.toml
          deploy_output=$(solana program-v4 deploy --program-keypair ./contracts/target/deploy/$SELECTED_PROGRAM-keypair.json ./contracts/target/deploy/$SELECTED_PROGRAM.so)
          program_id=$(echo "$deploy_output" | awk '/Program Id:/ {print $NF}')
          echo "$SELECTED_PROGRAM id: $program_id"
        else
          cargo build-sbf --manifest-path=./contracts/Cargo.toml
          for program in "${SOLANA_PROGRAMS[@]}"; do
            deploy_output=$(solana program-v4 deploy --program-keypair ./contracts/target/deploy/$program-keypair.json ./contracts/target/deploy/$program.so)
            program_id=$(echo "$deploy_output" | awk '/Program Id:/ {print $NF}')
            echo "$program id: $program_id"
          done
        fi
        solana program-v4 show
        ;;
    "reset-and-deploy")
        "$0" clean 
        "$0" build
        "$0" deploy
        ;;
    "logs")
        PROGRAM_ID=""
        LINES=10

        shift   
        while [[ $# -gt 0 ]]; do
            case $1 in
                --program-id)
                    PROGRAM_ID="$2"
                    shift 2
                    ;;
                --lines)
                    LINES="$2"
                    shift 2
                    ;;
                *)
                    echo "Unknown argument: $1"
                    ;;
            esac
        done
        if [[ -z "$PROGRAM_ID" ]]; then
            echo "Error: --program-id is required."
            exit 1
        fi
        echo "Streaming logs for Program ID: $PROGRAM_ID (Showing last $LINES lines per match)"
        solana logs | grep --line-buffered "$PROGRAM_ID invoke" -A "$LINES"
        ;;

    *)
        echo "Usage: $0 {start|logs}"
        ;;
esac
