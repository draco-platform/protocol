#!/bin/bash

# Stop Solana Test Validator Script

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

print_info() {
    echo -e "${YELLOW}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

echo "🛑 Stopping Solana Test Validator..."

# Kill all solana-test-validator processes
if pkill -f solana-test-validator; then
    print_success "Solana test validator stopped successfully"
else
    print_info "No running solana-test-validator processes found"
fi

# Clean up any leftover test ledger
if [ -d "test-ledger" ]; then
    print_info "Cleaning up test-ledger directory..."
    rm -rf test-ledger
    print_success "test-ledger directory removed"
fi

echo "✅ Cleanup complete!"
