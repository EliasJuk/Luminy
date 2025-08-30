use std::fs::{File, create_dir_all};
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

  //=====================================================
	// Criar bloco gênesis
  let genesis_tx = vec![Transaction {
    from_address: String::from("0"),
    to_address: wallet1_addr.clone(),
    amount: 100,
  }];

  // Minerar o Bloco Gênises
  let genesis_block = mine_block(0, genesis_tx, "0", 4);
  println!("\nBloco Gênesis: {:?}", genesis_block);

  //=====================================================
  // Criar segundo bloco
  let block2_tx = vec![Transaction {
    from_address: wallet1_addr.clone(),
    to_address: wallet2_addr.clone(),
    amount: 50,
  }];

  // Minerar o Bloco 2
  let block2 = mine_block(1, block2_tx, &genesis_block.hash, 3);
  println!("\nBloco 2: {:?}", block2);

  //=====================================================
  // Criar terceiro bloco
  let block3_tx = vec![
    Transaction {
      from_address: wallet1_addr.clone(),
      to_address: wallet2_addr.clone(),
      amount: 25,
    },
    Transaction {
      from_address: wallet2_addr.clone(),
      to_address: wallet1_addr.clone(),
      amount: 10,
    },
  ];

  // Minerar o bloco 3
  let block3 = mine_block(2, block3_tx, &block2.hash, 4);
  println!("\nBloco 3: {:?}", block3);


  //=====================================================
  // Garante que a pasta 'data' exista
  create_dir_all("./data").unwrap();

  // Salvar blockchain em JSON
  let blockchain = vec![genesis_block, block2, block3];
  let json = serde_json::to_string_pretty(&blockchain).unwrap();
  let mut file = File::create("data/blockchain.json").unwrap();
  file.write_all(json.as_bytes()).unwrap();
  println!("Blockchain salva em blockchain.json");
}