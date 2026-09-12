use serde::{Deserialize, Serialize};

pub mod encoding;

pub type Hash = [u8; 32];
pub type Address = [u8; 20];
pub type Amount = u128;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Account {
    pub balance: Amount,
    pub nonce: u64,
}
