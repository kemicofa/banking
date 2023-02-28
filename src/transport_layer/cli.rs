use clap::Parser;
use crate::application::features::open_bank_account::open_bank_account;
use crate::data_sources::in_memory_adapter::InMemoryAdapter;

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
        let accountDTO = open_bank_account(Box::new(InMemoryAdapter::new()));
        println!("{}", accountDTO.to_string());
        return;
    }

    println!("No arguments passed");
}
