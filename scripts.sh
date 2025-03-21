#! /bin/bash

SOLANA_PROGRAMS=("hello_solana" "calculator" "transfer_sol")

case $1 in
    "reset")
        for x in $(solana program-v4 show | awk 'RP==0 {print $1}'); do 
            if [[ $x != "Program" ]]; 
            then 
                solana program-v4 close $x;
            fi
        done
        cargo clean --manifest-path=./contracts/Cargo.toml
        rm -rf contracts/target/deploy
        ;;
    "clean")
        cargo clean --manifest-path=./contracts/Cargo.toml 
        ;;
    "build")
        cargo build-sbf --manifest-path=./contracts/Cargo.toml
        ;;
    "deploy")
        cargo build-sbf --manifest-path=./contracts/Cargo.toml
        for program in "${SOLANA_PROGRAMS[@]}"; do
            solana program-v4 deploy --program-keypair ./contracts/target/deploy/$program-keypair.json ./contracts/target/deploy/$program.so
        done
        solana program-v4 show
        ;;
    "reset-and-build")
        for x in $(solana program-v4 show | awk 'RP==0 {print $1}'); do 
            if [[ $x != "Program" ]]; 
            then 
                solana program-v4 close $x; 
            fi
        done
        cargo clean --manifest-path=./contracts/Cargo.toml
        rm -rf contracts/target/deploy
        cargo build-sbf --manifest-path=./contracts/Cargo.toml
        for program in "${SOLANA_PROGRAMS[@]}"; do
          solana program-v4 deploy --program-keypair ./contracts/target/deploy/$program-keypair.json ./contracts/target/deploy/$program.so  
        done
        solana program-v4 show
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
