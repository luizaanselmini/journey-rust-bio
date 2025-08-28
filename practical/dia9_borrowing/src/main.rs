fn main() {
    // 1. Criamos a dona da String. A variável é 'mut' pois pretendemos modificá-la depois.
    let mut seq = String::from("GATTACAT");

    println!("Sequência original: {}", seq);

    // 2. Empréstimo Imutável (&)
    // Passamos um 'crachá de leitor' para a função 'inspecionar_sequencia'
    inspecionar_sequencia(&seq);

    // A variável 'seq' ainda é a dona e está 100% válida aqui.
    println!("Após a inspeção, a sequência continua sendo: {}", seq);

    // 3. Empréstimo Mutável (&mut)
    // Agora passamos um 'crachá de editor' para a função 'corrigir_sequencia'.
    corrigir_sequencia(&mut seq);

    // A variável 'seq' ainda é a dona, mas seu conteúdo foi alterado pela função.
    println!("Após a correção, a sequência agora é: {}", seq);

}

/// Esta função pega emprestado o acesso de LEITURA a uma String.
fn inspecionar_sequencia(s: &String) {
    println!(" -> [Inspeção] O tamanho da sequência é {}.", s.len());
    if s.starts_with("GATTA") {
        println!(" -> [Inspeção] A sequência parece válida.");
    }
    // Não podemos faazer: s.push_str("A"); ERRO! 's' é uma referência imutável.
}

/// Esta função pega emprestado o acesso de ESCRITA a uma String.
fn corrigir_sequencia(s: &mut String) {
    println!(" -> [Correção] A sequência está sendo modificada...");
    s.push_str("G"); // Adicionamos a base que faltava no final.

}
