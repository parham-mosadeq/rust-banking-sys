use std::ptr::NonNull;

#[derive(Debug, Clone)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Account {
    fn new(id: u32, holder: String) -> Account {
        Account {
            id,
            holder,
            balance: 0,
        }
    }
}

impl Bank {
    fn new() -> Bank {
        Bank { accounts: vec![] }
    }

    fn deposit(&mut self, account_id: u32, balance: i32) {
        for ele in &mut self.accounts {
            if ele.id == account_id {
                ele.balance = balance;
            }
        }
    }

    fn get_account_owner(&mut self, account_id: u32) -> String {
        let mut holder: String = String::new();
        for ele in &mut self.accounts {
            if ele.id == account_id {
                holder = ele.holder.to_string();
            }
        }

        holder
    }

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }
}

fn main() {
    let mut bank = Bank::new();
    let account = Account::new(1, "parham".to_string());
    let account_id_ref = &account.id;
    let cloned_account = account.clone();

    bank.add_account(cloned_account);
    bank.deposit(*account_id_ref, 1000);

    println!(
        "The owner of this account is: {} ",
        bank.get_account_owner(account.id)
    );

    println!("Your balance is: {:#?} ", bank);
}
