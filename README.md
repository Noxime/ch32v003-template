# Template for embedded rust projects on the CH32V003

This example sets up the HAL and puts PC1 high, PC2 low

## Preparing Rust toolchain for RV32EC

```sh
git clone https://github.com/ch32-rs/rust --branch rv32ec --recursive && cd rust
echo '
change-id = 118703

# Use defaults for codegen affecting custom builds
profile = "codegen"

[llvm]
# Use our own LLVM build instead of downloading from CI
download-ci-llvm = false

[build]
# Build core/compiler_builtins/proc-macro-srv-cli for RV32EC
target = ["riscv32ec-unknown-none-elf", "x86_64-pc-windows-msvc"]
extended = true

[target.riscv32ec-unknown-none-elf]
# Needed to build libraries
cc = "clang"
cxx = "clang++"
ar = "build/host/llvm/bin/llvm-ar"

[rust]
# Enable building LLD for our target as well
lld = true
' >> config.toml

python x.py build
rustup toolchain link custom-rv32ec build/host/stage1
```
