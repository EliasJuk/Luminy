use std::time::Instant;
use std::thread;
use std::sync::{Arc, Mutex};
use std::collections::HashSet;

use crate::modules::wallet::generate_wallet;
use crate::modules::transaction::Transaction;
use crate::modules::miner::{mine_block, validate_block};
use crate::modules::difficulty::ajustar_dificuldade;
use crate::modules::block::Block;
use crate::modules::message::Message;
use crate::modules::storage::salvar_blockchain;
use crate::utils::config::Config;

pub fn blockchain() {
  // Gerar 2 carteiras para teste
  let (_wallet1_key, wallet1_addr) = generate_wallet();
  let (_wallet2_key, wallet2_addr) = generate_wallet();
  println!("Wallet 1: {}", wallet1_addr);
  println!("Wallet 2: {}", wallet2_addr);

  let wallet_miner = "1ffsfffsfsffsdfsf";


  //==================== Bloco Gênesis ====================
  //======================= Bloco 0 =======================
  
  let mut index: u64 = 0;
  let mut dificuldade: u32 = 2;
  let inicio_genesis = Instant::now();

  let genesis_tx = vec![Transaction {
    from_address: String::from("0"),
    to_address: wallet1_addr.clone(),
    amount: 100,
  }];

    // Minerar o Bloco Gênesis
    let genesis_block = mine_block(index, genesis_tx, "0", dificuldade as usize);
    let duracao_genesis = inicio_genesis.elapsed().as_millis() as u64;
    dificuldade = ajustar_dificuldade(duracao_genesis, dificuldade);
    println!("\nNovo Bloco Gênesis encontrado!");
    println!("Duração: {} ms | Nova dificuldade: {}", duracao_genesis, dificuldade);

    // Salvar o Bloco Gênesis com o novo formato de cabeçalho
    salvar_blockchain(&genesis_block, "./data");

    //==================== Bloco 1 ====================
    index = 1;
    let inicio_block1 = Instant::now();
    println!("Minerando bloco 1");

    let block1_tx = vec![Transaction {
        from_address: wallet1_addr.clone(),
        to_address: wallet2_addr.clone(),
        amount: 50,
    }];

    // Minerar o Bloco 1
    let block1 = mine_block(index, block1_tx, &genesis_block.hash, dificuldade as usize);
    let duracao_block1 = inicio_block1.elapsed().as_millis() as u64;
    dificuldade = ajustar_dificuldade(duracao_block1, dificuldade);
    println!("\nNovo Bloco 1 encontrado!");
    println!("Duração: {} ms | Nova dificuldade: {}", duracao_block1, dificuldade);
    
    // Validar e Salvar o Bloco 1
    if validate_block(&block1, &genesis_block) {
        println!("Bloco 1 válido!");
        salvar_blockchain(&block1, "./data");
    } else {
        println!("Bloco 1 inválido!");
    }

    //==================== Bloco 2 ====================
    index = 2;
    let inicio_block2 = Instant::now();
    println!("Minerando bloco 2");

    let block2_tx = vec![
        Transaction {
            from_address: wallet1_addr.clone(),
            to_address: wallet2_addr.clone(),
            amount: 25,
        },
        Transaction {
            from_address: wallet2_addr.clone(),
            to_address: wallet1_addr.clone(),
            amount: 10,
      }
    ];

    // Minerar o bloco 2
    let mut current_block = 2;
    let block2 = mine_block(index, block2_tx, &block1.hash, dificuldade as usize);

    let duracao_block2 = inicio_block2.elapsed().as_millis() as u64;
    dificuldade = ajustar_dificuldade(duracao_block2, dificuldade);
    println!("\nNovo Bloco 2 encontrado!");
    println!("Duração: {} ms | Nova dificuldade: {}", duracao_block2, dificuldade);

    // Validar e Salvar o Bloco 2
    if validate_block(&block2, &block1) {
      println!("Bloco 2 válido!");
      salvar_blockchain(&block2, "./data");
    } else {
      println!("Bloco 2 inválido!");
    }

  let mut last_block_hash = block2.hash;


  // NOVO BLOCOS
  let mut new_block = || {
    index += 1;
    current_block += 1;
    let inicio_new_block = Instant::now();

    //ADD coinbase_transaction
    let block_new_tx = vec![
      Transaction {
        from_address: wallet1_addr.clone(),
        to_address: wallet2_addr.clone(),
        amount: 25,
      },
      Transaction {
        from_address: wallet2_addr.clone(),
        to_address: wallet1_addr.clone(),
        amount: 10,
      }
    ];

    println!("\nIndex: {}, Novo bloco: {}", index, current_block);
    let novo_bloco = mine_block(index, block_new_tx, &last_block_hash, dificuldade as usize);
    let duracao_new_block = inicio_new_block.elapsed().as_millis() as u64;
    println!("Novo Bloco {} encontrado!", current_block);
    println!("HASH: {}, NONCE: {}", &novo_bloco.hash, &novo_bloco.nonce);
    println!("Duração: {} ms | Nova dificuldade: {}", duracao_new_block, dificuldade);
    
    salvar_blockchain(&novo_bloco, "./data");
    last_block_hash = novo_bloco.hash;
  };
  
  loop {
    new_block();
  }

  
}