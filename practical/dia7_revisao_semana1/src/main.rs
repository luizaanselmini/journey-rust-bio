fn main() {
    println!("--- Ferramenta de Análise de Sequência em Rust ---");

    // Dia 2: Declarando uma variável  do tipo String.
    let sequencia_dna = String::from("GATTACCA");

    println!("\nSequência de DNA análise: {}", sequencia_dna);
    println!("----------------------------------------");

    // Dia 3: Chamando nossa primeira função especialista.
    let sequencia_rna = transcrever_para_rna(&sequencia_dna);
    println!("Sequência de RNA Transcrita: {}", sequencia_rna);

    // Dia 3: Chamando nossa segunda função especialista.
    let conteudo_gc = calcular_conteudo_gc(&sequencia_dna);
    // Abaixo usamos o :.2 para formatar o float com duas casas decimais.
    println!("Conteúdo GC da sequência de DNA: {:.2}%", conteudo_gc * 100.0);

    println!("--------------------------------------");
    println!("Análise concluída.");

}

/// Função especialista 1: Transcrever uma sequência de DNA para RNA.
/// Dia 3: Recebe um parâmetro do tipo &str (empréstimo).
/// Dia 5: Retorna um novo dado do tipo String (transferência de posse).
fn transcrever_para_rna(dna: &str) -> String {
    // Dia 2 e 5: Criação de uma String mutável e vazia.
    let mut rna = String::new();

    // Dia 6: Usando um loop 'for' para iterar sobre cada caractere.
    for base in dna.chars() {
        // Dia 4: Usando 'match' (uma forma de controle de fluxo) para tomar decisões.
        let base_rna = match base {
            'A' => 'U',
            'T' => 'A',
            'C' => 'G',
            'G' => 'C',
            _ => 'N', // Caractere desconhecido
        };
        // Dia 5: Adicionado o caractere resultante à nossa String de RNA.
        rna.push(base_rna);
    }
    rna // Retorno da String recém-criada.
} 

/// Função especialista 2: Calcula a proporção de 'G' e 'C' em uma sequência;
/// Dia 3: Recebe um &str e retorna um f64.
fn calcular_conteudo_gc(dna: &str) -> f64 {
    let mut gc_count = 0.0; // Usamos f64 para evitar conversões depois.
    let mut total_count = 0.0;

    // Dia 6: Loop para analisar a sequência.
    for base in dna.chars() {
        // Dia 4: Controle de fluxo com 'if/else if'.
        if base == 'G' || base == 'C' {
            gc_count += 1.0;
            total_count += 1.0;
            } else if base == 'A' || base == 'T' {
                total_count += 1.0;
            }
    }

    // Evita divisão por zero se a sequência estiver vazia ou for inválida;
    if total_count == 0.0 {
        return 0.0;

    }

    // Retorna a proporção.
    gc_count / total_count
}