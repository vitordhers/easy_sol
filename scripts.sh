#! /bin/bash

SOLANA_PROGRAMS=("hello_solana" "calculator")

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
esac
