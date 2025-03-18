#! /bin/bash

SOLANA_PROGRAMS=("hello_solana" "calculator" "transfer_sol")
CLONE_UPGRADEABLE_PROGRAMS=("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")

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
esac
