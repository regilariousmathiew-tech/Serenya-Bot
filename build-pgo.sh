#!/usr/bin/env bash
set -e

# Profile-Guided Optimization (PGO) Build Script for Linux
# Ensure llvm-tools is installed:
# rustup component add llvm-tools

PGO_DIR="$(pwd)/target/pgo-data"
MERGED_PROFDATA="$PGO_DIR/merged.profdata"

# 1. Clean previous profile data
rm -rf "$PGO_DIR"
mkdir -p "$PGO_DIR"

# 2. Build instrumented binary
echo -e "\e[32mBuilding instrumented binary with target-cpu=native...\e[0m"
export RUSTFLAGS="-C profile-generate=$PGO_DIR -C target-cpu=native"
cargo build --profile pgo-gen

# 3. Instruction to collect profile data
echo ""
echo -e "\e[36m================================================================================\e[0m"
echo -e "\e[36mSTEP 2: RUN WORKLOAD TO GENERATE PROFILE DATA\e[0m"
echo -e "\e[36mPlease run the compiled binary under realistic workloads to gather profiles.\e[0m"
echo -e "\e[36mExample: Run the bot and play several tracks, search songs, etc.\e[0m"
echo -e "\e[36mThe binary is located at: ./target/pgo-gen/serenya\e[0m"
echo -e "\e[36mPress Enter when you are done running the bot and want to build the optimized binary.\e[0m"
echo -e "\e[36m================================================================================\e[0m"
read -r -p "Press Enter to continue..."

# 4. Locate llvm-profdata tool
SYSROOT=$(rustc --print sysroot)
TARGET_TRIPLE=$(rustc -vV | grep host: | awk '{print $2}')
LLVM_PROFDATA="$SYSROOT/lib/rustlib/$TARGET_TRIPLE/bin/llvm-profdata"

if [ ! -f "$LLVM_PROFDATA" ]; then
    # Try finding in the path as fallback
    LLVM_PROFDATA=$(which llvm-profdata || true)
fi

if [ -z "$LLVM_PROFDATA" ] || [ ! -f "$LLVM_PROFDATA" ]; then
    echo -e "\e[31mllvm-profdata not found. Please install llvm-tools using: rustup component add llvm-tools\e[0m"
    exit 1
fi

# 5. Merge profile data
echo -e "\e[32mMerging profile data...\e[0m"
"$LLVM_PROFDATA" merge -o "$MERGED_PROFDATA" "$PGO_DIR"

# 6. Build optimized binary using profile data
echo -e "\e[32mBuilding optimized binary with target-cpu=native...\e[0m"
export RUSTFLAGS="-C profile-use=$MERGED_PROFDATA -C target-cpu=native"
cargo build --profile pgo-use

echo -e "\e[32mOptimized binary built successfully!\e[0m"
echo -e "\e[33mLocation: ./target/pgo-use/serenya\e[0m"
