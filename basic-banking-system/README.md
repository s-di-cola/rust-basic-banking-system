# Banking System with Traits

A simple banking system implementation in Rust demonstrating traits, structs, and method implementations. Allows creating bank accounts, depositing money, withdrawing money, and checking balances.

## What it does

- Creates bank accounts with account numbers, holder names, and balances
- Deposits money into accounts
- Withdraws money from accounts
- Displays account balances
- Uses Rust traits to define common account behavior

## How to run

```bash
cargo run
```

## Example output

```
Account 1001 (Alice Smith) - Balance: $150
Account 1002 (Bob Johnson) - Balance: $75
```

## Key concepts shown

- **Traits**: Defining common behavior (`Account` trait with deposit, withdraw, balance methods)
- **Struct implementation**: `BankAccount` struct implementing the `Account` trait
- **Method calls**: Using trait methods on struct instances
- **Mutable references**: `&mut self` for methods that modify account state
- **Immutable references**: `&self` for methods that only read data

## Code structure

- `Account` trait - defines banking operations interface
- `BankAccount` struct - represents a bank account with account number, holder name, and balance
- Implementation of `Account` trait for `BankAccount`
- Main function demonstrating account creation and transactions

## Files

- `src/main.rs` - Main program demonstrating the banking system
- `src/bank.rs` - Banking module containing Account trait and BankAccount struct