use std::fs::{File, create_dir_all};
use std::io::{Write, Read};
use crate::modules::block::{Block, BlockchainData, Status};
use std::io::ErrorKind;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn salvar_blockchain(bloco_a_salvar: &Block, caminho: &str) {
  // Garante que a pasta exista
  if let Err(e) = create_dir_all(caminho) {
    eprintln!("Erro ao criar diretório: {}", e);
    return;
  }

  let filepath = format!("{}/blockchain.json", caminho);
  let mut blockchain_data: Option<BlockchainData> = None;

  // Tentar ler o conteúdo do arquivo existente
  let mut file_content = String::new();
  match File::open(&filepath) {
    Ok(mut file) => {
      if let Err(e) = file.read_to_string(&mut file_content) {
        eprintln!("Erro ao ler o arquivo: {}", e);
        return;
      }
  
      if !file_content.is_empty() {
        // Tenta desserializar no novo formato (com cabeçalho)
        match serde_json::from_str::<BlockchainData>(&file_content) {
          Ok(data) => blockchain_data = Some(data),
      
          Err(_) => {
            // Se falhar, tenta desserializar no formato antigo (array de blocos)
            match serde_json::from_str::<Vec<Block>>(&file_content) {
              Ok(blocks) => {
                eprintln!("Arquivo JSON antigo detectado. Convertendo para o novo formato.");
                blockchain_data = Some(BlockchainData {
                  status: Status {
                    last_block: blocks.last().map(|b| b.index).unwrap_or(0),
                    last_update: SystemTime::now()
                      .duration_since(UNIX_EPOCH)
                      .expect("Time went backwards")
                      .as_millis() as u64,
                  },
                  blocks,
                });
              },
            
              Err(e) => {
                eprintln!("Erro ao desserializar JSON. Conteúdo do arquivo pode estar corrompido: {}", e);
              }
            }
          }
        }
      }
    },
    Err(e) if e.kind() == ErrorKind::NotFound => {
      // O arquivo não existe, o que é normal na primeira execução
    },
    Err(e) => {
      eprintln!("Erro ao abrir o arquivo: {}", e);
        return;
    }
  }

  // Pega o timestamp atual para a última atualização
  let last_update = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .expect("Time went backwards")
    .as_millis() as u64;

  let mut data = match blockchain_data {
    Some(mut data) => {
      // Se já existe, atualiza o status e adiciona o novo bloco
      data.status.last_block = bloco_a_salvar.index;
      data.status.last_update = last_update;
      data.blocks.push(bloco_a_salvar.clone());
      data
    },  
    None => {
      // Se não existe, cria uma nova estrutura com o primeiro bloco
      let status = Status {
        last_block: bloco_a_salvar.index,
        last_update,
      };      
      BlockchainData {
        status,
        blocks: vec![bloco_a_salvar.clone()],
      }
    }
  };

  // Serializa a estrutura completa
  let json = match serde_json::to_string_pretty(&data) {
    Ok(j) => j,
    Err(e) => {
      eprintln!("Erro ao serializar a blockchain: {}", e);
      return;
    }
  };

  // Cria e escreve no arquivo, sobrescrevendo o conteúdo antigo
  match File::create(&filepath) {
    Ok(mut file) => {
      if let Err(e) = file.write_all(json.as_bytes()) {
        eprintln!("Erro ao escrever no arquivo: {}", e);
      } else {
        println!("Blockchain salva em {}", filepath);
      }
    },
    Err(e) => {
      eprintln!("Erro ao criar o arquivo: {}", e);
    }
  }
}