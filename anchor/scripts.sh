#!/usr/bin/env bash

ANCHOR_PROGRAMS=("tokens")
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
  if contains "$SELECTED_PROGRAM" "${ANCHOR_PROGRAMS[@]}"; then
    ANCHOR_PROGRAMS=("$SELECTED_PROGRAM")
  else
    echo "❌ Error: Program '$SELECTED_PROGRAM' is not in the list of available programs."
    echo "📜 Available programs: ${ANCHOR_PROGRAMS[*]}"
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
    echo "Cloning programs:"
    for program in "${CLONE_UPGRADEABLE_PROGRAMS[@]}"; do
      echo " - $program"
      CMD+=" --clone-upgradeable-program $program"
    done
    CMD+=" --url https://api.mainnet-beta.solana.com"
    [[ "$1" == "start" ]] && CMD+=" --reset"
    CMD += " --log"
    eval "$CMD"
    ;;

  "reset")
    anchor clean
    rm -rf target/
    ;;

  "build")
    for program in "${ANCHOR_PROGRAMS[@]}"; do
      anchor build --program-name "$program"
    done
    ;;

  "deploy")
    if [[ "$IS_RUNNING" == false ]]; then
      echo "Validator must be running before deploying contracts!"
      exit 1
    fi

    for program in "${ANCHOR_PROGRAMS[@]}"; do
      anchor deploy --program-name "$program"
    done
    anchor programs list
    ;;

  "redeploy")
    "$0" clean
    "$0" build
    "$0" deploy
    ;;

  "clean")
    anchor clean
    rm -rf target/deploy
    ;;

  "logs")
    if [[ "$IS_RUNNING" == false ]]; then
      echo "Validator must be running before logging!"
      exit 1
    fi

    if [[ ${#ANCHOR_PROGRAMS[@]} -ne 1 ]]; then
      echo "Error: logs can only be streamed when exactly one program is selected."
      echo "Current programs: ${ANCHOR_PROGRAMS[*]}"
      exit 1
    fi

    SELECTED_PROGRAM="${ANCHOR_PROGRAMS[0]}"
    IDL_PATH="target/idl/${SELECTED_PROGRAM}.json"

    if [[ ! -f "$IDL_PATH" ]]; then
      echo "Error: IDL not found at $IDL_PATH"
      exit 1
    fi

    PROGRAM_ID=$(solana address -k target/deploy/"${SELECTED_PROGRAM}"-keypair.json)
    if echo "$PROGRAM_ID" | grep -qi "error"; then
      echo "Error fetching program ID:"
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
