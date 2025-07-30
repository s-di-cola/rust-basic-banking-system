mod bank;
use crate::bank::{Account, BankAccount};

fn main() {
    let mut account_di_cola: BankAccount = BankAccount {
        balance: 3000000,
        account_number: 123456,
        holder_name: String::from("Simone Di Cola"),
    };

    let mut account_maggi: BankAccount = BankAccount{
        balance: 50000000,
        account_number: 654321,
        holder_name: String::from("Silvia Maggi"),
    };

    account_di_cola.deposit(39999);
    account_maggi.withdraw(9999);

    println!("Simone's account balance is {}", account_di_cola.balance);
    println!("Silvia Maggi's account balance is {}", account_maggi.balance);
}
