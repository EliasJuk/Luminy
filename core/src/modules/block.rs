use serde::{Deserialize, Serialize};
use super::transaction::Transaction;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
  pub index: u64,
  pub timestamp: u128,
  pub transactions: Vec<Transaction>,
  pub previous_hash: String,
  pub nonce: u64,
  pub hash: String,
  pub dificuldade: u32,
  pub tempo_ms: u64, 
}