/// Basic Banking System
/// A simple banking system implementation in Rust demonstrating traits, structs, and method implementations.
/// Allows creating bank accounts, depositing money, withdrawing money, and checking balances.
/// # Example
/// ```
/// let mut account = BankAccount {
///     balance: 1000,
///     account_number: 123456,
///     holder_name: "John Doe".to_string(),
/// };
///
/// account.deposit(500).unwrap();
///
/// assert_eq!(account.balance, 1500);
/// ```

pub struct BankAccount {
    pub balance: i32,
    pub account_number: u32,
    pub holder_name: String,
}

pub trait Account {
    fn deposit(&mut self, amount: i32) -> Result<(), String>;
    fn withdraw(&mut self, amount: i32) -> Result<(), String>;
    fn balance(&self) -> i32;
}
impl Account for BankAccount {
    fn deposit(&mut self, amount: i32) -> Result<(), String> {
        match amount {
            amount if amount < 0 => Err("Cannot deposit a negative amount".to_string()),
            amount if amount > 250000 => Err("Cannot deposit more than 250000".to_string()),
            _ => {
                self.balance += amount;
                Ok(())
            }
        }
    }

    fn withdraw(&mut self, amount: i32) -> Result<(), String> {
        match amount {
            amount if amount < 0 => Err("Cannot withdraw a negative amount".to_string()),
            amount if amount > 250000 => Err("Cannot withdraw more than 250000".to_string()),
            amount if self.balance < amount => {
                Err("Cannot withdraw more than current balance".to_string())
            }
            _ => {
                self.balance -= amount;
                Ok(())
            }
        }
    }

    fn balance(&self) -> i32 {
        self.balance
    }
}
