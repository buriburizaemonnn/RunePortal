use candid::{CandidType, Principal};
use ic_ledger_types::AccountIdentifier;
use icrc_ledger_types::icrc1::account::Account;
use tiny_keccak::{Hasher, Sha3};

#[derive(CandidType)]
pub struct Addresses {
    pub icrc1: Account,
    pub icrc1_string: String,
    pub account_identifier: AccountIdentifier,
    pub account_identifier_string: String,
    pub bitcoin: String,
}

impl From<[u8; 32]> for Addresses {
    fn from(subaccount: [u8; 32]) -> Self {
        let id = ic_cdk::id();
        let account_identifier =
            AccountIdentifier::new(&id, &ic_ledger_types::Subaccount(subaccount));
        let account = Account {
            owner: id,
            subaccount: Some(subaccount),
        };
        let bitcoin = "".to_string(); // account_to_p2pkh_address(&account);
        Addresses {
            icrc1: account,
            icrc1_string: account.to_string(),
            account_identifier,
            account_identifier_string: account_identifier.to_string(),
            bitcoin,
        }
    }
}

impl From<&Principal> for Addresses {
    fn from(principal: &Principal) -> Self {
        let mut hash = [0u8; 32];
        let mut hasher = Sha3::v256();
        hasher.update(principal.as_slice());
        hasher.finalize(&mut hash);
        Self::from(hash)
    }
}
