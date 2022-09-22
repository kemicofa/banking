/// An overdraft occurs when there isn't enough money in an account to cover a transaction or withdrawal,
/// but the bank allows the transaction anyway. Essentially, it's an extension of credit from the financial institution that is granted when an account reaches zero. 
/// The overdraft allows the account holder to continue withdrawing money even when the account has no funds in it or has insufficient funds to cover the amount of the withdrawal.
pub struct Overdraft {
    interest: i64,
    amount: i64
}
