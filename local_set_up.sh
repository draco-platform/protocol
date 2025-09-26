#!/bin/bash
set -e  # Exit on any error

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

print_step() {
    echo -e "${BLUE}[STEP]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_info() {
    echo -e "${YELLOW}[INFO]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

find_file_with_prefix() {
    prefix=$1
    for file in $prefix*.json; do
        if [ -f "$file" ]; then
            echo "$file"  # Return full filename with .json extension
            return 0
        fi
    done
    return 1
}

echo "=========================================="
echo "🚀 SOLANA LOCAL TESTING SETUP"
echo "=========================================="

########################################################

print_step "Cleaning up any hanging processes..."
rm -i mnt*.json || true
rm -i bos*.json || true
solana config set --url localhost
sleep 3

########################################################

print_step "Creating Mint Authority keypair..."
solana-keygen grind --starts-with bos:1

MINT_AUTHORITY_KEYPAIR=$(find_file_with_prefix "bos")
MINT_AUTHORITY_ADDRESS=$(basename "$MINT_AUTHORITY_KEYPAIR" .json)

print_step "Mint Authority address is $MINT_AUTHORITY_ADDRESS"

solana config set --keypair $MINT_AUTHORITY_KEYPAIR
# After this command, all the commands will be executed from the Mint Authority address perspective
# meaning that for example the `spl-token create-account $TOKEN_MINT_ADDRESS` will create
# an associated token account that has as address (Mint Authority address, Token (mint) address)

solana airdrop 10000 $MINT_AUTHORITY_ADDRESS

print_success "Mint Authority set up successfully"

########################################################

print_step "Creating Token (mint)"
solana-keygen grind --starts-with mnt:1

# This is the address where we are going to set the Token (mint)
TOKEN_MINT_KEYPAIR=$(find_file_with_prefix "mnt")
TOKEN_MINT_ADDRESS=$(basename "$TOKEN_MINT_KEYPAIR" .json)

print_step "Token Mint address is $TOKEN_MINT_ADDRESS"

# Since we have selected the Mint Authority as the default keypair, 
# the Token (mint) will have as authority the Mint Authority, and the address
# of the Token (mint) will be TOKEN_MINT_ADDRESS
spl-token create-token --program-id TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb --enable-metadata $TOKEN_MINT_KEYPAIR

print_success "Token (mint) created successfully"

########################################################

print_step "Creating Token Account for the Mint Authority so its able to hold tokens"
# We are creating an associated token account for the Mint Authority, which has as seed (Mint Authority address, Token (mint) address)

spl-token create-account $TOKEN_MINT_ADDRESS

print_step "Minting 1000000000 tokens to the Mint Authority"

spl-token mint $TOKEN_MINT_ADDRESS 1000000000

print_success "DRACO Token minted successfully"

########################################################

print_step "Disabling mint authority (setting to null) to cap supply at 1 billion tokens"

spl-token authorize $TOKEN_MINT_ADDRESS mint --disable

print_success "Mint authority disabled - no more tokens can ever be minted"

########################################################

ENV_FILE="../frontend/.env.local"
print_step "Updating $ENV_FILE file with TOKEN_MINT_ADDRESS"

if [ -f "$ENV_FILE" ]; then
    grep -v "^TOKEN_MINT_ADDRESS=" "$ENV_FILE" > "$ENV_FILE.tmp" || true
    mv "$ENV_FILE.tmp" "$ENV_FILE"
fi

# Add the TOKEN_MINT_ADDRESS line
echo "VITE_TOKEN_MINT_ADDRESS=$TOKEN_MINT_ADDRESS" >> "$ENV_FILE"
print_success "Updated $ENV_FILE with VITE_TOKEN_MINT_ADDRESS=$TOKEN_MINT_ADDRESS"

########################################################

print_step "Running anchor build..."
if ! anchor build; then
    print_error "Anchor build failed!"
    exit 1
fi
print_success "Anchor build completed successfully!"

########################################################

print_step "Airdrop anchor account to deploy the contract"

# This is the address of the solana-keygen new -o /Users/userName/.config/solana/id.json
# that we set as the account that anchor uses to deploy contracts
ANCHOR_TEST_ACCOUNT="8GvqRwNRH71qgWfcuQThgbQ4VTJsMZamxDjcCQ3ie9uF"
solana airdrop 1000 $ANCHOR_TEST_ACCOUNT

print_success "Airdropped 1000 SOL to $ANCHOR_TEST_ACCOUNT"

########################################################

print_step "Running anchor deploy..."
if ! anchor deploy; then
    print_error "Anchir deploy failed!"
    exit 1
fi
print_success "Anchor deploy completed successfully!"

########################################################

print_step "Copying IDL and types to frontend/anchor and backend/anchor"

mkdir -p ../frontend/anchor
mkdir -p ../backend/anchor

if [ -f "target/idl/draco_protocol.json" ]; then
    cp target/idl/draco_protocol.json ../frontend/anchor/
    cp target/idl/draco_protocol.json ../backend/anchor/
    print_success "Copied draco_protocol.json to frontend/anchor/ and backend/anchor/"
else
    print_error "IDL file not found at target/idl/draco_protocol.json"
fi

if [ -f "target/types/draco_protocol.ts" ]; then
    cp target/types/draco_protocol.ts ../frontend/anchor/
    cp target/types/draco_protocol.ts ../backend/anchor/
    print_success "Copied draco_protocol.ts to frontend/anchor/ and backend/anchor/"
else
    print_error "Types file not found at target/types/draco_protocol.ts"
fi
########################################################

print_info "########################################################"
print_info "########## Public key of the Mint Authority ############"
solana-keygen pubkey $MINT_AUTHORITY_KEYPAIR
print_info "########################################################"
print_info "########## Private key of the Mint Authority ###########"
python3 -c "
import json
with open('$MINT_AUTHORITY_KEYPAIR', 'r') as f:
    keypair = json.load(f)
print(bytes(keypair).hex())
"
print_info "########################################################"
print_info "########################################################"

########################################################