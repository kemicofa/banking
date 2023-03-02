use crate::application::features::feature::Feature;
use crate::infrastructure::Container;
use clap::{Parser, command, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
#[command(about = "CLI to easily handle bank accounts and transactions", long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: Command
}

#[derive(Debug, Subcommand)]
enum Command {
    #[command(arg_required_else_help = true)]
    OpenBankAccount {
        fullname: String
    }
}

pub fn run(container: Container) {
    let args = Cli::parse();

    match args.cmd {
        Command::OpenBankAccount { fullname } => {
            match container.open_bank_account.execute(Some(fullname)) {
                Ok(bank_account) => println!("{bank_account}"),
                Err(err) => panic!("{err}"),
            }
        }
    }

    println!("No arguments passed");
}
