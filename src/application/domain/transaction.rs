use std::fmt;

/// A bank transaction is any money that moves in or out of your bank account.
/// Types of bank transactions include cash withdrawals or deposits, checks, online payments, debit card charges, wire transfers and loan payments.
pub struct Transaction {
    pub id: String,
    pub from: String,
    pub to: String,
    pub amount: i64,
}

impl Transaction {
    pub fn new(id: String, from: String, to: String, amount: i64) -> Self {
        Self {
            id,
            from,
            to,
            amount
        }
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "> Transaction from ({}) to ({}) of amount {:.2} €",
            self.from, self.to, (self.amount as f64)/100.0
        )
    }
}