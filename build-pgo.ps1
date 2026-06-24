# Profile-Guided Optimization (PGO) Build Script for Windows
# Ensure you have the llvm-tools component installed:
# rustup component add llvm-tools

$PGO_DIR = Join-Path $Pwd "target/pgo-data"
$MERGED_PROFDATA = Join-Path $PGO_DIR "merged.profdata"

# 1. Clean previous profile data
if (Test-Path $PGO_DIR) {
    Remove-Item -Recurse -Force $PGO_DIR
}
New-Item -ItemType Directory -Path $PGO_DIR | Out-Null

# 2. Build instrumented binary
Write-Host "Building instrumented binary with target-cpu=native..." -ForegroundColor Green
$Env:RUSTFLAGS = "-C profile-generate=$PGO_DIR -C target-cpu=native"
cargo build --profile pgo-gen

# 3. Instruction to collect profile data
Write-Host ""
Write-Host "================================================================================" -ForegroundColor Cyan
Write-Host "STEP 2: RUN WORKLOAD TO GENERATE PROFILE DATA" -ForegroundColor Cyan
Write-Host "Please run the compiled binary under realistic workloads to gather profiles." -ForegroundColor Cyan
Write-Host "Example: Run the bot and play several tracks, search songs, etc." -ForegroundColor Cyan
Write-Host "The binary is located at: .\target\pgo-gen\serenya.exe" -ForegroundColor Yellow
Write-Host "Press enter when you are done running the bot and want to build the optimized binary." -ForegroundColor Cyan
Write-Host "================================================================================" -ForegroundColor Cyan
Read-Host "Press Enter to continue..."

# 4. Locate llvm-profdata tool
$SysRoot = (rustc --print sysroot)
$TargetTriple = (rustc -vV | Select-String "host:").Line.Split(" ")[1]
$LlvmProfData = Join-Path $SysRoot "lib\rustlib\$TargetTriple\bin\llvm-profdata.exe"

if (-not (Test-Path $LlvmProfData)) {
    # Try finding in the path as fallback
    $LlvmProfData = Get-Command llvm-profdata -ErrorAction SilentlyContinue | Select-Object -ExpandProperty Source
}

if (-not $LlvmProfData) {
    Write-Error "llvm-profdata not found. Please install llvm-tools using: rustup component add llvm-tools"
    Exit 1
}

# 5. Merge profile data
Write-Host "Merging profile data..." -ForegroundColor Green
& $LlvmProfData merge -o $MERGED_PROFDATA $PGO_DIR

# 6. Build optimized binary using profile data
Write-Host "Building optimized binary with target-cpu=native..." -ForegroundColor Green
$Env:RUSTFLAGS = "-C profile-use=$MERGED_PROFDATA -C target-cpu=native"
cargo build --profile pgo-use

Write-Host "Optimized binary built successfully!" -ForegroundColor Green
Write-Host "Location: .\target\pgo-use\serenya.exe" -ForegroundColor Yellow
