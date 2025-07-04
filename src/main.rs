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

    fn deposit(&self, account: &mut Account, balance: i32) {
        account.balance = balance;
    }
}

fn add_account(bank: &mut Bank, account: Account) {
    bank.accounts.push(account);
}

fn main() {
    let mut bank_a = Bank::new();
    let mut account_a = Account::new(1, "parham".to_string());

    bank_a.deposit(&mut account_a, 1000);
    add_account(&mut bank_a, account_a);

    println!("{:#?}", bank_a);
}
