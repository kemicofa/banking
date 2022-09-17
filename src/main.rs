mod account;

use account::Account;

fn main() {
    let account = Account::new(0);
    println!("{}", account.get_account_balance());
    println!("{}", account.get_available_balance());
}
