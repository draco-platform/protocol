print_step() {
    echo -e "${BLUE}[STEP]${NC} $1"
}
print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

########################################################

print_step "Cleaning up any hanging processes..."

pkill solana-test-validator || true
pkill spl-token || true

rm oracle13.json
rm oracle12.json
rm oracle11.json
rm oracle10.json
rm oracle9.json
rm oracle8.json
rm oracle7.json
rm oracle6.json
rm oracle5.json
rm oracle4.json
rm oracle3.json
rm oracle2.json
rm oracle1.json
rm oracle0.json
rm randomness_queue.json
rm sb_randomness_config.json
rm metadata.so
rm switchboard.so
rm ondemand.so
sleep 3

print_success "Cleaned up any hanging processes"

########################################################

print_step "Seting up Switchboard locally"

solana account -u m --output json-compact --output-file oracle13.json 9pPCSotuPGUDgdYUtngMCemNi3KHdWFvwLhqx57KkbXb
solana account -u m --output json-compact --output-file oracle12.json 645bCKGspzjizB1CN5h2A5CThoT2MxVTpFCCHhrBjuHN
solana account -u m --output json-compact --output-file oracle11.json CdAPHuaduAH4NmxC8AvCqWuiQ1LduTrawnFBJKBdjPy4
solana account -u m --output json-compact --output-file oracle10.json DznDmDUujQFSVjw5QWSQgwwPLMPvBWDdYpcS1m5YgUuw
solana account -u m --output json-compact --output-file oracle9.json Af6RAyMnkFk8FNyzFRPa7L2hfu4eXgek9HYBZQpQvcyD
solana account -u m --output json-compact --output-file oracle8.json B6myYsSNLmdbQxwXVhBQZsjbSVLRtoRhYF3hMFeRyftF
solana account -u m --output json-compact --output-file oracle7.json 48t1JSKsvDkgGHYxNrECg1ejnfmT111sGzwdLEoep7bb
solana account -u m --output json-compact --output-file oracle6.json 31Uys8oYqNAiRUKR9i24qLaG5ninMFuXckpkfV3FaPDp
solana account -u m --output json-compact --output-file oracle5.json 5LD5BjwuNN397r4xDubXhhdxX3SSx9wS4ub33J64pjfW
solana account -u m --output json-compact --output-file oracle4.json Bcup5msLa9wK6FxnSCZDQ6CGBuoAD6Wm78Epgi3KwGzV
solana account -u m --output json-compact --output-file oracle3.json 5wCwgqgPtFB9jwjZxLVkM717SGaZKmXXpvXYsyLehu69
solana account -u m --output json-compact --output-file oracle2.json 5eVyN3Wx88y3d19kvYC9wBhdaZAwNdmKeA3LiXKEm9hH
solana account -u m --output json-compact --output-file oracle1.json 8ev3ovH86XmD45JU6YhPy6B3ZVZonixLMVGEcw1B6gwC
solana account -u m --output json-compact --output-file oracle0.json 3Nv1DJdf7163FcB5dFEQGKbw6dUK4HqtwuUcyUf3DWni
solana account -u m --output json-compact --output-file randomness_queue.json A43DyUGA7s8eXPxqEjJY6EBu1KKbNgfxF8h17VAHn13w
solana account -u m --output json-compact --output-file sb_randomness_config.json 7Gs9n5FQMeC9XcEhg281bRZ6VHRrCvqp5Yq1j78HkvNa
solana program dump -u m SBondMDrcV3K4kxZR1HNVT7osZxAHVHgYXL5Ze1oMUv ondemand.so
solana program dump -u m SW1TCH7qEPTdLsDHRgPuMQjbQxKdH2aBStViMFnt64f switchboard.so
solana program dump -u m metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s metadata.so

print_success "Switchboard set up successfully"

########################################################

print_step "Restarting Solana test validator..."
solana-test-validator -r \
    --account 9pPCSotuPGUDgdYUtngMCemNi3KHdWFvwLhqx57KkbXb oracle13.json \
    --account 645bCKGspzjizB1CN5h2A5CThoT2MxVTpFCCHhrBjuHN oracle12.json \
    --account CdAPHuaduAH4NmxC8AvCqWuiQ1LduTrawnFBJKBdjPy4 oracle11.json \
    --account DznDmDUujQFSVjw5QWSQgwwPLMPvBWDdYpcS1m5YgUuw oracle10.json \
    --account Af6RAyMnkFk8FNyzFRPa7L2hfu4eXgek9HYBZQpQvcyD oracle9.json \
    --account B6myYsSNLmdbQxwXVhBQZsjbSVLRtoRhYF3hMFeRyftF oracle8.json \
    --account 48t1JSKsvDkgGHYxNrECg1ejnfmT111sGzwdLEoep7bb oracle7.json \
    --account 31Uys8oYqNAiRUKR9i24qLaG5ninMFuXckpkfV3FaPDp oracle6.json \
    --account 5LD5BjwuNN397r4xDubXhhdxX3SSx9wS4ub33J64pjfW oracle5.json \
    --account Bcup5msLa9wK6FxnSCZDQ6CGBuoAD6Wm78Epgi3KwGzV oracle4.json \
    --account 5wCwgqgPtFB9jwjZxLVkM717SGaZKmXXpvXYsyLehu69 oracle3.json \
    --account 5eVyN3Wx88y3d19kvYC9wBhdaZAwNdmKeA3LiXKEm9hH oracle2.json \
    --account 8ev3ovH86XmD45JU6YhPy6B3ZVZonixLMVGEcw1B6gwC oracle1.json \
    --account 3Nv1DJdf7163FcB5dFEQGKbw6dUK4HqtwuUcyUf3DWni oracle0.json \
    --bpf-program metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s metadata.so \
    --bpf-program SW1TCH7qEPTdLsDHRgPuMQjbQxKdH2aBStViMFnt64f switchboard.so\
    --account A43DyUGA7s8eXPxqEjJY6EBu1KKbNgfxF8h17VAHn13w randomness_queue.json \
    --bpf-program SBondMDrcV3K4kxZR1HNVT7osZxAHVHgYXL5Ze1oMUv ondemand.so \
    --account 7Gs9n5FQMeC9XcEhg281bRZ6VHRrCvqp5Yq1j78HkvNa sb_randomness_config.json
print_success "Validator started"