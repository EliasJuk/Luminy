pub fn ajustar_dificuldade(tempo_mineracao: u64, dificuldade_atual: u32) -> u32 {
  let tempo_ideal = 60000; // Tempo em milissegundos (1 minuto)

  if tempo_mineracao < tempo_ideal / 2 {
    dificuldade_atual + 1 // aumenta dificuldade
  } else if tempo_mineracao > tempo_ideal * 2 {
    dificuldade_atual.saturating_sub(1) // diminui dificuldade, sem ficar negativo
  } else {
    dificuldade_atual
  }
}