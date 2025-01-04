use candid::CandidType;

#[derive(CandidType)]
pub enum SubmittedTxidType {
    Bitcoin { txid: String },
}

pub enum TransactionType {
    Etching {},
}
