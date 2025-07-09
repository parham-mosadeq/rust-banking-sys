mod account;
mod bank;

use crate::account::Account;
use crate::bank::Bank;

fn main() {
    let mut bank = Bank::new();
    let account = Account::new(1, "parham".to_string());
    let account_id_ref = &account.id;
    let cloned_account = account.clone();
    let account_1 = Account::new(2, "user test".to_string());
    let account_id_ref_1 = &account.id;
    let cloned_account_1 = account.clone();

    bank.add_account(cloned_account);
    bank.deposit(*account_id_ref, 1000);
    bank.deposit(*account_id_ref, 2400);
    bank.withdraw(*account_id_ref, 200);
    bank.withdraw(*account_id_ref, 1000);

    bank.add_account(cloned_account_1);
    bank.deposit(*account_id_ref_1, 5000);
    bank.deposit(*account_id_ref_1, 12400);
    bank.withdraw(*account_id_ref_1, 1200);
    bank.withdraw(*account_id_ref_1, 100);

    println!(
        "The owner of this account is: {} ",
        bank.get_account_owner(account.id)
    );
    println!(
        "Your balance is: {:#?}$ ",
        bank.get_account_balance(*account_id_ref)
    );

    println!(
        "The owner of this account is: {} ",
        bank.get_account_owner(account_1.id)
    );
    println!(
        "Your balance is: {:#?}$ ",
        bank.get_account_balance(*account_id_ref_1)
    );

    println!("Your balance summary is: {:#?}$ ", bank.get_summary());
}
