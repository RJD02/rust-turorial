#![allow(unused)]

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!(
            "Withdrawing {} from account owned by {}",
            amount, self.owner
        );
        self.balance -= amount;
    }

    fn check_balance(&self) {
        println!(
            "Account owned by {} has balance of {}",
            self.owner, self.balance
        );
    }
}

// At any point a variable can have only one owner(mutable) or many immutable borrowers
// But cannot have both at the same time
// By default every variable in rust is immutable
fn main() {
    let mut account = BankAccount {
        owner: "Raviraj".to_string(),
        balance: 300.34,
    };
    // Immutable borrow to check the balance
    account.check_balance();
    // Mutable borrow to withdraw money
    account.withdraw(150.45);
    // check balance again
    account.check_balance();
}
