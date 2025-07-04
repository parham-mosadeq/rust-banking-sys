#[derive(Debug)]
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
        // let acc = &;
        for ele in &mut self.accounts {
            if ele.id == account_id {
                ele.balance = balance;
            }
        }
    }
}

fn add_account(bank: &mut Bank, account: Account) {
    bank.accounts.push(account);
}

fn main() {
    let mut bank_a = Bank::new();
    let account_a = Account::new(1, "parham".to_string());
    let account_a_id_ref = &account_a.id;

    bank_a.deposit(*account_a_id_ref, 1000);
    add_account(&mut bank_a, account_a);

    println!("{:#?}", bank_a);
}
