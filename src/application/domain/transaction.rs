/// A bank transaction is any money that moves in or out of your bank account.
/// Types of bank transactions include cash withdrawals or deposits, checks, online payments, debit card charges, wire transfers and loan payments.
pub struct Transaction {
    from: i64,
    to: i64,
    amount: i64,
}
