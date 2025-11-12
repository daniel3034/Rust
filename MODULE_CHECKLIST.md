# Module #1 Requirements Checklist

## ✅ Basic Requirements (All Required)

### 1. Variables (mutable and immutable)

- [ ] **Location in code**: Lines 162-163, 328, etc.
- [ ] **Example**: `let master_password: String` (immutable), `let mut passwords = HashMap::new()` (mutable)
- [ ] **Status**: ✅ IMPLEMENTED

### 2. Expressions

- [ ] **Location in code**: Lines 42-57 (password validation), 242-251 (conditionals)
- [ ] **Example**: `self.password.len() < 8`, `services.len()`
- [ ] **Status**: ✅ IMPLEMENTED

### 3. Conditionals

- [ ] **Location in code**: Lines 177-184 (if/else), 356-391 (match statement)
- [ ] **Example**: `if password == confirm`, `match choice.as_str()`
- [ ] **Status**: ✅ IMPLEMENTED

### 4. Loops

- [ ] **Location in code**: Lines 349-392 (main loop), 268-272 (for loop)
- [ ] **Example**: `loop { ... break; }`, `for (i, service) in services.iter().enumerate()`
- [ ] **Status**: ✅ IMPLEMENTED

### 5. Functions (ownership or reference)

- [ ] **Location in code**: Multiple functions throughout
- [ ] **Ownership Example**: `fn new(service: String, ...)` - takes ownership (Line 28)
- [ ] **Reference Example**: `fn validate_strength(&self)` - borrows immutably (Line 38)
- [ ] **Mutable Reference**: `fn add_password(&mut self)` - borrows mutably (Line 197)
- [ ] **Status**: ✅ IMPLEMENTED

## ✅ Additional Requirements (Choose One)

### Option 1: Data Structure ✅ CHOSEN

- [ ] **Type Used**: `HashMap<String, Password>`
- [ ] **Location**: Lines 93-95, 163, 209
- [ ] **Operations Demonstrated**:
  - Insert: Line 209
  - Get: Line 225
  - Get Mutable: Line 240
  - Remove: Line 252
  - Keys iteration: Line 265
- [ ] **Status**: ✅ FULLY IMPLEMENTED

### Option 2: Object-Oriented Techniques ✅ BONUS (Also Implemented!)

- [ ] **Struct Definition**: Lines 18-23 (`struct Password`)
- [ ] **Impl Block**: Lines 25-67 (`impl Password`)
- [ ] **Methods**: `new()`, `validate_strength()`, `display()`
- [ ] **Second Struct**: Lines 93-96 (`struct PasswordManager`)
- [ ] **Second Impl**: Lines 98-287 (`impl PasswordManager`)
- [ ] **Status**: ✅ FULLY IMPLEMENTED

## 📋 Submission Requirements

### Code Quality

- [ ] **Minimum 100 Lines**: ✅ YES (406 lines)
- [ ] **Useful Comments**: ✅ YES (extensive documentation)
- [ ] **Functions Documented**: ✅ YES (all functions have /// comments)

## 🎯 Your Current Status

### ✅ COMPLETED

1. Project structure created
2. All basic requirements implemented
3. Data structure (HashMap) implemented
4. Object-oriented techniques implemented
5. Code exceeds 100 lines (396 lines!)
6. Comprehensive comments added
7. README.md created and populated
8. Learning journal template created

### Code Walkthrough (2-3 minutes)

Point out and explain:

1. **Structs** (Lines 18-23, 93-96) - "Here's the Password struct..."
2. **HashMap** (Line 94) - "I use a HashMap to store passwords..."
3. **Ownership** (Line 28) - "This function takes ownership of the strings..."
4. **Borrowing** (Line 38) - "This uses an immutable reference..."
5. **Mutable Borrowing** (Line 197) - "Here I need mutable access..."
6. **Match Statement** (Lines 356-391) - "The main loop uses pattern matching..."
7. **Loops** (Line 349) - "The infinite loop continues until user exits..."

## 🚀 Quick Start Commands

```powershell
# 1. Navigate to project
cd "location of the project"

# 2. Build project
cargo build

# 3. Run project
cargo run

# 4. Check for errors
cargo check

# 5. Run tests (when you add them)
cargo test
```
