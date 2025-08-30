use std::time::{SystemTime, UNIX_EPOCH, Instant};
use blake3::Hasher;

use super::transaction::Transaction;
use super::block::Block;

pub fn calculate_hash(
  index: u64,
  timestamp: u128,
  transactions: &Vec<Transaction>,
  previous_hash: &str,
  nonce: u64,
  ) -> String {
    let mut hasher = Hasher::new();
    hasher.update(&index.to_le_bytes());
    hasher.update(&timestamp.to_le_bytes());
    hasher.update(serde_json::to_string(transactions).unwrap().as_bytes());
    hasher.update(previous_hash.as_bytes());
    hasher.update(&nonce.to_le_bytes());
    hasher.finalize().to_hex().to_string()
  }

pub fn mine_block(
  index: u64,
  transactions: Vec<Transaction>,
  previous_hash: &str,
  difficulty: usize,
  ) -> Block {
    let prefix = "0".repeat(difficulty);
    let mut nonce = 0;

    let inicio = Instant::now();
    let timestamp = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_millis();

    loop {
      let hash = calculate_hash(index, timestamp, &transactions, previous_hash, nonce);
      if hash.starts_with(&prefix) {
        let tempo_ms = inicio.elapsed().as_millis() as u64;

        return Block {
          index,
          timestamp,
          transactions,
          previous_hash: previous_hash.to_string(),
          nonce,
          hash,
          dificuldade: difficulty as u32,
          tempo_ms,
        };
      }
    nonce += 1;
  }
}