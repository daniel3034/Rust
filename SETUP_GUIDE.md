# Rust Installation and Setup Guide

## Step 1: Install Rust

### For Windows:

1. Download rustup from: https://rustup.rs/
2. Run the installer (rustup-init.exe)
3. Follow the prompts (default installation is recommended)
4. Restart VS Code or your terminal after installation

### Verify Installation:

```powershell
rustc --version
cargo --version
```

You should see version information for both commands.

## Step 2: Install VS Code Extensions

Install these recommended extensions:

1. **rust-analyzer** (rust-lang.rust-analyzer) - The official Rust language server
2. **CodeLLDB** (vadimcn.vscode-lldb) - Debugger for Rust
3. **crates** (serayuzgur.crates) - Helps manage dependencies

## Step 3: Create the Project

Once Rust is installed, run:

```powershell
cargo new password_manager
cd password_manager
```

This creates a new Rust project with the standard structure.

## Step 4: Run the Initial Project

```powershell
cargo run
```

This compiles and runs the default program.

## Common Cargo Commands

- `cargo build` - Compile the project
- `cargo run` - Compile and run the project
- `cargo check` - Check code without producing executable (faster)
- `cargo test` - Run tests
- `cargo doc --open` - Generate and open documentation

## Next Steps

After installation:

1. Review the LEARNING_JOURNAL.md for your progress tracking
2. Read README.md for project overview
3. Start working through Rust Book chapters 1-4
4. Begin implementing the password manager features

## Troubleshooting

### PATH Issues

If `cargo` command isn't recognized after installation:

- Restart your terminal/VS Code
- Manually add to PATH: `%USERPROFILE%\.cargo\bin`

### Permission Issues

- Run VS Code as administrator if needed
- Check antivirus isn't blocking the installation
