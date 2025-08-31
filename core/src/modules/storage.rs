use std::fs::{File, create_dir_all};
use std::io::Write;
use crate::modules::block::Block;

pub fn salvar_blockchain(blockchain: &Vec<Block>, caminho: &str) {
  // Garante que a pasta exista
  create_dir_all(caminho).unwrap();

  // Serializa a blockchain
  let json = serde_json::to_string_pretty(&blockchain).unwrap();

  // Cria e escreve no arquivo
  let mut file = File::create(format!("{}/blockchain.json", caminho)).unwrap();
  file.write_all(json.as_bytes()).unwrap();

  println!("Blockchain salva em {}/blockchain.json", caminho);
}