use clap::Parser;
use banking::secondary_adapters::in_memory_adapter::InMemoryAdapter;
use banking::application::features::open_bank_account::open_bank_account;

/// Command Line Interface for bank account
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Open bank account
    #[clap(long, short, action)]
    open_bank_account: bool,
}

pub fn main() {
    let args = Args::parse();

    if args.open_bank_account {
        let account = open_bank_account(Box::new(InMemoryAdapter::new()));
        println!("Available funds {}", account.get_account_balance());
        return;
    }

    println!("Should open bank account {}!", args.open_bank_account)
}
