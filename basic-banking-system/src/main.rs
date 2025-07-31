mod bank;
use crate::bank::{Account, BankAccount};

fn main() {
    let mut account_di_cola: BankAccount = BankAccount {
        balance: 3000000,
        account_number: 123456,
        holder_name: String::from("Simone Di Cola"),
    };

    let mut account_maggi: BankAccount = BankAccount {
        balance: 50000000,
        account_number: 654321,
        holder_name: String::from("Silvia Maggi"),
    };

    match account_di_cola.deposit(39999) {
        Ok(()) => println!(
            "Successfully deposited to {}",
            account_di_cola.account_number
        ),
        Err(msg) => println!("Error while depositing: {}", msg),
    }

    account_maggi.withdraw(300000).unwrap_or_else(|err| {
        println!("Error while withdrawing from  {}", err);
    });

    println!("Simone's account balance is {}", account_di_cola.balance);
    println!(
        "Silvia Maggi's account balance is {}",
        account_maggi.balance
    );
}
