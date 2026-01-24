# Anchor PDA Program

A basic Solana program built with the Anchor framework that demonstrates Program Derived Address (PDA) creation and management.

## Overview

This project showcases how to create and manage PDA accounts using Anchor's declarative syntax. The program initializes a PDA account that is derived from a combination of a static seed ("client1") and the signer's public key.

## Features

- **PDA Creation**: Creates a Program Derived Address using seeds
- **Account Initialization**: Initializes a new account with custom data structure
- **Anchor Framework**: Built using the Anchor framework for simplified Solana development
- **TypeScript Client**: Includes TypeScript test client for interacting with the program

## Project Structure

```
.
├── programs/
│   └── anchor-pda/           # Rust program source
│       ├── src/lib.rs        # Main program logic
│       ├── Cargo.toml        # Rust dependencies
│       └── Xargo.toml        # Build configuration
├── tests/
│   └── anchor-pda.ts         # TypeScript test client
├── Anchor.toml              # Anchor framework configuration
├── Cargo.toml               # Workspace configuration
├── package.json             # TypeScript dependencies
└── README.md                # This file
```

## Program Details

### Account Structure

The program creates a `StakeAccount` with the following structure:
- `num: u32` - A 32-bit unsigned integer field

### PDA Derivation

The PDA is derived using these seeds:
- `"client1"` - Static seed for namespace separation
- `signer.key().as_ref()` - Dynamic seed based on user's public key

This ensures each user gets their own unique PDA account.

### Account Space

The PDA account is allocated with 49 bytes:
- 8 bytes for account discriminator
- 32 bytes for the `num` field
- 8 bytes for additional data
- 1 byte for boolean flags

## Development Setup

### Prerequisites

- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor Framework](https://book.anchor-lang.com/getting_started/installation.html)
- [Node.js](https://nodejs.org/) (for TypeScript client)

### Installation

1. Install dependencies:
   ```bash
   yarn install
   ```

2. Build the program:
   ```bash
   anchor build
   ```

3. Deploy to localnet:
   ```bash
   anchor deploy
   ```

## Usage

### Running Tests

Execute the TypeScript test suite:
```bash
anchor test
```

This will:
- Start a local Solana validator
- Deploy the program
- Run the initialization test

### Manual Testing

The test client demonstrates how to:
1. Set up an Anchor provider
2. Initialize the PDA account
3. Verify transaction success

## Program ID

The program is deployed with the following ID:
```
FhAa7amenGbqaDkipL9oECcx7RCPJ9rAoxbdKQNNdbqm
```

## Key Concepts

### Program Derived Addresses (PDAs)

PDAs are special addresses that:
- Are derived from program ID and seeds
- Cannot sign transactions (no private key)
- Are controlled by the program that created them
- Provide secure, deterministic address generation

### Anchor Framework Benefits

- **Declarative Accounts**: Define account constraints in structs
- **Automatic Serialization**: Handle data serialization/deserialization
- **Safety**: Built-in security checks and validation
- **Type Safety**: Strong typing for both Rust and TypeScript

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Submit a pull request

## License

This project is licensed under the ISC License.

## Resources

- [Anchor Documentation](https://book.anchor-lang.com/)
- [Solana Documentation](https://docs.solana.com/)
- [Program Derived Addresses Guide](https://docs.solana.com/developing/programming-model/calling-between-programs#hash-based-generated-program-addresses)