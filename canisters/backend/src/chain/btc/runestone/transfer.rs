use crate::txn_handler::TransactionType;

pub struct RuneTranferArgs {
    runeid: u128,
}

pub fn transfer() -> Result<TransactionType, (u128, u64)> {
    todo!()
}
