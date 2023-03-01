use crate::application::features::feature::Feature;
use crate::infrastructure::Container;
use clap::Parser;

/// Command Line Interface for bank account
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Open bank account
    #[clap(long, short, action)]
    open_bank_account: bool,
}

pub fn run(container: Container) {
    let args = Args::parse();

    if args.open_bank_account {
        match container.open_bank_account.execute(None) {
            Ok(bank_account) => println!("{}", bank_account),
            Err(err) => panic!("{}", err),
        }
        return;
    }

    println!("No arguments passed");
}
