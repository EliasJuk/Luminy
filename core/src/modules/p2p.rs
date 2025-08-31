use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write};
use serde::{Serialize, Deserialize};
use serde_json;
use crate::modules::block::Block;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
	pub msg_type: String,
	pub payload: String,
}

pub fn handle_peer(mut stream: TcpStream, blockchain: &mut Vec<Block>) {
	let mut buffer = [0; 4096];
	loop {
		match stream.read(&mut buffer) {
			Ok(size) if size > 0 => {
				if let Ok(msg) = serde_json::from_slice::<Message>(&buffer[..size]) {
					println!("Mensagem recebida: {:?}", msg);
					if msg.msg_type == "NEW_BLOCK" {
						if let Ok(block) = serde_json::from_str::<Block>(&msg.payload) {
							blockchain.push(block);
							println!("Bloco adicionado à blockchain local!");
						}
					}
				}
			}
			_ => break,
		}
	}
}

pub fn start_node(address: &str, blockchain: &mut Vec<Block>) {
	let listener = TcpListener::bind(address).expect("Erro ao bindar a porta");
	println!("Nó rodando em {}...", address);

	for stream in listener.incoming() {
		match stream {
			Ok(stream) => {
				let mut bc = blockchain.clone();
				thread::spawn(move || {
					handle_peer(stream, &mut bc);
				});
			}
				Err(e) => println!("Erro: {}", e),
		}
	}
}

// Função para enviar mensagem JSON para outro nó
pub fn send_message(peer_addr: &str, msg: &Message) {
	if let Ok(mut stream) = TcpStream::connect(peer_addr) {
		let data = serde_json::to_vec(msg).unwrap();
		let _ = stream.write_all(&data);
	}
}