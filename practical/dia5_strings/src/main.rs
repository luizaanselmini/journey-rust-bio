// src/main.rs
fn main() {
    // 1. Criar uma String a partir de um &str
    let mut sequencia_dna = String::from("ATGCGGTCAT");

    println!("--- Análise Inicial da Sequência ---");
    println!("Sequência original: {}", sequencia_dna);
    println!("Tamanho inicial: {} bases", sequencia_dna.len());

    // 2. Modificar a String, adicionando uma região desconhecida
    let regiao_desconhecida = "NNNNN";
    sequencia_dna.push_str(regiao_desconhecida);

    println!("\n--- Sequência Modificada ---");
    println!("Sequência após adição: {}", sequencia_dna);
    println!("Tamanho final: {} bases", sequencia_dna.len());

    // 3. Construir um identificador de sequência usando format!
    let nome_gene = String::from("COX1");
    let organismo = "Homo sapiens";
    let id_acesso = 12345;

    let identificador_completo = format!(
        ">{}:{} | Acesso #{}",
        nome_gene,
        organismo,
        id_acesso,
    );

    println!("\n--- Identificador no Formato FASTA ---");
    println!("{}", identificador_completo);

    // 4. Iterar sobre a sequência original usando .chars()
    println!("\n--- Iterando sobre as bases da sequência '{}' ---", nome_gene);
    // Note que usamos a String 'nome_gene' aqui sem problemas, pois format! não a consumiu.
    for caractere in nome_gene.chars(){
        println!("Caractere: {}", caractere);
    }


}
