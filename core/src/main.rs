mod utils;
mod modules;

use std::env;
use crate::modules::blockchain;
use crate::utils::config::Config;

const VERSION: &str = "1.0.0";
const PROGRAM: &str = "core.exe";
const AWARD: u32 = 20;
const HALVING: u32 = 20000;


fn minerar() {
    println!("Função minerar executada!");
}

fn ports(config: &Config){
    println!("LOCAL -> IP: {}, PORT: {}", config.local_ip, config.local_port);
    println!("LISTEN -> IP: {}, PORT: {}", config.listen_ip, config.listen_port);
    println!("BRIDGE -> IP: {}, PORT: {}", config.bridge_ip, config.bridge_port);
}

fn commands(){
  println!("Uso: {} <comando>", PROGRAM);
  println!("Comandos disponíveis: minerar, listen, version, ports");
}

fn init_blochain(){
  blockchain::blockchain();
}

fn listen() {
  println!("Hello");
}

fn main() {
  let config = Config::load();
  let args: Vec<String> = env::args().collect();

  //let comando = args.get(1).map(|s| s.as_str()).unwrap_or("cmd");
  let comando = "blockchain";

  match comando {
    "cmd" => commands(),
    "commands" => commands(),
    "ports" => ports(&config),
    "minerar" => minerar(),
    "listen" => listen(),
    "blockchain" => init_blochain(),
    "version" => println!("Versão do programa: {}", VERSION),
    _ => println!("Comando desconhecido: {}", comando),
  }
}