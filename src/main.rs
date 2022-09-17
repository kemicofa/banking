mod account;

use account::Account;

fn main() {
    let account = Account::new(0);
    println!("{}", account.get_balance());
}
