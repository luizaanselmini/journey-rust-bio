fn main () {
    let sequencia_dna = "AGCTTGGAACATGCGATTACAG";
    let base_alvo = 'G';

    // Chamamos a função para contar as ocorrências da base 'G'.
    let contagem = contar_ocorrencias_base(sequencia_dna, base_alvo);

    println!("Analisando a sequência: {}", sequencia_dna);
    println!("O número de ocorrências da base '{}' é: {}", base_alvo, contagem);

    println!("\n--- Exemplo com 'continue' ---");
    imprimir_base_nao_n(sequencia_dna);
    
}

/// Conta quantas vezrs uma base específica aparece em uma sequência de DNA.
/// Usa um loop 'for' para iterar sobre cada caractere.
fn contar_ocorrencias_base(sequencia:&str, base_alvo:char) -> u32 {
    // Começamos com um contador mutável zerado.
    let mut contador: u32 = 0;

    // Iteramos sobre cada 'base' na 'sequencia'.
    // O método .chars() retorna um iterador, que o 'for' consome.
    for base in sequencia.chars() {
        // Se a base atual for igual à nossa base alvo...
        if base == base_alvo {
            // ... incrementamos o contador.
            contador += 1;
        }
    }

    // Retornamos o resultado final
    contador
}

/// Imrpime toddas as bases de uma sequência, pulando as bases 'N' (desconhecida).
fn imprimir_base_nao_n(sequencia:&str) {
    println!("Imprimindo apenas bases conhecidas (não 'N') da sequência...");
    for base in sequencia.chars() {
        if base == 'N' {
            println!(" - Base 'N' encontrada, pulando para a próxima.");
            continue; // Pula o resto do código do loop e vai para a próxima iteração.
        }
        // Este println! só será executado se a base não for 'N'.
        println!(" - Processando base: {}", base);
    }
}