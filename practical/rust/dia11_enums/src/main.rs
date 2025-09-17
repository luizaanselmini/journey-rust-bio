//src/main.rs

// 1. Definimos nossos tipos customizados com enums.

// Um aminoácido pode ser um de vários tipos.
#[derive(Debug)] // O 'derive' nos permite imprimir o enum com {:?}
enum Aminoacido {
    Alanina,
    Glicina,
    Leucina,
    Serina,
    // ... poderíamos adicionar todos os 20
}

// o sentido da fita de DNA.
#[derive(Debug)]
enum SentidoFita {
    Forward, // 5'-> 3'
    Reverse, // 3'-> 5'
}

// 2. Usamos nosso enum em uma struct.
#[derive(Debug)]
struct Proteina {
    nome: String,
    sentido: SentidoFita,
    comprimento: u32,
}
fn main() {
    // 3. Criamos instâncias usando nossos enums.
    let proteina_a = Proteina {
        nome: String::from("Albumina"),
        sentido: SentidoFita::Forward,
        comprimento: 585,

    };

    let aminoacido_lido = Aminoacido::Leucina;

    println!("--- Análise de Proteínas e Aminoácidos ---");
    println!("Proteína encontrada: {:?}", proteina_a);
    println!("Aminácido lido da sequência: {:?}", aminoacido_lido);

    // 4. Usamos 'match' para  executar código com base na variante do enum.
    analisar_aminoacido(aminoacido_lido);
    analisar_aminoacido(Aminoacido::Glicina);

}

/// Uma função que usa 'match' para tomar uma decisãi com base em um enum.
fn analisar_aminoacido(aa: Aminoacido) {
    println!("Analisando {:?} ", aa);
    match aa {
        Aminoacido::Alanina => println!("É um aminácido apolar."),
        Aminoacido::Glicina => println!("É o aminácido mais simples"),
        Aminoacido::Leucina => println!("É um aminácido essencial."),
        Aminoacido::Serina => println!("Possui um grupo hidroxila."),
    }
}