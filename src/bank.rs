use crate::account::Account;

pub struct Bank {
    pub accounts: Vec<Account>,
}

impl Bank {
    pub fn new() -> Bank {
        Bank { accounts: vec![] }
    }

    pub fn deposit(&mut self, account_id: u32, balance: i32) {
        for ele in &mut self.accounts {
            if ele.id == account_id {
                ele.balance = ele.balance + balance;
            }
        }
    }

    pub fn withdraw(&mut self, account_id: u32, amount: i32) {
        for ele in &mut self.accounts {
            if ele.id == account_id {
                ele.balance = ele.balance - amount;
            }
        }
    }

    pub fn get_account_owner(&mut self, account_id: u32) -> String {
        let mut holder: String = String::new();
        for ele in &mut self.accounts {
            if ele.id == account_id {
                holder = ele.holder.to_string();
            }
        }
        holder
    }

    pub fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    pub fn get_account_balance(&mut self, account_id: u32) -> i32 {
        let mut balance: i32 = 0;
        for ele in &mut self.accounts {
            if ele.id == account_id {
                balance = ele.balance;
            }
        }
        balance
    }
}
