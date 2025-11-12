# Getting Started with Password Manager

## Quick Start Guide

### Step 1: Install Rust

1. Visit https://rustup.rs/
2. Download and run the installer
3. Restart VS Code after installation
4. Verify installation:
   ```powershell
   rustc --version
   cargo --version
   ```

### Step 2: Navigate to Project

```powershell
cd "location of the project"
```

### Step 3: Build and Run

```powershell
# Build the project
cargo build

# Run the application
cargo run
```

### Step 4: Test the Application

1. When prompted, create a master password
2. Try adding a password:
   - Service: GitHub
   - Username: your_email@example.com
   - Password: Test123!@#
3. View the password you just added
4. Try the other menu options

## What You'll See

```
╔════════════════════════════════════════════╗
║  Welcome to Rust Password Manager v0.1     ║
║              Rust Language                 ║
╚════════════════════════════════════════════╝

=== Master Password Setup ===
This password will protect all your stored passwords.
Create master password:
```

## Project Files Created

✅ **README.md** - Comprehensive project documentation  
✅ **SETUP_GUIDE.md** - Installation instructions  
✅ **password_manager/Cargo.toml** - Project configuration  
✅ **password_manager/src/main.rs** - Complete working code

## Code Features Already Implemented

The `main.rs` file includes:

### ✅ All Required Language Features

- Variables (mutable and immutable)
- Expressions and operators
- Conditionals (if/else, match)
- Loops (loop, for)
- Functions (with borrowing examples)
- Structs and impl blocks
- HashMap data structure

### ✅ All Core Functionality

- Master password setup
- Add password entries
- View passwords
- Update passwords
- Delete passwords
- List all services
- Search functionality
- Password strength validation

### ✅ Learning-Friendly Code

- Extensive comments explaining Rust concepts
- References to specific Rust Book chapters
- TODO markers for future enhancements
- Clear demonstrations of ownership and borrowing

## Learning Path

### Week 1: Foundation

1. **Monday** - Install Rust, run the project
2. **Tuesday** - Study Rust Book Ch 1-4, understand variables and functions in the code
3. **Wednesday** - Study Ch 5-8, understand structs and HashMap
4. **Thursday** - Experiment with the menu system
5. **Friday** - Understand the authentication flow
6. **Saturday** - Study the Password struct implementation

### Week 2: Enhancement

1. **Monday** - Already have CRUD operations! Study how they work
2. **Tuesday** - Add encryption (implement the TODO)
3. **Wednesday** - Enhance search and validation
4. **Thursday** - Test and debug
5. **Friday** - Write documentation
6. **Saturday** - Record video and publish

## Next Steps

1. **Install Rust** using the setup guide
2. **Run the project** with `cargo run`
3. **Read through main.rs** and follow the comments
4. **Try adding a feature** - maybe a password generator?

## Tips for Success

- Don't worry about understanding everything at once
- The Rust compiler is your friend - read error messages carefully

## Getting Help

- **Rust Book**: https://doc.rust-lang.org/book/
- **Rust by Example**: https://doc.rust-lang.org/rust-by-example/
- **Rustlings**: https://github.com/rust-lang/rustlings
- **Stack Overflow**: Tag your questions with `rust`

## What Makes This Code Great for Learning

1. **Complete Working Example** - You can run it immediately
2. **Real-World Project** - Practical password manager
3. **Educational Comments** - Every concept is explained
4. **Progressive Complexity** - Simple concepts first, advanced later
5. **Clear TODOs** - Know exactly what to enhance next
6. **Rust Best Practices** - Demonstrates idiomatic Rust code

