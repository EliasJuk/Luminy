use blake3::Hasher;
use std::time::{SystemTime, UNIX_EPOCH};

// Estrutura de um bloco
#[derive(Debug, Clone)]
struct Block {
	index: u64,
	timestamp: u128,
	data: String,
	previous_hash: String,
	nonce: u64,
	hash: String,
}

// Função para calcular o hash do bloco
fn calculate_hash(
	index: u64,
	timestamp: u128,
	data: &str,
	previous_hash: &str,
	nonce: u64,
	) -> String {
		let mut hasher = Hasher::new();
		hasher.update(&index.to_le_bytes());
		hasher.update(&timestamp.to_le_bytes());
		hasher.update(data.as_bytes());
		hasher.update(previous_hash.as_bytes());
		hasher.update(&nonce.to_le_bytes());
		hasher.finalize().to_hex().to_string()
}

// Função de mineração (PoW)
fn mine_block(index: u64, data: &str, previous_hash: &str, difficulty: usize) -> Block {
	let prefix = "0".repeat(difficulty);
	let mut nonce = 0;
	let timestamp = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.unwrap()
		.as_millis();

	loop {
		let hash = calculate_hash(index, timestamp, data, previous_hash, nonce);
		if hash.starts_with(&prefix) {
			return 
				Block {
					index,
					timestamp,
					data: data.to_string(),
					previous_hash: previous_hash.to_string(),
					nonce,
					hash,
				};
		}
		nonce += 1;
		}
}

fn main() {
	//Criação do bloco gênesis
	let genesis_block = mine_block(0, "Bloco Gênesis", "0", 4);
	println!("Bloco Gênesis: {:?}\n", genesis_block);

	//Minerarando o segundo bloco
	let bloco2 = mine_block(
		1,
		"Transação: 0x1BgGZ9tcN4rm -> 0x1SdiUi4p26Lb : 50 Lumi",
		&genesis_block.hash,
		4,);
	println!("Bloco 2: {:?}", bloco2);
	println!("Hash do bloco 2: {}\n", bloco2.hash);

	//Minerarando o terceiro bloco
	let bloco3 = mine_block(
		2, 
		"Transação: 0x1SdiUi4p26Lb -> 0x19ZjTRaZMZQA :  20 Lumi",
		&bloco2.hash, 
		5);
	println!("Bloco 3: {:?}", bloco3);
	println!("Nonce do bloco 3: {}\n", bloco3.nonce);

	//Minerarando o quarto bloco
	let bloco4 = mine_block(
		3, 
		"Transação: 0x1SdiUi4p26Lb -> 0x19ZjTRaZMZQA :  20 Lumi",
		&bloco3.hash,
		6);
	println!("Bloco 4: {:?}", bloco4);
	println!("Nonce do bloco 4: {}\n", bloco4.nonce);
}