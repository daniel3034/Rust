# Command-Line Password Manager

**Author:** Daniel Holguin Delgado  
**Date:** October 28, 2025

## Project Overview

A secure command-line password manager built in Rust that demonstrates memory safety, ownership concepts, and practical systems programming. This project allows users to store, retrieve, and manage passwords through an interactive CLI interface.

## Learning Objectives

This project fulfills the Rust language module requirements by demonstrating:

### Core Language Features

- ✅ **Variables**: Immutable and mutable variables for security-critical data
- ✅ **Expressions**: Password strength calculations, encryption operations, string manipulations
- ✅ **Conditionals**: Menu navigation, password validation, authentication checks
- ✅ **Loops**: Main program loop, password generation iterations, menu systems
- ✅ **Functions**: CRUD operations demonstrating borrowing vs. ownership
- ✅ **Object-Oriented Techniques**: `struct Password` with `impl` blocks for methods
- ✅ **Data Structures**: `HashMap<String, Password>` for password storage

### Rust-Specific Concepts

- Ownership and borrowing
- Memory safety without garbage collection
- Error handling with `Result` and `Option`
- Pattern matching
- Module organization

## Features

### Core Functionality

- ✅ Master password authentication
- ✅ Add new password entries
- ✅ View stored passwords
- ✅ Update existing passwords
- ✅ Delete password entries
- ✅ Search for passwords by service name
- ✅ Password strength validation
- ✅ Basic encryption for stored passwords

### Security Features

- Master password protection
- Password encryption at rest
- Password strength validation
- Secure memory handling (Rust ownership)

## Prerequisites

### Required Software

- **Rust** (rustc 1.70+ recommended)
- **Cargo** (comes with Rust)
- **VS Code** (recommended) with extensions:
  - rust-analyzer
  - CodeLLDB (for debugging)
  - crates (for dependency management)

### Installation

1. **Install Rust:**

   - Visit https://rustup.rs/
   - Download and run rustup-init.exe
   - Follow installation prompts
   - Restart terminal after installation

2. **Verify Installation:**
   ```powershell
   rustc --version
   cargo --version
   ```

For detailed setup instructions, see [SETUP_GUIDE.md](SETUP_GUIDE.md).

## Project Structure

```
password_manager/
├── Cargo.toml              # Project configuration and dependencies
├── Cargo.lock              # Dependency lock file
├── src/
│   ├── main.rs             # Entry point and main loop
│   ├── password.rs         # Password struct and methods
│   ├── storage.rs          # HashMap storage management
│   ├── auth.rs             # Master password authentication
│   ├── crypto.rs           # Encryption/decryption
│   └── ui.rs               # User interface and menu
├── README.md               # This file
└── SETUP_GUIDE.md          # Installation instructions
```

## Getting Started

### 1. Create the Project

```powershell
cargo new password_manager
cd password_manager
```

### 2. Build the Project

```powershell
cargo build
```

### 3. Run the Application

```powershell
cargo run
```

### 4. Run Tests

```powershell
cargo test
```

## Usage

### Initial Setup

1. Launch the application: `cargo run`
2. Create a master password when prompted
3. Re-enter master password to authenticate

### Main Menu Options

```
1. Add Password    - Store a new password entry
2. View Password   - Display a stored password
3. Update Password - Modify an existing entry
4. Delete Password - Remove a password entry
5. List All        - Show all service names
6. Search          - Find passwords by service name
7. Exit            - Close the application
```

### Adding a Password

```
Enter service name: GitHub
Enter username: user@example.com
Enter password: ********
```

### Password Strength Validation

The application validates password strength based on:

- Minimum 8 characters
- Contains uppercase letters
- Contains lowercase letters
- Contains numbers
- Contains special characters

## Code Examples

### Password Struct

```rust
// Represents a single password entry
struct Password {
    service: String,
    username: String,
    password: String,
    created_at: String,
}

impl Password {
    // Constructor demonstrating ownership
    fn new(service: String, username: String, password: String) -> Password {
        Password {
            service,
            username,
            password,
            created_at: get_timestamp(),
        }
    }

    // Method demonstrating borrowing
    fn validate_strength(&self) -> bool {
        // Password validation logic
    }
}
```

### Storage with HashMap

```rust
use std::collections::HashMap;

struct PasswordManager {
    passwords: HashMap<String, Password>,
    master_password_hash: String,
}

impl PasswordManager {
    fn add_password(&mut self, service: String, password: Password) {
        self.passwords.insert(service, password);
    }

    fn get_password(&self, service: &str) -> Option<&Password> {
        self.passwords.get(service)
    }
}
```

## Requirements Coverage

### Variables (Mutable/Immutable)

```rust
let master_password: String = get_input();  // Immutable
let mut passwords = HashMap::new();          // Mutable for updates
```

### Expressions & Conditionals

```rust
let strength = if password.len() >= 8 && has_special_chars(&password) {
    "Strong"
} else {
    "Weak"
};
```

### Loops

```rust
loop {
    display_menu();
    match get_user_choice() {
        1 => add_password(),
        7 => break,
        _ => println!("Invalid option"),
    }
}
```

### Functions & Borrowing

```rust
// Takes ownership
fn encrypt_password(password: String) -> String { ... }

// Borrows immutably
fn validate_password(password: &str) -> bool { ... }

// Borrows mutably
fn update_password(password: &mut Password) { ... }
```

## Development Schedule

### Week 1: Learning & Foundation

- **Day 1 (2h)**: Environment setup, Rust installation
- **Day 2 (3h)**: Rust Book Ch 1-4 (basics, variables, functions)
- **Day 3 (3h)**: Rust Book Ch 5-8 (structs, ownership, collections)
- **Day 4 (2h)**: Basic CLI structure and menu
- **Day 5 (2h)**: Master password authentication
- **Day 6 (2h)**: Password struct and methods

### Week 2: Implementation & Polish

- **Day 7 (3h)**: CRUD operations with HashMap
- **Day 8 (3h)**: Encryption/decryption functionality
- **Day 9 (2h)**: Search and password validation
- **Day 10 (2h)**: Testing, debugging, cleanup
- **Day 11 (2h)**: Documentation and README
- **Day 12 (1h)**: Demo video and GitHub publication

**Total Time**: ~24 hours (12+ hours on project implementation)

## Risk Management

### Risk 1: Ownership and Borrowing Concepts

**Mitigation:**

- Extra study time on Rust Book chapters 4-5
- Complete Rustlings exercises
- Start with simple examples before complex operations
- Use compiler messages as learning opportunities

### Risk 2: Time Management

**Mitigation:**

- Begin with MVP (basic storage without encryption)
- Implement features incrementally
- Test frequently
- Have backup simpler features if needed

## Learning Resources

- 📚 [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- 🏋️ [Rustlings Exercises](https://github.com/rust-lang/rustlings)
- 📖 [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- 🦀 [Rust Standard Library Docs](https://doc.rust-lang.org/std/)
- 💬 [Rust Community Discord](https://discord.gg/rust-lang)

## Testing

Run tests with:

```powershell
cargo test
```

Run with output:

```powershell
cargo test -- --nocapture
```

## Building for Release

```powershell
cargo build --release
```

The optimized binary will be in `target/release/`.

## Dependencies

```toml
[dependencies]
# Will be added as needed, potentially:
# sha2 = "0.10"           # For password hashing
# aes = "0.8"             # For encryption
# chrono = "0.4"          # For timestamps
```

## Common Issues & Solutions

### Compiler Error: "value borrowed after move"

This is Rust's ownership system at work. Review who owns the data and consider:

- Using references (`&`) for borrowing
- Cloning data when needed (`.clone()`)
- Restructuring to avoid the move

### Compiler Error: "cannot borrow as mutable"

- Ensure variable is declared with `mut`
- Check if you're trying to have multiple mutable borrows
- Review borrowing rules

## Submission Checklist

- [ ] All code compiles without errors
- [ ] All features implemented and tested
- [ ] Code is well-commented
- [ ] README.md is complete
- [ ] 4-5 minute demonstration video recorded
- [ ] All files pushed to GitHub
- [ ] Repository is public or shared with instructor

## License

This project is created for educational.

## Contact

Daniel Holguin Delgado 

---

**Video**
https://www.youtube.com/watch?v=aEh6VyhOGjs

**Note:** This is a learning project. While it demonstrates encryption concepts, it should NOT be used for storing real sensitive passwords in a production environment.
