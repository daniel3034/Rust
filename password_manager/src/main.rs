// Password Manager - Main Entry Point
// Author: Daniel Holguin Delgado

use std::collections::HashMap;
use std::io::{self, Write};

// TODO: Once you learn about modules (Rust Book Ch 7), 
// you can move these into separate files:
// mod password;
// mod storage;
// mod auth;
// mod crypto;
// mod ui;

/// Represents a single password entry
/// Demonstrates: struct definition (Rust Book Ch 5)
#[derive(Debug, Clone)]
struct Password {
    service: String,
    username: String,
    password: String,
    created_at: String,
}

impl Password {
    /// Creates a new Password instance
    /// Demonstrates: ownership - function takes ownership of Strings
    fn new(service: String, username: String, password: String) -> Password {
        Password {
            service,
            username,
            password,
            created_at: get_timestamp(),
        }
    }

    /// Validates password strength
    /// Demonstrates: borrowing - function borrows &self (immutable reference)
    fn validate_strength(&self) -> (bool, String) {
        let mut issues = Vec::new();
        
        if self.password.len() < 8 {
            issues.push("at least 8 characters");
        }
        if !self.password.chars().any(|c| c.is_uppercase()) {
            issues.push("an uppercase letter");
        }
        if !self.password.chars().any(|c| c.is_lowercase()) {
            issues.push("a lowercase letter");
        }
        if !self.password.chars().any(|c| c.is_numeric()) {
            issues.push("a number");
        }
        if !self.password.chars().any(|c| !c.is_alphanumeric()) {
            issues.push("a special character");
        }

        if issues.is_empty() {
            (true, "Strong password!".to_string())
        } else {
            (false, format!("Weak password. Needs: {}", issues.join(", ")))
        }
    }

    /// Displays password information (for demonstration purposes)
    /// In a real app, you wouldn't display the actual password
    /// Demonstrates: borrowing with &self
    fn display(&self) {
        println!("\n--- Password Entry ---");
        println!("Service:    {}", self.service);
        println!("Username:   {}", self.username);
        println!("Password:   {}", self.password);
        println!("Created:    {}", self.created_at);
        
        let (_is_strong, message) = self.validate_strength();
        println!("Strength:   {}", message);
        println!("--------------------\n");
    }
}

/// Main password manager structure
/// Demonstrates: HashMap data structure (Rust Book Ch 8)
struct PasswordManager {
    passwords: HashMap<String, Password>,
    master_password: String,
    is_authenticated: bool,
}

impl PasswordManager {
    /// Creates a new PasswordManager
    /// Demonstrates: associated function (like static method)
    fn new() -> PasswordManager {
        PasswordManager {
            passwords: HashMap::new(),
            master_password: String::new(),
            is_authenticated: false,
        }
    }

    /// Sets up the master password
    /// Demonstrates: mutable borrowing with &mut self
    fn setup_master_password(&mut self) {
        println!("\n=== Master Password Setup ===");
        println!("This password will protect all your stored passwords.");
        
        let password = get_password_input("Create master password: ");
        let confirm = get_password_input("Confirm master password: ");
        
        if password == confirm {
            self.master_password = password;
            self.is_authenticated = true;
            println!("✓ Master password set successfully!\n");
        } else {
            println!("✗ Passwords don't match. Please try again.\n");
            self.setup_master_password(); // Recursive call
        }
    }

    /// Authenticates the user
    /// Demonstrates: conditionals and boolean expressions
    /// Note: Currently using setup_master_password for initial auth
    #[allow(dead_code)]
    fn authenticate(&mut self) -> bool {
        println!("\n=== Authentication Required ===");
        let input = get_password_input("Enter master password: ");
        
        // TODO: In a real app, you'd use password hashing (sha2 crate)
        // For learning purposes, we're using plain text comparison
        if input == self.master_password {
            self.is_authenticated = true;
            println!("✓ Authentication successful!\n");
            true
        } else {
            println!("✗ Incorrect password!\n");
            false
        }
    }

    /// Adds a new password entry
    /// Demonstrates: mutable borrowing, HashMap insert
    fn add_password(&mut self) {
        println!("\n=== Add New Password ===");
        
        let service = get_input("Service name (e.g., GitHub, Gmail): ");
        
        // Check if service already exists
        if self.passwords.contains_key(&service) {
            println!("✗ A password for '{}' already exists. Use Update instead.", service);
            return;
        }
        
        let username = get_input("Username/Email: ");
        let password = get_password_input("Password: ");
        
        let entry = Password::new(service.clone(), username, password);
        
        // Validate password strength
        let (is_strong, message) = entry.validate_strength();
        println!("{}", message);
        
        if !is_strong {
            let confirm = get_input("Add anyway? (y/n): ");
            if confirm.to_lowercase() != "y" {
                println!("Password not saved.");
                return;
            }
        }
        
        // Insert into HashMap
        self.passwords.insert(service.clone(), entry);
        println!("✓ Password for '{}' saved successfully!\n", service);
    }

    /// Views a stored password
    /// Demonstrates: HashMap get, Option handling
    fn view_password(&self) {
        println!("\n=== View Password ===");
        let service = get_input("Service name: ");
        
        // Option<&Password> - might be Some(password) or None
        match self.passwords.get(&service) {
            Some(password) => password.display(),
            None => println!("✗ No password found for '{}'.\n", service),
        }
    }

    /// Updates an existing password
    /// Demonstrates: HashMap get_mut for mutable access
    fn update_password(&mut self) {
        println!("\n=== Update Password ===");
        let service = get_input("Service name: ");
        
        if self.passwords.contains_key(&service) {
            let new_password = get_password_input("New password: ");
            let new_username = get_input("New username (press Enter to keep current): ");
            
            // Get mutable reference to update the entry
            if let Some(entry) = self.passwords.get_mut(&service) {
                entry.password = new_password;
                if !new_username.trim().is_empty() {
                    entry.username = new_username;
                }
                entry.created_at = get_timestamp(); // Update timestamp
                
                println!("✓ Password for '{}' updated successfully!\n", service);
            }
        } else {
            println!("✗ No password found for '{}'.\n", service);
        }
    }

    /// Deletes a password entry
    /// Demonstrates: HashMap remove
    fn delete_password(&mut self) {
        println!("\n=== Delete Password ===");
        let service = get_input("Service name: ");
        
        match self.passwords.remove(&service) {
            Some(_) => println!("✓ Password for '{}' deleted successfully!\n", service),
            None => println!("✗ No password found for '{}'.\n", service),
        }
    }

    /// Lists all stored service names
    /// Demonstrates: iteration over HashMap keys
    fn list_all(&self) {
        println!("\n=== Stored Services ===");
        
        if self.passwords.is_empty() {
            println!("No passwords stored yet.\n");
            return;
        }
        
        let mut services: Vec<&String> = self.passwords.keys().collect();
        services.sort(); // Sort alphabetically
        
        for (i, service) in services.iter().enumerate() {
            println!("{}. {}", i + 1, service);
        }
        println!("Total: {} password(s)\n", services.len());
    }

    /// Searches for passwords by service name
    /// Demonstrates: string operations, filtering
    fn search_passwords(&self) {
        println!("\n=== Search Passwords ===");
        let query = get_input("Search for service: ").to_lowercase();
        
        let results: Vec<&String> = self.passwords.keys()
            .filter(|service| service.to_lowercase().contains(&query))
            .collect();
        
        if results.is_empty() {
            println!("No services found matching '{}'.\n", query);
        } else {
            println!("Found {} match(es):", results.len());
            for service in results {
                println!("  - {}", service);
            }
            println!();
        }
    }
}

/// Displays the main menu
/// Demonstrates: simple function, no ownership concerns
fn display_menu() {
    println!("╔════════════════════════════════════╗");
    println!("║    PASSWORD MANAGER - Main Menu    ║");
    println!("╠════════════════════════════════════╣");
    println!("║ 1. Add Password                    ║");
    println!("║ 2. View Password                   ║");
    println!("║ 3. Update Password                 ║");
    println!("║ 4. Delete Password                 ║");
    println!("║ 5. List All Services               ║");
    println!("║ 6. Search Passwords                ║");
    println!("║ 7. Exit                            ║");
    println!("╚════════════════════════════════════╝");
}

/// Gets user input from stdin
/// Demonstrates: String ownership, error handling
fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // Ensure prompt is displayed
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    
    input.trim().to_string()
}

/// Gets password input (in a real app, this would hide input)
/// Demonstrates: function reuse
fn get_password_input(prompt: &str) -> String {
    // TODO: Use a crate like 'rpassword' to hide password input
    // For learning purposes, we'll use regular input
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    
    input.trim().to_string()
}

/// Gets current timestamp as string
/// Demonstrates: String creation
fn get_timestamp() -> String {
    // TODO: Use chrono crate for proper timestamp
    // For now, return a placeholder
    "2025-11-09".to_string()
}

/// Main function - entry point
/// Demonstrates: main loop, pattern matching, program flow
fn main() {
    println!("\n╔════════════════════════════════════════════╗");
    println!("║  Welcome to Rust Password Manager v0.1     ║");
    println!("║               Rust Language                ║");
    println!("╚════════════════════════════════════════════╝\n");

    // Create password manager instance
    // Demonstrates: mutable variable
    let mut manager = PasswordManager::new();

    // Setup master password
    manager.setup_master_password();

    // Main program loop
    // Demonstrates: infinite loop with break condition
    loop {
        display_menu();
        let choice = get_input("Enter your choice (1-7): ");

        // Pattern matching on user input
        // Demonstrates: match expression, conditionals
        match choice.as_str() {
            "1" => {
                if manager.is_authenticated {
                    manager.add_password();
                } else {
                    println!("Please authenticate first.\n");
                }
            }
            "2" => {
                if manager.is_authenticated {
                    manager.view_password();
                } else {
                    println!("Please authenticate first.\n");
                }
            }
            "3" => {
                if manager.is_authenticated {
                    manager.update_password();
                } else {
                    println!("Please authenticate first.\n");
                }
            }
            "4" => {
                if manager.is_authenticated {
                    manager.delete_password();
                } else {
                    println!("Please authenticate first.\n");
                }
            }
            "5" => {
                if manager.is_authenticated {
                    manager.list_all();
                } else {
                    println!("Please authenticate first.\n");
                }
            }
            "6" => {
                if manager.is_authenticated {
                    manager.search_passwords();
                } else {
                    println!("Please authenticate first.\n");
                }
            }
            "7" => {
                println!("\n👋 Thank you for using Password Manager!");
                println!("Stay secure! 🔒\n");
                break; // Exit the loop
            }
            _ => {
                println!("✗ Invalid choice. Please enter 1-7.\n");
            }
        }
    }
}
