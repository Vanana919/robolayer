#!/bin/bash
set -e

# RoboLayer Deployment Script
# Usage: ./scripts/deploy.sh --network <devnet|mainnet-beta>

NETWORK="devnet"
PROGRAM_NAME="robolayer"
KEYPAIR_PATH="./target/deploy/robolayer-keypair.json"

# Parse arguments
while [[ $# -gt 0 ]]; do
  case $1 in
    --network)
      NETWORK="$2"
      shift 2
      ;;
    --keypair)
      KEYPAIR_PATH="$2"
      shift 2
      ;;
    *)
      echo "Unknown option: $1"
      exit 1
      ;;
  esac
done

echo "========================================="
echo "  RoboLayer Deployment"
echo "  Network: $NETWORK"
echo "========================================="
echo ""

# Verify Solana CLI
if ! command -v solana &> /dev/null; then
    echo "Error: Solana CLI not found. Install from https://docs.solana.com/cli/install-solana-cli-tools"
    exit 1
fi

# Verify Anchor CLI
if ! command -v anchor &> /dev/null; then
    echo "Error: Anchor CLI not found. Install with: cargo install --git https://github.com/coral-xyz/anchor avm"
    exit 1
fi

# Set network
echo "[1/5] Setting network to $NETWORK..."
solana config set --url $NETWORK

# Check balance
echo "[2/5] Checking deployer balance..."
BALANCE=$(solana balance | awk '{print $1}')
echo "  Balance: $BALANCE SOL"

if (( $(echo "$BALANCE < 2" | bc -l) )); then
    echo "Warning: Low balance. Deployment requires ~2 SOL."
    if [ "$NETWORK" = "devnet" ]; then
        echo "  Requesting airdrop..."
        solana airdrop 2
        sleep 5
    else
        echo "  Please fund your wallet before deploying to mainnet."
        exit 1
    fi
fi

# Build
echo "[3/5] Building program..."
anchor build

# Get program ID
PROGRAM_ID=$(solana-keygen pubkey $KEYPAIR_PATH 2>/dev/null || echo "")
if [ -z "$PROGRAM_ID" ]; then
    echo "Error: Could not read program keypair at $KEYPAIR_PATH"
    exit 1
fi
echo "  Program ID: $PROGRAM_ID"

# Deploy
echo "[4/5] Deploying to $NETWORK..."
if [ "$NETWORK" = "mainnet-beta" ]; then
    echo "  WARNING: Deploying to MAINNET. Confirm? (y/n)"
    read -r CONFIRM
    if [ "$CONFIRM" != "y" ]; then
        echo "  Deployment cancelled."
        exit 0
    fi
fi

anchor deploy --provider.cluster $NETWORK

# Verify
echo "[5/5] Verifying deployment..."
solana program show $PROGRAM_ID

echo ""
echo "========================================="
echo "  Deployment complete!"
echo "  Program: $PROGRAM_ID"
echo "  Network: $NETWORK"
echo "  Explorer: https://explorer.solana.com/address/$PROGRAM_ID?cluster=$NETWORK"
echo "========================================="
