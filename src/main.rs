mod account;
mod bank;

use crate::account::Account;
use crate::bank::Bank;

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

    println!(
        "Your balance is: {:#?}$ ",
        bank.get_account_balance(*account_id_ref)
    );
}
