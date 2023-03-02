use crate::application::features::{feature::Feature, initiate_transaction::TransactionPayload};
use crate::infrastructure::Container;
use clap::{command, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
#[command(about = "CLI to easily handle bank accounts and transactions", long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[command(arg_required_else_help = true)]
    OpenBankAccount {
        fullname: String,
    },
    ListBankAccounts,
    InitiateTransaction {
        from: String,
        to: String,
        amount: i64,
    },
}

pub fn run(container: Container) {
    let args = Cli::parse();

    match args.cmd {
        Command::OpenBankAccount { fullname } => {
            match container.open_bank_account.execute(fullname) {
                Ok(bank_account) => println!("{bank_account}"),
                Err(err) => panic!("{err}"),
            };
        }
        Command::ListBankAccounts => {
            match container.bank_account_repository.query() {
                Ok(bank_accounts) => {
                    for bank_account in bank_accounts {
                        println!("{bank_account}");
                    }
                }
                Err(err) => panic!("{err}"),
            };
        }
        Command::InitiateTransaction { to, from, amount } => {
            let transaction_payload = TransactionPayload { to, from, amount };
            match container.initiate_transaction.execute(transaction_payload) {
                Ok(transaction) => println!("{transaction}"),
                Err(err) => panic!("{err}"),
            }
        }
    };
}
