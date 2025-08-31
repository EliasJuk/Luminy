use std::time::Instant;
use std::thread;

use crate::modules::wallet::generate_wallet;
use crate::modules::transaction::Transaction;
use crate::modules::miner::{mine_block, validate_block};
use crate::modules::difficulty::ajustar_dificuldade;
use crate::modules::p2p::{Message, start_node, send_message};
use crate::modules::block::Block;
use crate::modules::storage::salvar_blockchain;

pub fn blockchain() {
  // Gerar 2 carteiras para teste
  let (_wallet1_key, wallet1_addr) = generate_wallet();
  let (_wallet2_key, wallet2_addr) = generate_wallet();
  println!("Wallet 1: {}", wallet1_addr);
  println!("Wallet 2: {}", wallet2_addr);

  let mut blockchain: Vec<Block> = Vec::new();

  //==================== Iniciar nó P2P ====================
  let mut bc_clone = blockchain.clone();
  thread::spawn(move || {
    start_node("127.0.0.1:4000", &mut bc_clone);
  });

  //==================== Bloco Gênesis ====================
  let mut dificuldade: u32 = 2;
  let inicio_genesis = Instant::now();

  let genesis_tx = vec![Transaction {
    from_address: String::from("0"),
    to_address: wallet1_addr.clone(),
    amount: 100,
  }];

  // Minerar o Bloco Gênises
  let genesis_block = mine_block(0, genesis_tx, "0", dificuldade as usize);  
  let duracao_genesis = inicio_genesis.elapsed().as_millis() as u64;
  
  dificuldade = ajustar_dificuldade(duracao_genesis, dificuldade);
  println!("\nBloco Gênesis: {:?}", genesis_block);
  println!("Duração: {} ms | Nova dificuldade: {}", duracao_genesis, dificuldade);

  //==================== Bloco 2 ====================
  let inicio_block2 = Instant::now();
  print!("\n Minerando bloco 2");

  let block2_tx = vec![Transaction {
    from_address: wallet1_addr.clone(),
    to_address: wallet2_addr.clone(),
    amount: 50,
  }];

  // Minerar o Bloco 2
  let block2 = mine_block(1, block2_tx, &genesis_block.hash, dificuldade as usize);

  let duracao_block2 = inicio_block2.elapsed().as_millis() as u64;
  dificuldade = ajustar_dificuldade(duracao_block2, dificuldade);
  println!("\nBloco 2: {:?}", block2);
  println!("Duração: {} ms | Nova dificuldade: {}", duracao_block2, dificuldade);

  //==================== VALIDAR BLOCO 2 ====================
  if validate_block(&block2, &genesis_block) {
    println!("Bloco 2 válido!");
    blockchain.push(block2.clone());

    let msg = Message {
      msg_type: "NEW_BLOCK".to_string(),
      payload: serde_json::to_string(&block2).unwrap(),
    };

    // Enviar para outro nó (ex: 127.0.0.1:4001)
    send_message("127.0.0.1:4001", &msg);
    } else {
      println!("Bloco 2 inválido!");
    }

  //==================== Bloco 3 ====================
  let inicio_block3 = Instant::now();
  print!("\n Minerando bloco 3");

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
  let block3 = mine_block(2, block3_tx, &block2.hash, dificuldade as usize);

  let duracao_block3 = inicio_block3.elapsed().as_millis() as u64;
  dificuldade = ajustar_dificuldade(duracao_block3, dificuldade);
  println!("\nBloco 3: {:?}", block3);
  println!("Duração: {} ms | Nova dificuldade: {}", duracao_block3, dificuldade);


  //==================== Salvar Blockchain ====================
  let blockchain = vec![genesis_block, block2, block3];
  salvar_blockchain(&blockchain, "./data");
}