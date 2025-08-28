use std::fs::File;
use std::io::Write;

mod modules;
use modules::wallet::generate_wallet;
use modules::transaction::Transaction;
use modules::miner::mine_block;

fn main() {
  // Gerar 2 carteiras para teste
  let (_wallet1_key, wallet1_addr) = generate_wallet();
  let (_wallet2_key, wallet2_addr) = generate_wallet();
  println!("Wallet 1: {}", wallet1_addr);
  println!("Wallet 2: {}", wallet2_addr);

		// Criar bloco gênesis
  let genesis_tx = vec![Transaction {
    from_address: String::from("0"),
    to_address: wallet1_addr.clone(),
    amount: 100,
  }];
  let genesis_block = mine_block(0, genesis_tx, "0", 4);
  println!("Bloco Gênesis: {:?}", genesis_block);

  // Criar segundo bloco
  let block2_tx = vec![Transaction {
    from_address: wallet1_addr.clone(),
    to_address: wallet2_addr.clone(),
    amount: 50,
  }];
  let block2 = mine_block(1, block2_tx, &genesis_block.hash, 6);
  println!("Bloco 2: {:?}", block2);

  // Salvar blockchain em JSON
  let blockchain = vec![genesis_block, block2];
  let json = serde_json::to_string_pretty(&blockchain).unwrap();
  let mut file = File::create("blockchain.json").unwrap();
  file.write_all(json.as_bytes()).unwrap();
  println!("Blockchain salva em blockchain.json");
}