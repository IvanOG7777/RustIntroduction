use rand::Rng;
use crate::AccountType::{Checking, Savings};
use crate::Transaction::{Deposit, Withdrawal, Transfer};

enum AccountType {
    Checking,
    Savings
}

enum Transaction<'a> {
    // User id # and amount
    Deposit(u32, i32),
    Withdrawal(u32, i32),

    // User id and user id, and amount to transfer
    Transfer(& 'a mut BankAccount, u32, u32, i32)
}
struct BankAccount {
    owner: String,
    id: u32,
    balance: i32,
    account_type: AccountType

}

impl BankAccount {
    fn create_account(name: String, account_type: AccountType,id: u32, init_deposit: Option<i32>) -> BankAccount {
        BankAccount {
            owner: name,
            id,
            balance: init_deposit.unwrap_or(0),
            account_type
        }
    }

    fn print_balance(&self) {
        println!("Balance is: {}", self.balance);
    }

    fn transaction(&mut self, transaction: Transaction) {
        match transaction {
            Deposit(passed_id, deposit_amount) => {
                if passed_id != self.id {
                    println!("Cant process this deposit!");
                    return;
                }

                if deposit_amount <= 0 {
                    println!("Cannot deposit this amount");
                    return;
                }

                self.balance += deposit_amount;
                self.print_balance();
            }

            Withdrawal(passed_id, withdrawal_amount) => {
                if passed_id != self.id {
                    println!("Cant process this withdrawal!");
                    return;
                }

                if withdrawal_amount <= 0 {
                    println!("Cannot withdraw this amount");
                    return;
                }

                if withdrawal_amount <= self.balance {
                    self.balance -= withdrawal_amount;
                    self.print_balance();
                } else {
                    println!("Insufficient funds");
                    return;
                }
            }

            Transfer(to_bank_account, to_id, from_id, transfer_amount) => {
                if from_id != self.id {
                    println!("Cant process this transaction");
                    return;
                }

                if transfer_amount <= 0 {
                    println!("Cannot transfer this amount");
                    return;
                }

                if transfer_amount <= self.balance {
                    println!("Transferring {} to account {}", transfer_amount, to_id);
                    self.balance -= transfer_amount;
                     to_bank_account.transaction(Deposit(to_id, transfer_amount));
                    self.print_balance();
                } else {
                    println!("Insufficient funds");
                    return;
                }
            }
        }
    }

    fn show_bank_details(&self) {
        println!("Account holder: {}", self.owner);
        println!("Account ID: {}", self.id);
        println!("Account type: {}", match self.account_type {
            Checking => "Checking",
            Savings => "Savings"
        });
        println!("Balance: {}", self.balance);
    }
}
fn main() {

    let mut bank_account1 = BankAccount::create_account(String::from("Ivan Argueta"), Savings, 10, Option::Some(10));
    let mut bank_account2 = BankAccount::create_account(String::from("Joe shmoe"), Checking, 11, Option::None);

    bank_account1.show_bank_details();
    println!();
    bank_account2.show_bank_details();
    println!();

    bank_account1.transaction(Deposit(10, 100));
    bank_account2.transaction(Withdrawal(11, -100));

    bank_account1.transaction(Transfer(&mut bank_account2, 11, 10, 50));
    println!();
    bank_account2.show_bank_details();


}