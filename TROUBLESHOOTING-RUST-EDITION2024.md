# Troubleshooting Rust Edition2024 Issues

This document provides solutions for common issues related to the Rust edition2024 requirement in the DECENTRALIZED-APP project.

## Problem: `failed to parse manifest` with `edition2024` error

### Error Message:
```
error: failed to download `base64ct v1.8.0`

Caused by:
  unable to get packages from source       

Caused by:
  failed to parse manifest at `...`

Caused by:
  feature `edition2024` is required

  The package requires the Cargo feature called `edition2024`, but that feature is not stabilized in this version of Cargo (1.80.1).
  Consider trying a newer version of Cargo (this may require the nightly release).
```

## Solution 1: Install and Use Rust Nightly (Recommended)

### For Windows (PowerShell):
```powershell
# Install nightly toolchain
rustup install nightly

# Set nightly as default
rustup default nightly

# Verify installation
rustc --version
# Should show something like: rustc 1.xx.x-nightly (xxxxxxxxx yyyy-mm-dd)

# Try building again
cargo build
```

### For Linux/macOS:
```bash
# Install nightly toolchain
rustup install nightly

# Set nightly as default
rustup default nightly

# Verify installation
rustc --version
# Should show something like: rustc 1.xx.x-nightly (xxxxxxxxx yyyy-mm-dd)

# Try building again
cargo build
```

## Solution 2: Use Nightly Toolchain Explicitly

If you don't want to change your default toolchain, you can use the nightly version explicitly:

### For all cargo commands:
```bash
# Build with nightly
cargo +nightly build

# Run with nightly
cargo +nightly run

# Test with nightly
cargo +nightly test
```

### For Windows PowerShell:
```powershell
# Build with nightly
cargo +nightly build

# Run with nightly
cargo +nightly run

# Test with nightly
cargo +nightly test
```

## Solution 3: Update rust-toolchain.toml

Ensure your `rust-toolchain.toml` file contains:
```toml
[toolchain]
channel = "nightly"
components = ["rustfmt", "clippy"]
```

## Solution 4: Verify Installation

After installing nightly, verify everything works:

```bash
# Check toolchain
rustup show

# Check specific components
rustup component list --toolchain nightly

# Install missing components if needed
rustup component add rustfmt --toolchain nightly
rustup component add clippy --toolchain nightly
```

## Solution 5: Clean and Rebuild

Sometimes a clean rebuild helps:

```bash
# Clean build artifacts
cargo clean

# Rebuild with nightly
cargo +nightly build
```

## Additional Tips

### 1. IDE Configuration
If you're using an IDE like VS Code with the Rust extension, make sure it's configured to use the nightly toolchain:
- Open VS Code settings (Ctrl+,)
- Search for "rust"
- Set "rust-client.channel" to "nightly"

### 2. Shell Integration
Add to your shell profile (`.bashrc`, `.zshrc`, etc.) for persistent settings:
```bash
# Set nightly as default (optional)
export RUSTUP_TOOLCHAIN=nightly
```

### 3. Project-specific Toolchain
The project includes a `rust-toolchain.toml` file that should automatically use the correct toolchain when you run cargo commands from within the project directory.

## Common Issues and Solutions

### Issue: "toolchain 'nightly' is not installed"
```bash
# Solution: Install it explicitly
rustup install nightly-x86_64-unknown-linux-gnu  # Linux
rustup install nightly-x86_64-pc-windows-msvc    # Windows
rustup install nightly-x86_64-apple-darwin        # macOS
```

### Issue: "error: toolchain 'nightly' does not contain component"
```bash
# Solution: Install missing components
rustup component add rustfmt --toolchain nightly
rustup component add clippy --toolchain nightly
```

### Issue: Permission errors on Windows
```powershell
# Solution: Run as Administrator or use:
rustup install nightly --force-non-host
```

## Verifying the Fix

After applying the solution, verify it works:

```bash
# Check version
rustc --version

# Run project verification script
./verify-build.ps1  # Windows
./verify-build.sh   # Linux/macOS

# Or manually check
cargo +nightly check
cargo +nightly build --bin dex-cli
```

If everything works correctly, you should be able to run the installation script successfully:

```bash
# Windows
.\install.ps1

# Linux/macOS
./install.sh
```

## Need More Help?

If you continue to experience issues:

1. Check the [Rust Edition 2024 documentation](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#edition-2024)
2. Visit the [Rust Discord](https://discord.gg/rust-lang) or [users forum](https://users.rust-lang.org/)
3. File an issue on the project repository with:
   - Your operating system
   - Rust version (`rustc --version`)
   - Error message
   - Steps you've tried