// src/main.rs

// Enum para representar os aminoácidos que nossa função reconhece.
#[derive(Debug)] // Permite que a gente imprima com {:?}
enum Aminoacido {
    Alanina, // 'A'
    Glicina, // 'G'
}

/// Tentar converter uma string em um vetor de Aminoácidos.
/// Retorna Ok(Vetor) em caso de sucesso.
/// Retorna Err(&str) em caso de falha.
fn parse_sequencia_proteica(seq: &str) -> Result<Vec<Aminoacido>, &'static str> {
    let mut proteina = Vec::new(); // Vec é um vetor, uma lista que veremos em breve.

    for base in seq.chars() {
        match base {
            'A' => proteina.push(Aminoacido::Alanina),
            'G' => proteina.push(Aminoacido::Glicina),
            // Se encontrarmos um caracer que não conhecemos, a operação falha!
            _ => return Err("Caractere inválido encontrado na sequência!"),
        }
    }

    // Se o loop terminar sem erros, a operação foi um sucesso!
    Ok(proteina)
}

fn main () {
    let seq_valida = "AGGA";
    let seq_invalida = "AGXAG"; // Contém um 'X' inválido.

    println!("--- Testando com sequência válida: {} ---", seq_valida);
    let resultado1 = parse_sequencia_proteica(seq_valida);

    // Usamos 'match' para lidar com o Result retornado.
    match resultado1 {
        Ok(proteina) => println!("Sucesso! Sequência parseada: {:?}", proteina),
        Err(erro) => println!("Erro: {}", erro),
    }

    println!("\n--- Testando com sequência inválida: {} ---", seq_invalida);
    let resultado2 = parse_sequencia_proteica(seq_invalida);

    match resultado2 {
        Ok(proteina) => println!("Sucesso! Sequência parseada: {:?}", proteina),
        Err(erro) => println!("Erro: {}", erro),
    }
}
