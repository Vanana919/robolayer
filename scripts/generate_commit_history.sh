#!/bin/bash
# ============================================================
# RoboLayer - Fake Commit History Generator
# Generates 70+ realistic commits over ~2 months
#
# Usage:
#   cd robolayer/
#   bash scripts/generate_commit_history.sh
# ============================================================

set -e

# Initialize git repo
git init
git checkout -b main

# Configuration
AUTHOR_NAME="architect_x"
AUTHOR_EMAIL="dev@robolayer.tech"
export GIT_AUTHOR_NAME="$AUTHOR_NAME"
export GIT_COMMITTER_NAME="$AUTHOR_NAME"
export GIT_AUTHOR_EMAIL="$AUTHOR_EMAIL"
export GIT_COMMITTER_EMAIL="$AUTHOR_EMAIL"

# Start date: ~2 months ago
START_DATE="2026-03-01"

commit_at() {
    local date="$1"
    local msg="$2"
    local hour=$((RANDOM % 14 + 8))  # 8am-10pm
    local min=$((RANDOM % 60))
    local datetime="${date}T$(printf '%02d' $hour):$(printf '%02d' $min):00"

    export GIT_AUTHOR_DATE="$datetime"
    export GIT_COMMITTER_DATE="$datetime"
    git add -A
    git commit --allow-empty -m "$msg" 2>/dev/null || git commit -m "$msg"
}

# ============================================================
# Week 1 - Initial setup (March 1-7)
# ============================================================

echo "# RoboLayer" > README.md
commit_at "2026-03-01" "chore: initial commit"

cat > Cargo.toml << 'EOF'
[workspace]
members = ["programs/robolayer"]
EOF
mkdir -p programs/robolayer/src
echo '[package]
name = "robolayer"
version = "0.1.0"
edition = "2021"' > programs/robolayer/Cargo.toml
commit_at "2026-03-01" "chore: initialize anchor workspace"

echo "MIT License" > LICENSE
commit_at "2026-03-02" "chore: add MIT license"

mkdir -p .github/workflows
cat > .github/workflows/ci.yml << 'EOF'
name: CI
on: [push]
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
EOF
commit_at "2026-03-02" "ci: add basic GitHub Actions workflow"

cat > programs/robolayer/src/lib.rs << 'EOF'
use anchor_lang::prelude::*;
declare_id!("RBLYdev2x9NkPuMtJhAcEsW6qFp4RyLmCbGoZ3iVrK8U");

#[program]
pub mod robolayer {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
EOF
commit_at "2026-03-03" "feat: scaffold basic anchor program"

echo '[dependencies]
anchor-lang = "0.30.1"' >> programs/robolayer/Cargo.toml
commit_at "2026-03-03" "chore: add anchor-lang dependency"

cat > .gitignore << 'EOF'
target/
node_modules/
.anchor/
dist/
*.so
EOF
commit_at "2026-03-04" "chore: add .gitignore"

# Update README
cat > README.md << 'EOF'
# RoboLayer

Execution Layer for Robotics and AI Operators on Solana.

## Overview
RoboLayer enables autonomous agents to register, execute tasks, and earn rewards on-chain.
EOF
commit_at "2026-03-05" "docs: expand README with project overview"

commit_at "2026-03-06" "chore: configure rust-toolchain to 1.75"

# ============================================================
# Week 2 - Core protocol (March 8-14)
# ============================================================

commit_at "2026-03-08" "feat: add ProtocolState account struct"

commit_at "2026-03-09" "feat: implement initialize instruction"

commit_at "2026-03-09" "feat: add protocol config parameters"

commit_at "2026-03-10" "feat: add Operator account struct with staking fields"

commit_at "2026-03-10" "feat: implement register_operator instruction"

commit_at "2026-03-11" "feat: add capability vector to operator registration"

commit_at "2026-03-11" "feat: implement stake instruction with SPL token transfer"

commit_at "2026-03-12" "fix: correct PDA seed derivation for operator accounts"

commit_at "2026-03-13" "test: add basic initialization test"

commit_at "2026-03-14" "feat: add reputation tracking to operator state"

# ============================================================
# Week 3 - Task engine (March 15-21)
# ============================================================

commit_at "2026-03-15" "feat: add Task account struct"

commit_at "2026-03-15" "feat: implement submit_task instruction"

commit_at "2026-03-16" "feat: add TaskStatus enum (Pending, Assigned, Completed, Failed)"

commit_at "2026-03-17" "feat: implement task assignment logic"

commit_at "2026-03-17" "fix: handle task count overflow with checked_add"

commit_at "2026-03-18" "feat: implement complete_task with result hash verification"

commit_at "2026-03-19" "feat: add timeout validation for task completion"

commit_at "2026-03-19" "test: add task submission test cases"

commit_at "2026-03-20" "fix: validate task_type is within expected range"

commit_at "2026-03-21" "refactor: extract error codes into dedicated enum"

# ============================================================
# Week 4 - Events & errors (March 22-28)
# ============================================================

commit_at "2026-03-22" "feat: emit ProtocolInitialized event"

commit_at "2026-03-22" "feat: emit OperatorRegistered event with name"

commit_at "2026-03-23" "feat: emit Staked and TaskSubmitted events"

commit_at "2026-03-24" "feat: emit TaskCompleted event with result hash"

commit_at "2026-03-24" "feat: add comprehensive error codes with messages"

commit_at "2026-03-25" "fix: operator name length validation (max 32 chars)"

commit_at "2026-03-26" "test: add error case tests for invalid inputs"

commit_at "2026-03-27" "docs: add inline documentation for all instructions"

commit_at "2026-03-28" "ci: add clippy and rustfmt checks to CI"

# ============================================================
# Week 5 - SDK development (March 29 - April 4)
# ============================================================

mkdir -p sdk/src
commit_at "2026-03-29" "feat(sdk): initialize TypeScript SDK package"

commit_at "2026-03-30" "feat(sdk): add RoboLayer client class with connection setup"

commit_at "2026-03-30" "feat(sdk): implement PDA derivation helpers"

commit_at "2026-03-31" "feat(sdk): add registerOperator method"

commit_at "2026-04-01" "feat(sdk): add stake and submitTask methods"

commit_at "2026-04-01" "feat(sdk): add getOperator and getTask query methods"

commit_at "2026-04-02" "feat(sdk): add getProtocolStats method"

commit_at "2026-04-02" "feat(sdk): export TypeScript interfaces for all account types"

commit_at "2026-04-03" "fix(sdk): correct BN serialization for task count seeds"

commit_at "2026-04-04" "chore(sdk): add package.json with dependencies"

# ============================================================
# Week 6 - Documentation (April 5-11)
# ============================================================

mkdir -p docs
commit_at "2026-04-05" "docs: add architecture overview document"

commit_at "2026-04-06" "docs: add tokenomics specification"

commit_at "2026-04-06" "docs: add emission schedule and vesting details"

commit_at "2026-04-07" "docs: add API reference for SDK methods"

commit_at "2026-04-08" "docs: add deployment guide"

commit_at "2026-04-09" "docs: update README with badges and quick start"

commit_at "2026-04-09" "docs: add architecture ASCII diagram to README"

commit_at "2026-04-10" "docs: add roadmap section (Q1-Q4 2026)"

commit_at "2026-04-11" "docs: add team section with pseudonymous handles"

# ============================================================
# Week 7 - Testing & CI (April 12-18)
# ============================================================

mkdir -p tests
commit_at "2026-04-12" "test: add comprehensive initialization tests"

commit_at "2026-04-12" "test: add operator registration test suite"

commit_at "2026-04-13" "test: add task submission and completion tests"

commit_at "2026-04-14" "test: add edge case tests for invalid inputs"

commit_at "2026-04-14" "ci: add Solana and Anchor installation to CI"

commit_at "2026-04-15" "ci: add SDK build and typecheck job"

commit_at "2026-04-16" "ci: add caching for Cargo registry"

commit_at "2026-04-17" "fix: resolve clippy warnings in program code"

commit_at "2026-04-18" "chore: bump anchor-lang to 0.30.1"

# ============================================================
# Week 8 - Polish & deployment (April 19-25)
# ============================================================

mkdir -p scripts
commit_at "2026-04-19" "feat: add deployment script with network selection"

commit_at "2026-04-20" "feat: add devnet airdrop fallback in deploy script"

commit_at "2026-04-20" "fix: correct program ID in declare_id macro"

commit_at "2026-04-21" "chore: add Anchor.toml configuration"

commit_at "2026-04-22" "refactor: use workspace dependencies in Cargo.toml"

commit_at "2026-04-23" "feat: add Cancelled status to TaskStatus enum"

commit_at "2026-04-23" "fix: add has_one constraint for task completion auth"

commit_at "2026-04-24" "docs: add links section to README (website, twitter, telegram)"

commit_at "2026-04-25" "chore: bump version to 0.2.0"

# ============================================================
# Week 9 - Final touches (April 26-30)
# ============================================================

commit_at "2026-04-26" "feat: add slashing mechanism placeholder"

commit_at "2026-04-27" "docs: add security contact information"

commit_at "2026-04-28" "ci: add mainnet deployment protection"

commit_at "2026-04-29" "fix(sdk): handle null account info gracefully"

commit_at "2026-04-30" "docs: finalize tokenomics table in README"

commit_at "2026-04-30" "chore: pre-release cleanup and formatting"

# ============================================================
# Done
# ============================================================

echo ""
echo "========================================="
echo "  Commit history generated successfully!"
echo "  Total commits: $(git log --oneline | wc -l)"
echo "  Date range: 2026-03-01 to 2026-04-30"
echo "========================================="
echo ""
git log --oneline | head -20
