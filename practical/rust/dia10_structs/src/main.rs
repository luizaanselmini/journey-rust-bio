
// 1. Definimos o molde para nossos dados.
// Queremos agrupar um ID e uma sequência de DNA.
struct RegistroFasta {
    id: String,
    sequencia: String,
}

fn main() {
    // 2. Criamos uma instância da nossa struct.
    let registro1 = RegistroFasta {
        id: String::from("gene_COX1_humano"),
        sequencia: String::from("GATTACATGGTT"),
    };

    // Criamos outra instância para um organismo diferente.
    let registro2 = RegistroFasta {
        id: String::from("gene_COX1_camundongo"),
        sequencia: String::from("GCTTACATGGCT"),
    };

    println!("--- Análise de Registro FASTA ---");

    // 3. Chamamos uma função para analisar nosso primeiro registro.
    // Passamos uma referência (&) para não mover a posse!
    imprimir_resumo_registro(&registro1);
    imprimir_resumo_registro(&registro2);

    // registro1 e registro2 continuam válidos aqui porque só emprestamos o acesso.
    println!("\nAnálise concluída. Ambos os registros ainda são de nossa posse.");

}

/// Uma função que recebe um EMPRÉSTIMO de um RegistroFasta e imprime um resumo.
/// O tipo do parâmetro é &RegistroFasta - uma referência à nossa struct.
fn imprimir_resumo_registro(registro: &RegistroFasta) {
    println!("\nAnalisando o registro com ID: {}", registro.id); // Acessamos os campos com '.'
    println!(" - Comprimento da sequência: {} pb", registro.sequencia.len());

    if registro.sequencia.starts_with("GATTA") {
        println!(" - A sequência começa com o motivo esperado.");
    }   else {
        println!(" - A sequência NÃO  começa com o motivo esperado.")
    }
    
}
