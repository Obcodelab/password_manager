# Simple Password Manager in Rust

## Overview

This project implements a command-line password manager using Rust. The Password Manager allows users to securely store, retrieve, and manage their credentials. It utilizes:

- **HashMap**: For storing credentials as key-value pairs (service-password).
- **File I/O**: To persist credentials to a file (`credentials.json`).
- **Serialization**: To convert the `PasswordManager` struct into JSON format for storage and back for retrieval.
- **Menu-Driven Interface**: For user-friendly operations.
- **Custom Input Module**: Encapsulates functions for reading user input and validating numeric input.

## Features

1. **Add Credential**:

   - Stores a new service-password pair in memory.

2. **Retrieve Credential**:

   - Retrieves the password for a specified service.

3. **Remove Credential**:

   - Deletes the service-password pair from memory.

4. **View All Credentials**:

   - Lists all stored credentials.

5. **Save Credentials to File**:

   - Serializes and saves all credentials to a JSON file.

6. **Load Credentials from File**:

   - Reads and deserializes credentials from a JSON file.

7. **Persistent Storage**:

   - Ensures credentials are saved automatically before exiting.

## Input Module

The program uses a custom input module to handle user input operations:

- `get_input`: Prompts the user for input and returns a trimmed string.

This modular approach keeps input handling separate and reusable, making the codebase cleaner and more maintainable.

## Example User Interaction

```plaintext
 ______________________________
| 1. Add a credential          |
| 2. Retrieve a credential     |
| 3. Remove a credential       |
| 4. Retrieve all credentials  |
| 5. Save credentials          |
| 6. Exit                      |
 ______________________________

Enter your choice: 1
Enter the service: GitHub
Enter the password: my_secure_password
Credential added successfully.

Enter your choice: 2
Enter the service: GitHub
Password for GitHub: my_secure_password

Enter your choice: 6
Credentials saved to credentials.json
```

## Prerequisites

- Rust and Cargo installed. [Install Rust here](https://www.rust-lang.org/tools/install).

## Running the Program

1. Clone the repository:

```sh
git clone https://github.com/Obcodelab/password_manager.git
```

2. Navigate to the project directory:

```sh
cd password_manager
```

3. Run the program:

```sh
cargo run
```

## Future Enhancements

- **Encryption**:

  - Encrypt credentials using a library like aes for enhanced security.

- **Authentication**:

  - Add a master password to protect access to the stored credentials.

- **Backup and Restore**:

  - Implement features for exporting and importing credentials securely.

- **User-Friendly Interface**:

  - Add colored outputs and error messages for a better user experience.
