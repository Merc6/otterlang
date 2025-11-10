# OtterLang

<p>
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://github.com/jonathanmagambo/otterlang/blob/main/image.png?raw=true" width="400">
      <img src="https://github.com/jonathanmagambo/otterlang/blob/main/image.png?raw=true" width="400" alt="OtterLang Logo" />
    </picture>
    <br>
    <strong>Simple syntax, native performance, transparent Rust FFI.</strong>
</p>

[![Build Status](https://github.com/jonathanmagambo/otterlang/workflows/CI/badge.svg)](https://github.com/jonathanmagambo/otterlang/actions)
[![Discord](https://img.shields.io/badge/Discord-Join%20Server-5865F2?style=flat&logo=discord&logoColor=white)](https://discord.gg/y3b4QuvyFk)

An indentation-sensitive programming language with an LLVM backend. OtterLang compiles to native binaries with a focus on simplicity and performance.

## Quick Start

```bash
git clone https://github.com/jonathanmagambo/otterlang.git
cd otterlang

# Using Nix (recommended)
nix develop
cargo +nightly build --release

# Create and run your first program
cat > hello.ot << 'EOF'
def main():
    print("Hello from OtterLang!")
EOF

otter run hello.ot
```

## Installation

### Using Nix (Recommended)

```bash
nix develop
cargo +nightly build --release
```

The Nix flake automatically provides Rust nightly, LLVM 18, and all dependencies.

### Manual Setup

**Prerequisites:**
- Rust (via rustup) - nightly required for FFI features
- LLVM 18

**macOS:**
```bash
brew install llvm@18
export LLVM_SYS_181_PREFIX=$(brew --prefix llvm@18)
export LLVM_SYS_180_PREFIX=$LLVM_SYS_181_PREFIX
export PATH="$LLVM_SYS_181_PREFIX/bin:$PATH"
rustup toolchain install nightly
cargo +nightly build --release
```

**Ubuntu/Debian:**
```bash
sudo apt-get install -y llvm-18 llvm-18-dev clang-18
export LLVM_SYS_181_PREFIX=/usr/lib/llvm-18
export LLVM_SYS_180_PREFIX=$LLVM_SYS_181_PREFIX
rustup toolchain install nightly
cargo +nightly build --release
```

**Fedora 43:**
```bash
sudo dnf -y install llvm18 llvm18-devel clang18
export LLVM_SYS_181_PREFIX=/usr/lib64/llvm18
export LLVM_SYS_180_PREFIX=$LLVM_SYS_181_PREFIX
rustup toolchain install nightly
cargo +nightly build --release
```

**Windows:**
```powershell
# Install LLVM 18.1 using llvmenv (recommended)
cargo install llvmenv --locked
llvmenv install 18.1
llvmenv global 18.1

# Set environment variables
$llvmPath = llvmenv prefix
$env:LLVM_SYS_181_PREFIX = $llvmPath
$env:LLVM_SYS_180_PREFIX = $llvmPath
$env:Path = "$llvmPath\bin;$env:Path"

# Alternative: Install using winget or Chocolatey
# winget install --id LLVM.LLVM --silent --accept-package-agreements --accept-source-agreements
# choco install llvm -y
# $env:LLVM_SYS_181_PREFIX = "C:\Program Files\LLVM"
# $env:LLVM_SYS_180_PREFIX = $env:LLVM_SYS_181_PREFIX
# $env:Path = "$env:LLVM_SYS_181_PREFIX\bin;$env:Path"

# Install Rust nightly
rustup toolchain install nightly
rustup default nightly

# Build
cargo +nightly build --release
```

**Note for Windows:** If using winget/Chocolatey, LLVM may be installed in `C:\Program Files\LLVM` or `C:\Program Files (x86)\LLVM`.

**Important:** On Windows, you must use the **x64 Native Tools Command Prompt for VS 2022** to build. The MSVC linker requires environment variables that are automatically set in the Developer Command Prompt. Open it from the Start menu, then navigate to your project directory and run the build commands. Regular PowerShell/CMD will not have the MSVC environment configured.

## After Building

Once the build completes successfully, you can:

**Run a program:**
```bash
cargo +nightly run --release --bin otterlang -- run examples/basic/hello.ot
```

**Build an executable:**
```bash
cargo +nightly run --release --bin otterlang -- build examples/basic/hello.ot -o hello
```

**Run tests:**
```bash
cargo +nightly test --release
```

**Use the compiler directly:**
```bash
# The binary is located at:
# target/release/otterlang (or target/release/otterlang.exe on Windows)
./target/release/otterlang run program.ot
# Or on Windows:
# target\release\otterlang.exe run program.ot
```

## Language Features

OtterLang features a clean, indentation-based syntax with modern language features:

- **Pythonic syntax** - `def` for functions, `class` for structs, `print()` for output
- **Type system** - Static typing with type inference
- **Enums and pattern matching** - Tagged unions with `match` expressions
- **Exception handling** - `try/except/finally` blocks with zero-cost abstractions
- **Concurrency** - `spawn` and `await` for async operations
- **Transparent Rust FFI** - Use any Rust crate without manual bindings

For complete syntax and language details, see the [Language Specification](docs/LANGUAGE_SPEC.md).

### Transparent Rust FFI

Automatically use any Rust crate without manual configuration. No manual bindings needed - just `use rust:crate_name` and start using it. See [docs/FFI_TRANSPARENT.md](docs/FFI_TRANSPARENT.md) for details.

### Standard Library

Built-in modules include `core` (Option, Result), `math`, `io`, `time`, and more. See the [API Reference](docs/API_REFERENCE.md) for complete documentation.


## CLI Commands

```bash
otterlang run program.ot          # Run program
otterlang build program.ot -o out # Build executable
otterlang build program.ot --target wasm32-unknown-unknown -o out.wasm # Build to WebAssembly
otterlang fmt                      # Format code
otterlang repl                     # Start REPL
otterlang profile memory program.ot # Profile memory
```

### WebAssembly Support

OtterLang can compile to WebAssembly! Use the `--target` flag:

```bash
# Compile to WebAssembly (wasm32-unknown-unknown)
otterlang build program.ot --target wasm32-unknown-unknown -o program.wasm

# Compile to WebAssembly System Interface (wasm32-wasi)
otterlang build program.ot --target wasm32-wasi -o program.wasm
```

**Requirements:**
- LLVM 18 with WebAssembly target support
- `clang` and `wasm-ld` in your PATH (usually included with LLVM)

When targeting `wasm32-wasi` the generated binary can talk directly to WASI's
stdio and wall-clock APIs. For the more barebones `wasm32-unknown-unknown`
target we import a minimal host surface so you can decide how to surface
output:

- `env.otter_write_stdout(ptr: i32, len: i32)` – write UTF-8 data to stdout
- `env.otter_write_stderr(ptr: i32, len: i32)` – write UTF-8 data to stderr
- `env.otter_time_now_ms() -> i64` – optional wall-clock timestamp in ms

A tiny JavaScript host that wires these up under Node.js looks like:

```js
import fs from 'node:fs';

const memory = new WebAssembly.Memory({ initial: 8 });
const decoder = new TextDecoder();

const env = {
  memory,
  otter_write_stdout(ptr, len) {
    const bytes = new Uint8Array(memory.buffer, ptr, len);
    process.stdout.write(decoder.decode(bytes));
  },
  otter_write_stderr(ptr, len) {
    const bytes = new Uint8Array(memory.buffer, ptr, len);
    process.stderr.write(decoder.decode(bytes));
  },
  otter_time_now_ms() {
    return BigInt(Date.now());
  },
};

const { instance } = await WebAssembly.instantiate(fs.readFileSync('program.wasm'), { env });
instance.exports.main?.();
```

The generated `.wasm` file can be run in any WebAssembly runtime (Node.js, browsers, wasmtime, etc.).

## Examples

**Basic Programs:**
- `examples/basic/hello.ot` - Hello world
- `examples/basic/exception_basics.ot` - Exception handling basics
- `examples/basic/exception_advanced.ot` - Advanced exceptions
- `examples/basic/exception_resource.ot` - Resource management
- `examples/basic/exception_validation.ot` - Data validation
- `examples/basic/struct_methods_demo.ot` - Struct methods
- `examples/basic/struct_demo.ot` - Struct usage
- `examples/basic/advanced_pipeline.ot` - Complex computation
- `examples/basic/task_benchmark.ot` - Task benchmarks
- `examples/basic/fibonacci.ot` - Fibonacci sequence
- `examples/basic/pythonic_demo.ot` - Pythonic style
- `examples/basic/multiline_test.ot` - Multi-line strings

**FFI Examples:**
- `examples/ffi/ffi_rand_demo.ot` - Random number generation
- `examples/ffi/ffi_rand_advanced.ot` - Advanced FFI usage

## Documentation

- **[Language Specification](docs/LANGUAGE_SPEC.md)** - Complete language reference
- **[Tutorials](docs/TUTORIALS.md)** - Step-by-step guides
- **[API Reference](docs/API_REFERENCE.md)** - Standard library documentation
- **[FFI Guide](docs/FFI_TRANSPARENT.md)** - Using Rust crates from OtterLang

## Status

**Early Access (v0.1.0)** - Experimental, not production-ready.

### Known Limitations

- Type inference is limited (explicit types recommended)
- Module system has some limitations
- Requires LLVM 18 and Rust nightly (for FFI features)

## Contributing

Contributions welcome! See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT License - see [LICENSE](LICENSE).
