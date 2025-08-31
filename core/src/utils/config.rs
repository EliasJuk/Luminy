use configparser::ini::Ini;
use std::path::Path;

pub struct Config {
    pub local_ip: String,
    pub local_port: u16,
    pub bridge_ip: String,
    pub bridge_port: u16,
}

impl Config {
  pub fn load() -> Config {
    let path = Path::new("conf.ini");
    let mut config = Ini::new();

    // Se não existir, cria com valores padrão
    if !path.exists() {
      config.set("LOCAL", "IP", Some("127.0.0.1".to_string()));
      config.set("LOCAL", "PORT", Some("4389".to_string()));
      config.set("BRIDGE", "IP", Some("127.0.0.1".to_string()));
      config.set("BRIDGE", "PORT", Some("4390".to_string()));
      config
        .write(path.to_str().unwrap()).expect("Erro ao criar conf.ini");
      println!("Arquivo conf.ini criado com valores padrão.");
    }

    // Carrega o conf.ini
    config.load(path.to_str().unwrap()).expect("Erro ao ler conf.ini");

    Config {
			local_ip: config.get("LOCAL", "IP").unwrap(),
      local_port: config.get("LOCAL", "PORT").unwrap().parse().unwrap(),
      bridge_ip: config.get("BRIDGE", "IP").unwrap(),
      bridge_port: config.get("BRIDGE", "PORT").unwrap().parse().unwrap(),
		}
  }
}