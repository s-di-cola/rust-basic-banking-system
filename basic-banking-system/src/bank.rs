pub struct BankAccount {
    pub balance: i32,
    pub account_number: u32,
    pub holder_name: String,
}

pub trait Account {
    fn deposit(&mut self, amount: i32);
    fn withdraw(&mut self, amount: i32);
    fn balance(&self) -> i32;
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: i32) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: i32) {
        self.balance -= amount;
    }

    fn balance(&self) -> i32 {
        self.balance
    }
}