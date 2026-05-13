#!/bin/bash
set -euo pipefail

# Robodyne deployment helper.
# Usage:
#   ./scripts/deploy.sh --network sepolia
#   ./scripts/deploy.sh --network mainnet
#
# Required env (load via .env or shell):
#   DEPLOYER_PRIVATE_KEY  — 0x-prefixed private key (Sepolia funded)
#   SEPOLIA_RPC_URL       — JSON-RPC endpoint
#   MAINNET_RPC_URL       — JSON-RPC endpoint
#   BASE_RPC_URL          — JSON-RPC endpoint
#   ETHERSCAN_API_KEY     — for source verification
#   TREASURY_ADDRESS      — slash payout recipient

NETWORK="sepolia"

while [[ $# -gt 0 ]]; do
  case $1 in
    --network)
      NETWORK="$2"
      shift 2
      ;;
    *)
      echo "Unknown option: $1"
      exit 1
      ;;
  esac
done

echo "========================================="
echo "  Robodyne Deployment"
echo "  Network: $NETWORK"
echo "========================================="

if ! command -v forge &> /dev/null; then
  echo "Error: foundry not installed. Run: curl -L https://foundry.paradigm.xyz | bash && foundryup"
  exit 1
fi

RPC_VAR=""
case "$NETWORK" in
  sepolia)      RPC_VAR="SEPOLIA_RPC_URL" ;;
  mainnet)      RPC_VAR="MAINNET_RPC_URL" ;;
  base)         RPC_VAR="BASE_RPC_URL" ;;
  base_sepolia) RPC_VAR="BASE_SEPOLIA_RPC_URL" ;;
  *)
    echo "Unknown network: $NETWORK"
    exit 1
    ;;
esac

RPC_URL="${!RPC_VAR:-}"
if [[ -z "$RPC_URL" ]]; then
  echo "Error: $RPC_VAR is not set."
  exit 1
fi

if [[ "$NETWORK" == "mainnet" ]]; then
  echo "WARNING: Deploying to MAINNET. Confirm? (y/n)"
  read -r CONFIRM
  if [[ "$CONFIRM" != "y" ]]; then
    echo "Deployment cancelled."
    exit 0
  fi
fi

forge script script/Deploy.s.sol:Deploy \
  --rpc-url "$RPC_URL" \
  --broadcast \
  --verify \
  -vvvv

echo "========================================="
echo "  Deployment complete."
echo "  Network: $NETWORK"
echo "========================================="
