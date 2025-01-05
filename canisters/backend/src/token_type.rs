use candid::CandidType;
use serde::Deserialize;

use crate::updater::RuneId;

#[derive(CandidType, Deserialize, PartialEq, Eq, Hash)]
pub enum TokenType {
    Bitcoin,
    Rune(RuneId),
}
