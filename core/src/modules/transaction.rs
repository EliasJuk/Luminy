use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
  pub from_address: String,
  pub to_address: String,
  pub amount: u64,
}